#[cfg(test)]
mod tests {

    use super::super::*;
    use serde::Serialize;
    use super::super::{
        error::Result,
        base64::{
            string_to_base_64,
            b64_to_string
        },
    }; 

    #[test]
    fn string_to_base_64_ok() -> Result<()> {
        let string = "hello";
        let base64_string = string_to_base_64(string);
        assert_eq!(base64_string, "aGVsbG8=");
        Ok(())
    }

    #[test]
    fn b64_str_to_normal_str_ok() -> Result<()> {
        let b64_string = "aGVsbG8=";
        let normal_string = b64_to_string(b64_string)?;
        assert_eq!(normal_string, "hello");
        Ok(())
    }

    #[test]
    fn turn_struct_with_serde_serialize_into_hashmap_ok() -> Result<()> {
        #[derive(Serialize)]
        struct SomeStruct {
            a: String,
        }

        let some_struct_instance = SomeStruct {
            a: "hello".to_string(),
        };

        let some_struct_hashmap = turn_struct_with_serde_serialize_into_hashmap(some_struct_instance)?;
        let struct_keys = some_struct_hashmap.keys().collect::<Vec<_>>();

        assert_eq!(struct_keys, vec!["a"]);

        Ok(())
    }

    #[test]
    fn turn_struct_with_serde_serialize_into_hashmap_ok_with_option() -> Result<()> {
        #[derive(Serialize)]
        struct SomeStruct {
            a: Option<&'static str>,
            b: Option<String>,
        }

        let some_struct_instance = SomeStruct {
            a: Some("hello"),
            b: None,
        };

        let some_struct_hashmap = turn_struct_with_serde_serialize_into_hashmap(some_struct_instance)?;
        let struct_keys = some_struct_hashmap.keys().collect::<Vec<_>>();
        let struct_values = some_struct_hashmap.values().collect::<Vec<_>>();

        // fun fact the order of the keys is not guaranteed

        assert!(struct_keys.len() == 2);
        assert!(struct_values.len() == 2);

        Ok(())
    }
}
