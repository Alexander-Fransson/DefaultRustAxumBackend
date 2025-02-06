pub mod error;
pub mod traits_for_proc_macros;
mod utils_test;

use std::collections::HashMap;
use serde::Serialize;
pub use serde_json;

pub use error::{Error, Result};

pub fn turn_struct_with_serde_serialize_into_hashmap<T:Serialize>(data: T)
-> Result<HashMap<String, String>>{
    let json_value = serde_json::to_value(data)?; 
    
    let map_value = json_value.as_object().ok_or(Error::FailedToTurnJsonValueIntoMap)?;

    let hashmap: Result<HashMap<String, String>> = map_value.into_iter().map(|(key, value)| {

        let str_value = value.as_str();
        
        if let Some(value) = str_value {
            return Ok((key.clone(), value.to_string()));
        } else {
            return Err(Error::FailedToTurnJsonValueIntoStr);
        }
    }).collect();

    Ok(hashmap?)
}

// also make tests for this

// macro rules operate on a level which makes it unable to access the keys of a struct before it is created
// I realy need to learn more about proc macros and so on but for the base I think that the derive macro you did will probably suffice
// Or maybe you can get it through a functional proc macro