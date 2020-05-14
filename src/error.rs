use thiserror::Error;

/// List of error types used in the project.
#[derive(Error, Debug)]
pub enum GndError {
    /// Indicates that an environment variable cannot be loaded.
    #[error("{err} ({env_var})")]
    EnvVarError { env_var: String, err: String },
}
