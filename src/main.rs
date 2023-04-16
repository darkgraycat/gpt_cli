mod macros;
mod response;

use reqwest::blocking::Client;
use reqwest::Error;
use response::Response;

const CHAT_GPT_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const CHAT_GPT_MODEL: &'static str = "gpt-3.5-turbo";

fn ask_gpt(session: &mut Session, question: &str) -> Result<Response, Error> {
    let payload = format!(
        r#"{{
        "model": "{}",
        "messages": [
            {{ "role": "system", "content": "{}" }},
            {{ "role": "user", "content": "{}" }}
        ]
    }}"#,
        CHAT_GPT_MODEL, session.system_info, question
    );
    let bearer_apikey = format!("Bearer {}", session.apikey);
    let client = Client::new();
    let request = client
        .post(CHAT_GPT_URL)
        .header("Authorization", bearer_apikey)
        .header("Content-Type", "application/json")
        .body(payload);
    let response = request.send()?;
    response.json()
}

#[derive(Default, Debug)]
struct Session {
    apikey: String,
    system_info: String,
}

fn main() {
    let mut session = Session::default();
    print!("Enter apikey > ");
    session.apikey = readln!();
    session.system_info = "Please provide short answers".to_owned();

    let response = ask_gpt(&mut session, "How do you feel today");
    println!("R:{:?}", &response);
    println!("Answer {}", response.unwrap().parse());
}
