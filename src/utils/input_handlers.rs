use std::io::{self, Write};

pub fn ask_name() -> String {
    print!("Name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut name = String::from("");
    io::stdin().read_line(&mut name).unwrap();
    return name.trim().to_string();
}

pub fn ask_lastame() -> String{
    print!("Lastname: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut lastname = String::from("");
    io::stdin().read_line(&mut lastname).unwrap();
    return lastname.trim().to_string();
}

pub fn ask_age() -> String{
    print!("Age: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut age = String::from("");
    io::stdin().read_line(&mut age).unwrap();
    return age.trim().to_string();
}

pub fn ask_address() -> String{
    print!("Address: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut address = String::from("");
    io::stdin().read_line(&mut address).unwrap();
    return address.trim().to_string();
}

pub fn ask_available_money() -> String{
    print!("Available money (USD): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut available_money = String::from("");
    io::stdin().read_line(&mut available_money).unwrap();
    return available_money.trim().to_string();
}