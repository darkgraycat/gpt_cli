mod macros;
mod response;
mod session;

use reqwest::blocking::Client;
use reqwest::Error;
use response::{Message, Response, Role};
use serde_json;
use session::Session;

const CHAT_GPT_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const CHAT_GPT_MODEL: &'static str = "gpt-3.5-turbo";
const DEFAULT_SYS_USER_MESSAGE: &'static str = "Please provide short answers";
const SEPARATOR: &'static str = "---------------- type 'quit' to exit ----------------";

fn ask_gpt(session: &mut Session<Message>) -> Result<Response, Error> {
    let payload = serde_json::to_string(&session).unwrap();
    let request = Client::new()
        .post(CHAT_GPT_URL)
        .header("Authorization", format!("Bearer {}", session.apikey))
        .header("Content-Type", "application/json")
        .body(payload);
    let response = request.send()?;
    response.json()
}

fn main() {
    print!("Enter apikey > ");
    let input = readln!();
    let mut session = Session::<Message>::new(input, String::from(CHAT_GPT_MODEL));

    print!("Enter context or leave empty > ");
    let input = readln!();

    session.push_message(Message::new(
        Role::System,
        if input.is_empty() {
            DEFAULT_SYS_USER_MESSAGE
        } else {
            &input
        },
    ));

    loop {
        print!("{}\n> ", SEPARATOR);
        let input = readln!();
        if input == "quit" {
            break;
        }
        session.push_message(Message::new(Role::User, &input));
        let response = ask_gpt(&mut session);
        match response {
            Ok(r) => {
                let message = r.get_message();
                println!("{}", message.content);
                session.push_message(message);
            }
            Err(err) => eprintln!("Error occured: {}", err),
        }
    }
}
