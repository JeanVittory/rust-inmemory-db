
use std::io::{self, Write};

pub fn init(mut should_ask_another: &mut String) -> &String{
    if should_ask_another.trim().to_uppercase() != "N" && should_ask_another.trim().to_uppercase() != "Y" {
        let mut should_ask_again =  true;
        while should_ask_again {
            print!("We don't get your answer. Please write down 'y' if you want to add a new user or 'n' if you don't: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut should_ask_another_retry = String::from("");
            io::stdin().read_line(&mut should_ask_another_retry).unwrap();
            should_ask_another_retry = should_ask_another_retry.trim().to_uppercase();

            if should_ask_another_retry == "N" || should_ask_another_retry == "Y" {
                should_ask_again = false;
                *should_ask_another = should_ask_another_retry;
            }
        }
    }
    return should_ask_another;
}