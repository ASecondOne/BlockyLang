const vscode = require('vscode');

const BUILTINS = [
    ['if', 'Run the following closure when a boolean is true.'],
    ['let', 'Declare a variable inside a <define> block.'],
    ['new', 'Create an unset String, Number, or Bool value.'],
    ['type', 'Return the kind of a value or variable.'],
    ['origin', 'Return the declaration origin of a variable.'],
    ['transfer', 'Copy a closure-local variable to its immediate parent scope.'],
    ['inc_one', 'Return a number increased by one.'],
    ['print', 'Print a value without a trailing newline.'],
    ['println', 'Print a value followed by a newline.'],
    ['get_AcMods', 'Read a variable’s access modifiers.'],
    ['set_AcMods', 'Set access modifiers, currently including "mutabl".'],
    ['mut', 'Add the Mutabl access modifier to a variable.']
];

const KEYWORD_HELP = new Map(BUILTINS);
const BLOCK_TYPES = new Set(['define', 'execute']);
const diagnosticsByUri = new Map();
let diagnosticCollection;
let errorDecoration;
let warningDecoration;

function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('blocky');
    errorDecoration = vscode.window.createTextEditorDecorationType({
        after: { color: new vscode.ThemeColor('errorForeground'), margin: '0 0 0 2em' }
    });
    warningDecoration = vscode.window.createTextEditorDecorationType({
        after: { color: new vscode.ThemeColor('editorWarning.foreground'), margin: '0 0 0 2em' }
    });

    const validate = document => {
        if (document.languageId !== 'blocky') return;
        const diagnostics = validateDocument(document);
        diagnosticCollection.set(document.uri, diagnostics);
        diagnosticsByUri.set(document.uri.toString(), diagnostics);
        updateVisibleEditors();
        return diagnostics;
    };

    context.subscriptions.push(
        diagnosticCollection,
        errorDecoration,
        warningDecoration,
        vscode.workspace.onDidOpenTextDocument(validate),
        vscode.workspace.onDidChangeTextDocument(event => validate(event.document)),
        vscode.workspace.onDidSaveTextDocument(validate),
        vscode.window.onDidChangeActiveTextEditor(updateVisibleEditors),
        vscode.workspace.onDidChangeConfiguration(event => {
            if (event.affectsConfiguration('blocky.inlineDiagnostics')) updateVisibleEditors();
        }),
        vscode.commands.registerCommand('blocky.validateDocument', () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'blocky') return;
            const diagnostics = validate(editor.document) || [];
            const errors = diagnostics.filter(item => item.severity === vscode.DiagnosticSeverity.Error).length;
            const warnings = diagnostics.length - errors;
            vscode.window.setStatusBarMessage(`Blocky: ${errors} error(s), ${warnings} warning(s)`, 3000);
        }),
        vscode.languages.registerCompletionItemProvider('blocky', {
            provideCompletionItems() {
                const items = BUILTINS.map(([name, description]) => {
                    const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Keyword);
                    item.detail = 'Blocky built-in';
                    item.documentation = new vscode.MarkdownString(description);
                    return item;
                });
                for (const block of BLOCK_TYPES) {
                    const item = new vscode.CompletionItem(`<${block}>`, vscode.CompletionItemKind.Module);
                    item.detail = 'Blocky block';
                    items.push(item);
                }
                for (const type of ['String', 'Number', 'Bool', 'Boolean']) {
                    const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.TypeParameter);
                    item.detail = 'Blocky variable type';
                    items.push(item);
                }
                return items;
            }
        }, '.'),
        vscode.languages.registerHoverProvider('blocky', {
            provideHover(document, position) {
                const word = document.getWordRangeAtPosition(position, /[A-Za-z_][A-Za-z0-9_]*/);
                if (!word) return undefined;
                const name = document.getText(word);
                const description = KEYWORD_HELP.get(name);
                if (!description) return undefined;
                return new vscode.Hover(new vscode.MarkdownString(`**Blocky \`${name}\`**\n\n${description}`), word);
            }
        }),
        vscode.languages.registerInlayHintsProvider('blocky', {
            provideInlayHints(document, range) {
                if (!vscode.workspace.getConfiguration('blocky').get('inlayHints.types', true)) return [];
                return provideTypeHints(document, range);
            }
        })
    );

    for (const document of vscode.workspace.textDocuments) validate(document);
    updateVisibleEditors();
}

