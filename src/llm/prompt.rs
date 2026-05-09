pub struct PromptTemplate;

impl PromptTemplate {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, user_prompt: &str) -> String {
        user_prompt.to_string()
    }
}
