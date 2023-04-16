use crate::response::Message;

#[derive(Debug)]
pub struct Session {
    pub apikey: String,
    messages: Vec<Message>,
}
impl Session {
    pub fn new(apikey: &str) -> Session {
        Session {
            apikey: apikey.to_owned(),
            messages: Vec::new(),
        }
    }
    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    pub fn get_messages(&self) -> String {
        self.messages
            .iter()
            .map(|msg| msg.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
