use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Session<T> {
    pub model: String,
    pub messages: Vec<T>,
    #[serde(skip)]
    pub apikey: String,
}
impl<T> Session<T> {
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
}
