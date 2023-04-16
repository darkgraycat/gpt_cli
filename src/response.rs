use serde::Deserialize;

pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    role: String,
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
            }.to_owned(),
        }
    }
}
impl ToString for Message {
    fn to_string(&self) -> String {
        format!(r#"{{"role":"{}","content":"{}"}}"#, self.role, self.content)
    }
}
