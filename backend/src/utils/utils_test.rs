#[cfg(test)]
mod tests {

    use super::super::*;
    use serde::Serialize;
    use super::super::error::{Error, Result};

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

        assert_eq!(struct_keys, vec!["a", "b"]);
        assert_eq!(struct_values, vec!["hello", "NULL"]);

        Ok(())
    }
}
