use anyhow::Result;
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage},
};

use crate::ENV;

pub struct BaseAI {
    messages: Vec<ChatCompletionMessage>,
}

impl BaseAI {
    pub fn new() -> BaseAI {
        BaseAI {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: ChatCompletionMessage) {
        self.messages.push(message);
    }

    pub async fn get_response(&mut self) -> Result<ChatCompletionMessage> {
        let mut response = ChatCompletion::builder("gpt-4o-mini", self.messages.clone())
            .credentials(Credentials::new(
                ENV.get_var("API_KEY".to_string())
                    .expect(".env must have `API_KEY` key"),
                "https://api.openai.com/v1/",
            ))
            .max_completion_tokens(2048u64)
            .create()
            .await?;
        let first = response.choices.remove(0);
        Ok(first.message)
    }
}
