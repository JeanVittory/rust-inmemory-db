use crate::structs::person::Person;

pub fn instance_user(user: (String, String, i32, String, f64)) -> Person{
    return Person{
        name: user.0,
        lastname: user.1,
        age: user.2,
        address: user.3,
        available_money: user.4
    };
}