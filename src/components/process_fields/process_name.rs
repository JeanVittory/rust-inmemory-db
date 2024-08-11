use crate::utils::{input_handlers, validators};

pub fn init() -> String{
    let mut ask_name = true;
    let mut name:String = String::new();

    while ask_name {
        name = input_handlers::ask_name();
        let is_valid_name = validators::name_validator(&name);
        match is_valid_name {
            Ok(valid_name) => {
                ask_name = false;
                name = valid_name
            },
            Err(e)=>{
                println!("{}", e);
            }
        }
    }

    return name;
}