use crate::utils::{input_handlers, validators};

pub fn init() -> i32{
    let mut ask_age = true;
    let mut age:i32 = 0;
    while ask_age {
        let user_age = input_handlers::ask_age();
        match validators::age_validator(&user_age){
            Ok(is_valid) => {
                ask_age = false;
                age = is_valid
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    return age;
}