function deactivate() {
    diagnosticsByUri.clear();
}

function validateDocument(document) {
    const diagnostics = [];
    const lines = document.getText().split(/\r?\n/);
    let openBlock = null;
    const delimiters = [];
    let inString = false;

    const report = (line, start, end, message, severity = vscode.DiagnosticSeverity.Error) => {
        const safeLine = Math.min(Math.max(line, 0), Math.max(lines.length - 1, 0));
        const text = lines[safeLine] || '';
        const from = Math.min(Math.max(start, 0), text.length);
        const to = Math.min(Math.max(end, from), text.length);
        const diagnostic = new vscode.Diagnostic(
            new vscode.Range(safeLine, from, safeLine, Math.max(to, Math.min(from + 1, text.length))),
            message,
            severity
        );
        diagnostic.source = 'Blocky';
        diagnostics.push(diagnostic);
    };

    for (let lineNumber = 0; lineNumber < lines.length; lineNumber++) {
        const line = lines[lineNumber];
        const trimmed = line.trim();
        const openTag = trimmed.match(/^<([A-Za-z_][A-Za-z0-9_]*)>$/);
        const closeTag = trimmed.match(/^<\/([A-Za-z_][A-Za-z0-9_]*)>$/);

        if (trimmed.startsWith('<') && !openTag && !closeTag) {
            report(lineNumber, line.indexOf('<'), line.length, 'Malformed block tag. Expected <define>, <execute>, or a matching closing tag.');
        } else if (openTag) {
            const tag = openTag[1];
            if (!BLOCK_TYPES.has(tag)) {
                report(lineNumber, line.indexOf('<'), line.length, `Unknown Blocky block <${tag}>.`);
            }
            if (openBlock) {
                report(lineNumber, line.indexOf('<'), line.length, `Block <${tag}> opened before </${openBlock.name}> closed.`);
            } else {
                openBlock = { name: tag, line: lineNumber };
            }
        } else if (closeTag) {
            const tag = closeTag[1];
            if (!openBlock) {
                report(lineNumber, line.indexOf('<'), line.length, `Closing block </${tag}> has no matching opening tag.`);
            } else if (openBlock.name !== tag) {
                report(lineNumber, line.indexOf('<'), line.length, `Expected </${openBlock.name}> but found </${tag}>.`);
                openBlock = null;
            } else {
                openBlock = null;
            }
        }

        for (let column = 0; column < line.length; column++) {
            const character = line[column];
            if (character === '"') {
                inString = !inString;
                continue;
            }
            if (inString) continue;

            if (character === '(' || character === '{') {
                delimiters.push({ character, line: lineNumber, column });
            } else if (character === ')' || character === '}') {
                const expected = character === ')' ? '(' : '{';
                const previous = delimiters.pop();
                if (!previous || previous.character !== expected) {
                    report(lineNumber, column, column + 1, `Unmatched '${character}'.`);
                }
            }
        }

        if (inString && lineNumber < lines.length - 1) {
            report(lineNumber, Math.max(0, line.lastIndexOf('"')), line.length, 'String literal is not closed on this line.');
            inString = false;
        }
    }

    if (inString) {
        const last = lines.length - 1;
        report(last, Math.max(0, (lines[last] || '').lastIndexOf('"')), (lines[last] || '').length, 'Unclosed string literal.');
    }
    for (const delimiter of delimiters) {
        const closing = delimiter.character === '(' ? ')' : '}';
        report(delimiter.line, delimiter.column, delimiter.column + 1, `Unclosed '${delimiter.character}'; expected '${closing}'.`);
    }
    if (openBlock) {
        const line = lines[openBlock.line] || '';
        report(openBlock.line, line.indexOf('<'), line.length, `Block <${openBlock.name}> is not closed.`);
    }

    return diagnostics;
}

