use crate::utils::{input_handlers, validators};

pub fn init() -> String{
    let mut ask_lastname = true;
    let mut lastname = String::new();
    while ask_lastname {
        lastname = input_handlers::ask_lastame();
        let is_valid_lastname = validators::lastname_validator(&lastname);
        match is_valid_lastname {
            Ok(valid_lastname) => {
                ask_lastname = false;
                lastname = valid_lastname
            }
            Err(e) =>{
                println!("{}", e);
            }
        }
    }
    return lastname;
}