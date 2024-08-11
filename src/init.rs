use std::io::{self, Write};
use crate::components::entities::users::instance_user;
use crate::components::process_fields::process_address;
use crate::components::process_fields::process_age;
use crate::components::process_fields::process_available_money;
use crate::components::process_fields::process_lastname;
use crate::components::process_fields::process_name;
use crate::utils::add_another_user;
use crate::structs::person::Person;

pub fn start(){
    println!("Hello, please add a new user");

    let mut list_user: Vec<Person> = Vec::new();
    let mut should_add_new_user:bool = true;

    while should_add_new_user {

        let name = process_name::init();
        let lastname = process_lastname::init();
        let age = process_age::init();
        let address = process_address::init();
        let available_money = process_available_money::init();

        let user = instance_user((name, lastname, age, address, available_money));

        list_user.push(user);
        let mut should_ask_another = add_another_user::init();

        
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
                    should_ask_another = should_ask_another_retry;
                }
            }
        }
        if should_ask_another.trim().to_uppercase() == "N" {
            should_add_new_user = false;
        }
    }

    println!("El resultado es {:#?}", list_user)
}

