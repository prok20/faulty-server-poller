use std::convert::TryInto;
use std::env;

use crate::configuration::environment::Environment;
use crate::configuration::settings::Settings;
use anyhow::Result;

const CONFIGURATION_PATH: &str = "configuration";

pub fn get_settings() -> Result<Settings> {
    let conf_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(CONFIGURATION_PATH);

    let mut conf = config::Config::default();
    conf.merge(config::File::from(conf_path.join("default")).required(true))?;

    let environment: Environment = env::var("APP_ENVIRONMENT")
        .expect("Failed to get environment. Set APP_ENVIRONMENT to either 'development' or 'production'")
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT. Set it to either 'development' or 'production'");

    conf.merge(config::File::from(conf_path.join(environment.as_str())).required(true))?;

    conf.merge(config::Environment::with_prefix("APP").separator("_"))?;

    Ok(conf.try_into()?)
}
