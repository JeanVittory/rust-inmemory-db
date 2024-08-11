use crate::utils::{input_handlers, validators};

pub fn init() -> f64{
    let mut ask_available_money = true;
    let mut available_money:f64 = 0.0;
    while ask_available_money {
        let available_money_entered = input_handlers::ask_available_money();
        match validators::available_money_validator(&available_money_entered){
            Ok(money) => {
                ask_available_money = false;
                available_money = money;
            },
            Err(e) =>{
                println!("{}", e);
            } 
        };
    }
    return available_money;
}