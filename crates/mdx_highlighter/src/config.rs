pub struct HighlighterConfig {
    pub line_number: bool
}

impl Default for HighlighterConfig {
    fn default() -> Self {
        Self {
            line_number: true
        }
    }
}
