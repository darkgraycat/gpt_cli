#[macro_export]
macro_rules! readln {
    () => {{
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut buf = String::with_capacity(10);
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim_end().to_owned()
    }};
}

