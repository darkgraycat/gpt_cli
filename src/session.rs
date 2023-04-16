#[derive(Debug)]
pub struct Session<T: ToString> {
    pub apikey: String,
    messages: Vec<T>,
}
impl <T: ToString> Session<T> {
    pub fn new(apikey: &str) -> Session<T> {
        Session {
            apikey: apikey.to_owned(),
            messages: Vec::new(),
        }
    }
    pub fn push_message(&mut self, message: T) {
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
