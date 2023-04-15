#[macro_export]
macro_rules! readln {
    () => {{
        use std::io::{self, Write};
        let mut buf = String::with_capacity(10);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim_end().to_owned()
    }};
}
