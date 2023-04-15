mod macros;
mod http;

#[derive(Debug)]
#[derive(Default)]
struct Session {
    apikey: String,
}

fn main() {
    //let mut session = Session::default();
    //print!("Enter apikey > ");
    //session.apikey = readln!();

    use http::HttpMethod::Get;
    let resp = http::make_request("api.chucknorris.io", 443, Get, "/jokes/random");
    match resp {
        Ok(data) => println!("Data: {}", data),
        Err(err) => eprintln!("Error {}", err),
    }
}
