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
        icon_url: String,
    }
    let url = "https://api.chucknorris.io/jokes/random";
    loop {
        let response: ChuckNorrisJoke = get(url).unwrap().json().unwrap();
        println!("{}", response.value);
        print!("Want more? ");
        if readln!() == "no" {
            break;
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
