use crate::{Result, Error};
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
    pub WEB_FOLDER: String
}
impl EnvVariableContainer {
    pub fn load_vars_from_env() -> Result<EnvVariableContainer> {
        Ok(EnvVariableContainer {
            WEB_FOLDER: get_var_from_env_parsed("WEB_FOLDER")?
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