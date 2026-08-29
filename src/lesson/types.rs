/// A module's lesson: prose read before attempting that module's exercises.
///
/// The body is raw markdown loaded verbatim from `exercises/lessons/<id>.md`.
/// Metadata lives in `info.toml` under `[[modules]]` rather than in front
/// matter, so the markdown files stay pure prose that any AI tutor can read
/// without parsing.
#[derive(Debug, Clone)]
pub struct Lesson {
    pub module_id: String,
    /// The first `# ` heading in the body, falling back to the module's theme.
    pub title: String,
    /// Raw markdown, unmodified.
    pub body: String,
    /// The Rust Book chapter this lesson is drawn from.
    pub book_url: Option<String>,
    /// Short concept names this lesson covers.
    pub concepts: Vec<String>,
}

impl Lesson {
    /// Concepts rendered for display, e.g. `"move semantics, Copy, Clone"`.
    /// Empty string when the module lists none.
    pub fn concepts_line(&self) -> String {
        self.concepts.join(", ")
    }
}
