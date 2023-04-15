mod macros;

#[derive(Debug)]
#[derive(Default)]
struct Session {
    apikey: String,
}

fn main() {
    let mut session = Session::default();
    print!("Enter apikey > ");
    session.apikey = readln!();

    println!("Session: {:?}", session);
}
