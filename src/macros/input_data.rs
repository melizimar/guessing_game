#[macro_export]
macro_rules! input_data {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading input");
        input.trim().to_string()
    }};

    ($ty:ty) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading input");
        input.trim().parse::<$ty>().expect("Failed to parse")
    }};

    ($msg:expr) => {{
        eprint!("{}", $msg);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading input");
        input.trim().to_string()
    }};

    ($ty:ty, $msg:expr) => {{
        eprint!("{}", $msg);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error while reading input");
        input.trim().parse::<$ty>().expect("Failed to parse")
    }};
}