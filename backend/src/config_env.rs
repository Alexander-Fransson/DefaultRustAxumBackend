use crate::{utils::base64::b64_to_u8, Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

// statically load env variables as to not have to construct them every time you need them
pub fn get_env_variables() -> &'static EnvVariableContainer {
    static INSTANCE: OnceLock<EnvVariableContainer> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        EnvVariableContainer::load_vars_from_env()
        .unwrap_or_else(|ex| {
            panic!("Failed to load EnvVariableContainer: {}", ex)
        })
    })
}

#[allow(non_snake_case)]
pub struct EnvVariableContainer {
    //pub WEB_FOLDER: String,
    pub DB_CONNECTION_STRING: String,
    pub DB_DEFAULT_USER_CONNECTION_STRING: String,
    pub LISTENER_URL: &'static str,
    pub JWT_KEY: Vec<u8>,
    pub JWT_TOKEN_DURRATION_SEC: f64
}
impl EnvVariableContainer {
    pub fn load_vars_from_env() -> Result<EnvVariableContainer> {
        Ok(EnvVariableContainer {
            //WEB_FOLDER: get_var_from_env_parsed("WEB_FOLDER")?,
            DB_CONNECTION_STRING: get_var_from_env_parsed("DB_CONNECTION_STRING")?,
            DB_DEFAULT_USER_CONNECTION_STRING: get_var_from_env_parsed("DB_DEFAULT_USER_CONNECTION_STRING")?,
            LISTENER_URL: "127.0.0.1:3000",
            JWT_KEY: get_env_b64_var_as_u8("JWT_KEY")?,
            JWT_TOKEN_DURRATION_SEC: get_var_from_env_parsed("JWT_TOKEN_DURRATION_SEC")?
        })
    }
}

fn get_specific_var_from_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::CannotFindEnvWithSuchName(name))
}

fn get_var_from_env_parsed<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_specific_var_from_env(name)?;

    val.parse::<T>().map_err(|_| Error::FailedToParse(name))
}

fn get_env_b64_var_as_u8(name: &'static str) -> Result<Vec<u8>> {
    let val = get_specific_var_from_env(name)?;
    let bytes = b64_to_u8(&val)?;
    
    Ok(bytes)
}