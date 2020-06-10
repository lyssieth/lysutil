use super::ConfigError;

trait Config {
    fn path(&self) -> String;

    fn read() -> Result<Box<Self>, ConfigError> {
        Err(ConfigError::default())
    }
}