function provideTypeHints(document, requestedRange) {
    const lines = document.getText().split(/\r?\n/);
    const declarations = [];
    const types = new Map();
    const modifierEdits = [];
    let currentBlock = null;

    for (let lineNumber = 0; lineNumber < lines.length; lineNumber++) {
        const line = lines[lineNumber];
        const mutDeclaration = line.match(/^\s*mut\s+([A-Za-z_][A-Za-z0-9_]*)\s*;?\s*$/);
        const mutMethod = line.match(/\b([A-Za-z_][A-Za-z0-9_]*)\.set_AcMods\s*\(\s*"([^"]*)"\s*\)/i);
        const mutSpaceCall = line.match(/\bset_AcMods\s+([A-Za-z_][A-Za-z0-9_]*)\s*,\s*"([^"]*)"/i);
        if (mutDeclaration) {
            modifierEdits.push({ name: mutDeclaration[1], line: lineNumber, mutable: true });
        }
        const methodEdit = mutMethod && { name: mutMethod[1], mutable: /\bmutabl\b/i.test(mutMethod[2]) };
        const spaceEdit = mutSpaceCall && { name: mutSpaceCall[1], mutable: /\bmutabl\b/i.test(mutSpaceCall[2]) };
        const modifierEdit = methodEdit || spaceEdit;
        if (modifierEdit) {
            modifierEdits.push({ name: modifierEdit.name, line: lineNumber, mutable: modifierEdit.mutable });
        }
        const block = line.trim().match(/^<(define|execute)>$/);
        const close = line.trim().match(/^<\/(define|execute)>$/);
        if (block) currentBlock = block[1];
        if (currentBlock === 'define') {
            const declaration = line.match(/^\s*let\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s*,\s*(String|Number|Bool|Boolean))?\s*(?:=\s*(.*?))?\s*;?\s*$/);
            if (declaration) {
                const declaredType = declaration[2] === 'Bool' ? 'Boolean' : declaration[2];
                const item = { name: declaration[1], line: lineNumber, declaredType, initializer: declaration[3] };
                declarations.push(item);
                types.set(item.name, item.declaredType || (item.initializer === undefined ? 'Undefined' : undefined));
            }
        }
        if (close) currentBlock = null;
    }

    for (let pass = 0; pass <= declarations.length; pass++) {
        let changed = false;
        for (const declaration of declarations) {
            if (declaration.declaredType || declaration.initializer === undefined) continue;
            const inferred = inferType(declaration.initializer, types);
            if (inferred && types.get(declaration.name) !== inferred) {
                types.set(declaration.name, inferred);
                changed = true;
            }
        }
        if (!changed) break;
    }

    // Snapshot access at each declaration; later modifier changes must not
    // retroactively mark the declaration as mutable.
    const mutableState = new Set();
    let editIndex = 0;
    const orderedEdits = modifierEdits.slice().sort((left, right) => left.line - right.line);
    for (const declaration of declarations) {
        while (editIndex < orderedEdits.length && orderedEdits[editIndex].line < declaration.line) {
            const edit = orderedEdits[editIndex++];
            if (edit.mutable) mutableState.add(edit.name);
            else mutableState.delete(edit.name);
        }
        declaration.mutableAtDeclaration = mutableState.has(declaration.name);
    }

    const hints = [];
    for (const declaration of declarations) {
        if (declaration.line < requestedRange.start.line || declaration.line > requestedRange.end.line) continue;
        const type = types.get(declaration.name);
        if (!type) continue;

        const line = lines[declaration.line];
        const nameStart = line.indexOf(declaration.name, line.indexOf('let') + 3);
        const accessHint = declaration.mutableAtDeclaration ? ':M' : '';
        const hint = new vscode.InlayHint(
            new vscode.Position(declaration.line, nameStart + declaration.name.length),
            `: ${type}${accessHint}`,
            vscode.InlayHintKind.Type
        );
        hint.paddingRight = true;
        hint.tooltip = new vscode.MarkdownString(`Inferred Blocky type: **${type}**`);
        hints.push(hint);
    }

    mutableState.clear();
    for (const edit of modifierEdits) {
        const type = types.get(edit.name);
        if (!type || edit.line < requestedRange.start.line || edit.line > requestedRange.end.line) {
            if (edit.mutable) mutableState.add(edit.name);
            else mutableState.delete(edit.name);
            continue;
        }
        const wasMutable = mutableState.has(edit.name);
        const before = `${type}${wasMutable ? ':M' : ''}`;
        if (edit.mutable) mutableState.add(edit.name);
        else mutableState.delete(edit.name);
        const after = `${type}${edit.mutable ? ':M' : ''}`;
        const line = lines[edit.line];
        const hint = new vscode.InlayHint(
            new vscode.Position(edit.line, line.length),
            `: ${before} → ${after}`,
            vscode.InlayHintKind.Type
        );
        hint.paddingLeft = true;
        hint.tooltip = new vscode.MarkdownString(`Access modifier update for **${edit.name}**.`);
        hints.push(hint);
    }
    return hints;
}

