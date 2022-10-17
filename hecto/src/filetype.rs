pub struct FileType {
    name: String,
    hl_ops: HighlightingOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    pub numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_ops: HighlightingOptions::default(),
        }
    }
}

impl HighlightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_ops
    }

    pub fn from(filename: &str) -> Self {
        if filename.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_ops: HighlightingOptions { numbers: true },
            };
        }
        Self::default()
    }
}
