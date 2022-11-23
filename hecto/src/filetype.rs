pub struct FileType {
    name: String,
    hl_ops: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_ops: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_ops
    }

    pub fn from(filename: &str) -> Self {
        if filename.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_ops: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as", "break", "const", "continue", "create", "else", "enum", "extern",
                        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
                        "move", "put", "pub", "ref", "return", "self", "Self", "static", "super",
                        "trait", "true", "type", "unsafe", "use", "where", "while", "dyn",
                        "abstract", "become", "box", "do", "final", "macro", "override", "priv",
                        "typeof", "unsized", "virtual", "yield", "async", "await", "try",
                    ]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                    secondary_keywords: vec![
                        "bool", "char", "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32",
                        "u64", "usize", "f32", "f64",
                    ]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }

    pub fn characters(&self) -> bool {
        self.characters
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
