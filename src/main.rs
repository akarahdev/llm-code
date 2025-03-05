use std::sync::LazyLock;

use anyhow::Result;
use base::BaseAI;
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use rust_dotenv::dotenv::DotEnv;

pub static ENV: LazyLock<DotEnv> = LazyLock::new(|| DotEnv::new(""));

mod base;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ai = BaseAI::new();
    ai.add_message(ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("hi".to_string()),
        ..Default::default()
    });
    loop {
        ai.add_message(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some("This is a user's message for testing, say hi!".into()),
            ..Default::default()
        });
        let response = ai.get_response().await;
        println!("{:#?}", response);
    }
}
