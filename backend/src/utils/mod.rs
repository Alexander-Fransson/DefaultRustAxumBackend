pub mod error;
pub mod traits_for_proc_macros;
mod utils_test;

use std::collections::HashMap;
use serde::Serialize;

// hashmap creates does not order its keys but postgres demands order

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
            return Ok((key.clone(), "NULL".to_string()));
        }
    }).collect();

    Ok(hashmap?)
}