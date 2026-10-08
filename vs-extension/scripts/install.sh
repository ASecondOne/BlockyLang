#!/usr/bin/env bash
set -euo pipefail

EXTENSION_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VSCODE_COMMAND=""

if command -v code >/dev/null 2>&1; then
    VSCODE_COMMAND="code"
elif command -v codium >/dev/null 2>&1; then
    VSCODE_COMMAND="codium"
else
    echo "Could not find the VS Code or VSCodium command-line tool (code/codium)." >&2
    exit 1
fi

EXTENSIONS_DIR="${VSCODE_EXTENSIONS_DIR:-${HOME}/.vscode/extensions}"
INSTALL_PATH="${EXTENSIONS_DIR}/blocky-lang-local.blocky-lang-0.1.0"

mkdir -p "${EXTENSIONS_DIR}"

if [[ -e "${INSTALL_PATH}" || -L "${INSTALL_PATH}" ]]; then
    echo "An extension install already exists at: ${INSTALL_PATH}" >&2
    echo "Remove or move that exact path before reinstalling; nothing was overwritten." >&2
    exit 1
fi

ln -s "${EXTENSION_ROOT}" "${INSTALL_PATH}"
echo "Installed Blocky Lang extension for ${VSCODE_COMMAND} at: ${INSTALL_PATH}"
echo "Reload VS Code to activate it."
