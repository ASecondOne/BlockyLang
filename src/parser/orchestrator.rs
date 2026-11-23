use std::{any::Any, path::PathBuf};

use crate::parser::prepations::prep_raw_source;

pub struct SourceFile {
    path: PathBuf
}

pub struct SourceBlock {

}

pub struct SourceLine {

}

pub enum SourceSymbols {

}

struct RawSourceTree {
    pub file: Option<PathBuf>,
    pub folder: Option<Vec<RawSourceTree>>,
    pub source: Option<PathBuf>
}

pub fn orchestrator(source: PathBuf) {
    let indexed_raw_source_tree = file_indexing(None, Some(source));

    let raw_formated_tree = prep_raw_source("".to_string());
}

fn file_indexing(source: Option<RawSourceTree>, path: Option<PathBuf>) -> RawSourceTree {
    RawSourceTree {
        file: {
            if let Some(s) = &source {
                if let Some(s) = &s.source {
                    if s.is_file() {
                        Some(s);
                    }
                }
            }
            
            None
        },
        folder: {
            if let Some(s) = source {
                if let Some(s) = s.source {
                    if s.is_dir() {
                        Some(file_indexing(None, Some(s)));
                    }
                }
            }

            None
        },
        source: None
    }
}