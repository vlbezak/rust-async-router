use crate::{Result,Error};

// e.g. my_key: 123
pub fn key_num(item: &str) -> Result<(&str, i32)> {
    if let Some((key,val)) = item.split_once(':') {
        if let Ok(val) = val.trim().parse::<i32>() {
            Ok((key,val))
        }
        else{
            Err(Error::Custom("Invalid num".to_string()))
        }
    }
    else {
        Err(Error::Custom("Invalid item".to_string()))
    }
}

// Let else demonstration - guards
pub fn key_num_let_else(item: &str) -> Result<(&str, i32)> {
    let Some((key,val)) = item.split_once(":") else {
        return Err(Error::Custom("Invalid item".to_string()));
    };
    let Ok(val) = val.trim().parse::<i32>() else {
        return Err(Error::Custom("Invalid item".to_string()));
    };
    Ok((key,val))
}