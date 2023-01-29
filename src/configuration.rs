use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    // let mut settings = config::Config::builder();
    //
    // // Add configuration values from a file named `configuration`
    // // It will look for any top-level file with an extension
    // // that `config` knows how to parse: yaml, json, etc.
    // settings.add_source(config::File::with_name("configuration")).build().unwrap();
    //
    // // Try to convert the configuration values it read into
    // // our Settings type
    // // settings.try_into()

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    // Read the "default" configuration file
    // Default to `local` if unspecified.
    // let environment: Environment = std::env::var("APP_ENVIRONMENT")
    //     .unwrap_or_else(|_| "local".into())
    //     .try_into()
    //     .expect("Failed to parse APP_ENVIRONMENT");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let builder = config::Config::builder()
        // Add from file
        .add_source(
            // config::File::new("configuration/base.yaml", config::FileFormat::Yaml).required(true),
            config::File::new(
                &configuration_directory.join("base").display().to_string(),
                config::FileFormat::Yaml,
            )
            .required(true),
        )
        // Add from env
        .add_source(
            config::File::new(
                &configuration_directory
                    .join(environment.as_str())
                    .as_path()
                    .display()
                    .to_string(),
                config::FileFormat::Yaml,
            )
            .required(true),
        )
        // Generate  from environment to use in cloud env
        .add_source(config::Environment::with_prefix("app").separator("__"));
    // Layer on the environment-specific values.
    // .set_override("APP_ENVIRONMENT", environment.as_str());

    // build configuration
    let settings = builder.build().expect("Could not load configuration.");

    // Return the parsed type
    settings.try_deserialize::<Settings>()
}

/// The possible runtime environment for our application
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a suppoerted environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
