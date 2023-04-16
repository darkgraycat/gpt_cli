mod macros;
mod response;
mod session;

use reqwest::blocking::Client;
use reqwest::Error;
use session::Session;
use response::{Response, Message, Role};

const CHAT_GPT_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const CHAT_GPT_MODEL: &'static str = "gpt-3.5-turbo";
const DEFAULT_SYS_USER_MESSAGE: &'static str = "Please provide short answers";

fn ask_gpt(session: &mut Session) -> Result<Response, Error> {
    let payload = format!(
        r#"{{
            "model": "{}",
            "messages": [{}]
        }}"#,
        CHAT_GPT_MODEL,
        session.get_messages()
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

fn main() {
    print!("Enter apikey > ");
    let input = readln!();
    let mut session = Session::new(&input);

    print!("Enter context or leave empty > ");
    let input = readln!();
    let sys_context = if input.is_empty() { DEFAULT_SYS_USER_MESSAGE } else { &input };
    session.push_message(Message::new(Role::System, sys_context));

    println!("Type \'quit\' to close application");
    loop {
        print!("--------------------------------\n> ");
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
                session.push_message(Message::new(Role::Assistant, message.content.as_str()));
            },
            Err(err) => eprintln!("Error occured: {}", err),
        }
    }
}
