use std::fmt::Display;

pub mod json;
pub mod toml;

/// An error given from various Config systems  
/// Derives: Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Debug
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Debug)]
struct ConfigError {
    /// The cause of the error
    pub cause: &'static str,
    /// Info relating to the error
    pub info: &'static str,
}

impl Display for ConfigError {
    /// Formats as: "Error at \`{}\`: {}"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at `{}`: {}", self.cause, self.info)
    }
}
