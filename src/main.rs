mod macros;

use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Default, Debug)]
struct Session {
    apikey: String,
}

fn print_jokes() {
    #[derive(Deserialize)]
    struct ChuckNorrisJoke {
        value: String,
    }
    let url = "https://api.chucknorris.io/jokes/random";
    loop {
        let response: ChuckNorrisJoke = get(url).unwrap().json().unwrap();
        println!("{}", response.value);
        print!("Want more? ");
        match readln!().as_str() {
            "no" | "quit" | "exit" => break,
            _ => continue,
        }
    }
}

fn main() {
    //let mut session = Session::default();
    //print!("Enter apikey > ");
    //session.apikey = readln!();
    println!("Hi");
    print_jokes();
}
