use std::io::{self, Write};

pub fn init() -> String{
    print!("¿Do you want to add another user? (y/n): ");
        
    io::stdout().flush().expect("Failed to flush stdout");
    let mut should_ask_another = String::from("");
    io::stdin().read_line(&mut should_ask_another).unwrap();
    return should_ask_another
}