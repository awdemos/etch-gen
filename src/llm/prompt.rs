pub struct PromptTemplate;

impl Default for PromptTemplate {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptTemplate {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, user_prompt: &str) -> String {
        user_prompt.to_string()
    }
}
