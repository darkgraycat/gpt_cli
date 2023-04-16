use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    id: String,
    object: String,
    model: String,
    usage: Usage,
    pub choices: Vec<Choice>,
}
impl Response {
    pub fn parse(self) -> String {
        self.choices[0].message.content.to_owned()
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
    pub message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}
