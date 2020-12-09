mod log {
    #[macro_export]
    macro_rules! info{
        ($($text:expr), *) => {
            print!("\n");
            $(print!("[INFO] {}\n", $text);)*
            print!("\n");
        };
    }

    #[macro_export]
    macro_rules! debug{
        ($($text:expr), *) => {
            print!("\n");
            $(print!("[DEBUG] {}", $text);)*
            print!("...Debug message\n");
        };
    }

    #[macro_export]
    macro_rules! error{
        ($($text:expr), *) => {
            print!("\n");
            $(print!("[ERROR] {}", $text);)*
            print!("...Error message\n");
        };
    }
}