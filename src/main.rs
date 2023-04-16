mod macros;
mod response;

use reqwest::blocking::Client;
use reqwest::Error;
use response::Response;

const CHAT_GPT_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const CHAT_GPT_MODEL: &'static str = "gpt-3.5-turbo";
const CHAT_GPT_SYSINFO: &'static str = "Please provide short answers";

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
    session.system_info = CHAT_GPT_SYSINFO.to_owned();

    println!("Type \'quit\' to close application");
    loop {
        print!("--------------------------------\n> ");
        let input = readln!();
        if input == "quit" { break; }
        let response = ask_gpt(&mut session, &input);
        match response {
            Ok(r) => println!("{}", r.parse()),
            Err(err) => eprintln!("Error occured: {}", err),
        }
    }
}
