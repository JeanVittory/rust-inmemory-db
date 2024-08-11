use crate::components::entities::users::instance_user;
use crate::components::process_fields::process_address;
use crate::components::process_fields::process_age;
use crate::components::process_fields::process_available_money;
use crate::components::process_fields::process_lastname;
use crate::components::process_fields::process_name;
use crate::utils::ask_another_again;
use crate::utils::ask_another_user;
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
        let mut should_ask_another = ask_another_user::init();
        ask_another_again::init(&mut should_ask_another);
        

        if should_ask_another.trim().to_uppercase() == "N" {
            should_add_new_user = false;
        }
    }

    println!("The data entered was: {:#?}", list_user)
}

