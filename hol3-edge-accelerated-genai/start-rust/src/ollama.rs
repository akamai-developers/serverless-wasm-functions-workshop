use anyhow::Context;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct ChatRequestOptions {
    pub temperature: f32,
    pub num_ctx: u32,
}
#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub options: ChatRequestOptions,
}

impl IntoBody for ChatRequest {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self)
            .context("Failed to serialize ChatRequest to JSON")
            .unwrap()
    }
}   

#[derive(Serialize, Debug, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done: bool,
}
impl ChatRequest {
    pub fn new(prompt: String, model: String, temperature: f32) -> Self {
        ChatRequest {
            model: model,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: Self::get_system_prompt(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
            stream: false,
            options: ChatRequestOptions {
                temperature: temperature,
                num_ctx: 2048,
            }
        }
    }

    fn get_system_prompt() -> String {
        r#"You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.

        Follow the pattern of the following examples:

User: Hi, my name is Bob
Bot: neutral

User: Hi, love the Summer
Bot: positive

User: I am so happy today
Bot: positive

User: I am so sad today
Bot: negative"#.to_string()
    }
}
