use serde::{Serialize, Deserialize};

pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Response {
    id: String,
    object: String,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}
impl Response {
    pub fn get_message(mut self) -> Message {
        self.choices.remove(0).message
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
impl Message {
    pub fn new(role: Role, content: &str) -> Message {
        Message {
            content: content.to_owned(),
            role: match role {
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::User => "user",
            }
            .to_owned(),
        }
    }
}
