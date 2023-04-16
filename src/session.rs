use serde::Serialize;

#[derive(Debug)]
pub struct Session<T> {
    pub apikey: String,
    pub model: String,
    messages: Vec<T>,
}
impl<T: Clone> Session<T> {
    pub fn new(apikey: String, model: String) -> Session<T> {
        Session {
            apikey,
            model,
            messages: Vec::new(),
        }
    }
    pub fn push_message(&mut self, message: T) {
        self.messages.push(message);
    }
    pub fn to_payload(&self) -> Payload<T> {
        Payload {
            model: self.model.to_owned(),
            messages: self.messages.to_owned()
        }
    }
}

#[derive(Serialize)]
pub struct Payload<T> {
    model: String,
    messages: Vec<T>,
}