function inferType(expression, knownTypes) {
    const value = expression.trim().replace(/;\s*$/, '').trim();
    if (/^"String"\.new\(\)$/i.test(value) || /^new\s*\(\s*"String"\s*\)$/i.test(value)) return 'String';
    if (/^"Number"\.new\(\)$/i.test(value) || /^new\s*\(\s*"Number"\s*\)$/i.test(value)) return 'Number';
    if (/^"Bool"\.new\(\)$/i.test(value) || /^new\s*\(\s*"Bool"\s*\)$/i.test(value)) return 'Boolean';
    if (/^"(?:[^"\\]|\\.)*"$/.test(value)) return 'String';
    if (/^-?\d+$/.test(value)) return 'Number';
    if (/^(true|false)$/.test(value)) return 'Boolean';
    if (/^(inc_one\b|type\b|origin\b|get_AcMods\b)/.test(value)) {
        return value.startsWith('inc_one') ? 'Number' : 'String';
    }
    const method = value.match(/\.([A-Za-z_][A-Za-z0-9_]*)\s*\(/);
    if (method && ['type', 'origin', 'get_AcMods'].includes(method[1])) return 'String';
    if (/^"(?:String|Number|Bool)"\.new\(\)$/i.test(value)) {
        return value.toLowerCase().includes('number') ? 'Number' : value.toLowerCase().includes('bool') ? 'Boolean' : 'String';
    }
    if (/^new\s+"(?:String|Number|Bool)"$/i.test(value)) {
        return value.toLowerCase().includes('number') ? 'Number' : value.toLowerCase().includes('bool') ? 'Boolean' : 'String';
    }
    if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) return knownTypes.get(value);
    return undefined;
}

function updateVisibleEditors() {
    const enabled = vscode.workspace.getConfiguration('blocky').get('inlineDiagnostics', true);
    for (const editor of vscode.window.visibleTextEditors) {
        if (!enabled || editor.document.languageId !== 'blocky') {
            editor.setDecorations(errorDecoration, []);
            editor.setDecorations(warningDecoration, []);
            continue;
        }

        const diagnostics = diagnosticsByUri.get(editor.document.uri.toString()) || [];
        const errors = [];
        const warnings = [];
        for (const diagnostic of diagnostics) {
            const line = editor.document.lineAt(diagnostic.range.start.line);
            const options = {
                range: new vscode.Range(diagnostic.range.start.line, line.text.length, diagnostic.range.start.line, line.text.length),
                renderOptions: {
                    after: {
                        contentText: `  ${diagnostic.message}`,
                        color: new vscode.ThemeColor(diagnostic.severity === vscode.DiagnosticSeverity.Error ? 'errorForeground' : 'editorWarning.foreground')
                    }
                }
            };
            (diagnostic.severity === vscode.DiagnosticSeverity.Error ? errors : warnings).push(options);
        }
        editor.setDecorations(errorDecoration, errors);
        editor.setDecorations(warningDecoration, warnings);
    }
}

module.exports = { activate, deactivate };
