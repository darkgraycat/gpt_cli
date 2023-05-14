use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Result, Write};

const DEFAULT_SESSION_FILENAME: &'static str = "session.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Session<T> {
    pub model: String,
    pub messages: Vec<T>,
    #[serde(skip)]
    pub apikey: String,
}
impl<'de, T: Serialize + Deserialize<'de>> Session<T> {
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

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(DEFAULT_SESSION_FILENAME)?;
        let data = serde_json::to_string(&self)?;
        file.write_all(data.as_bytes())?;
        file.flush()?;
        Ok(())
    }
    pub fn load(apikey: String) -> Result<Session<T>> {
        let mut file = File::open(DEFAULT_SESSION_FILENAME)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut session: Session<T> = serde_json::from_str(Box::leak(data.into_boxed_str()))?;
        session.apikey = apikey;

        Ok(session)
    }
}

