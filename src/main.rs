use std::io::{stdin, Read};
use regex::Regex;
use rand::seq::SliceRandom;
use rand::thread_rng;

enum Level {
    Weak,
    Medium,
    Strong,
}

pub struct Obfuscator {
    level: Level,
    rename: bool,
    reorder: bool,
    code: String,
    symbols: Vec<String>,
}

impl Obfuscator {
    pub fn new(level: Level, code: String) -> Self {
        let rename = match level {
            Level::Weak => false,
            Level::Medium | Level::Strong => true,
        };
        let reorder = match level {
            Level::Strong => true,
            _ => false,
        };

        Obfuscator {
            level,
            rename,
            reorder,
            code,
            symbols: Vec::new(),
        }
    }

    pub fn obfuscate(&mut self) -> String {
        if self.rename {
            self.rename_symbols();
        }
        if self.reorder {
            self.reorder_code();
        }
        self.code.clone()
    }

    pub fn rename_symbols(&mut self) {
        let mut rng = thread_rng();
        let symbol_regex = Regex::new(r"([a-zA-Z_]\w*)").unwrap();
        let mut unique_symbols: Vec<String> = symbol_regex
            .find_iter(&self.code)
            .map(|m| m.as_str().to_string())
            .collect();
        unique_symbols.sort();
        unique_symbols.dedup();
        unique_symbols.shuffle(&mut rng);

        let reserved_keywords = vec![
            "abstract", "alignof", "as", "async", "await", "become", "box", "break", "const",
            "continue", "crate", "do", "dyn", "else", "enum", "extern", "false", "final",
            "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move",
            "mut", "override", "priv", "pub", "ref", "return", "Self", "self", "sizeof",
            "static", "struct", "super", "trait", "true", "try", "type", "typeof", "unsafe",
            "unsized", "use", "virtual", "where", "while", "yield", "main",
        ];

        for symbol in &unique_symbols {
            if !reserved_keywords.contains(&symbol.as_str()) {
                let new_symbol = format!("__{}", symbol);
                self.code = self.code.replace(symbol, &new_symbol);
                self.symbols.push(new_symbol);
            }
        }
    }

    pub fn reorder_code(&mut self) {
        let mut rng = thread_rng();
        let mut lines: Vec<&str> = self.code.lines().collect();
        lines.shuffle(&mut rng);
        self.code = lines.join("\n");
    }
}

fn main() {
    let mut s = String::new();

    println!("Enter the code:");
    stdin().read_line(&mut s).unwrap();
    println!("{}", s);

    let mut obfuscator = Obfuscator::new(Level::Strong, s);
    let obfuscated_code = obfuscator.obfuscate();
    println!("Obfuscated code:\n\n{}", obfuscated_code);
}
