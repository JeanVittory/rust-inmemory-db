pub fn name_validator(name: &str) -> Result<String, String>{
    if name.is_empty() {
        return Err("The name only can have alphabetic values".to_string());
    }
    let is_valid_string = name.chars().all(|char| char.is_alphabetic() || char.is_whitespace());
    if is_valid_string {
        return Ok(name.to_string());
    }else{
       return Err("The name only can have alphabetic values. Please try it again".to_string());
    }
}

pub fn lastname_validator(lastname: &str) -> Result<String, String>{
    if lastname.is_empty() {
        return Err("The lastname only can have alphabetic values. Please try it again".to_string());
    }

    let is_valid_lastname = lastname.chars().all(|char| char.is_alphabetic()  || char.is_whitespace());
    if is_valid_lastname {
        return Ok(lastname.to_string());
    }else{
        return Err("The lastname only can have alphabetic values. Please try it again".to_string());
    }
}

pub fn age_validator(age: &str) -> Result<i32, String>{
    if age.is_empty() {
        return Err("The number given should be an integer".to_string());
    }
    age.parse::<i32>().map_err(|_| "The number given should be an integer. Please try it again".to_string())
}

pub fn available_money_validator(money: &str) -> Result<f64, String>{
    if money.is_empty() {
        return Err("The number given should be an integer".to_string());
    }
    money.parse::<f64>().map_err(|_| "The number given should be an integer. Please try it again".to_string())
}