/// Returned when an attempt to load a plugin fails.
#[derive(thiserror::Error, Debug)]
pub enum PluginLoadError {
    #[error(transparent)]
    WasiError(#[from] wasi_common::Error),
    #[error(transparent)]
    StringArray(#[from] wasi_common::StringArrayError),
    #[error(transparent)]
    CreateRedisPool(#[from] deadpool_redis::CreatePoolError),
    #[error(transparent)]
    CreateInstancePool(#[from] deadpool::managed::BuildError),
    #[error(transparent)]
    Resolution(#[from] bulwark_config::ResolutionError),
    #[error("at least one resource required")]
    ResourceMissing,
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
}

/// Returned when an attempt to instantiate a plugin fails.
#[derive(thiserror::Error, Debug)]
pub enum PluginInstantiationError {
    #[error(transparent)]
    WasiError(#[from] wasi_common::Error),
    #[error(transparent)]
    StringArray(#[from] wasi_common::StringArrayError),
    #[error(transparent)]
    ContextInstantiation(#[from] ContextInstantiationError),
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
}

/// Returned when an attempt to instantiate a plugin fails.
#[derive(thiserror::Error, Debug)]
pub enum PluginPoolError {
    #[error(transparent)]
    PluginInstantiation(#[from] PluginInstantiationError),
    #[error(transparent)]
    ContextInstantiation(#[from] ContextInstantiationError),
}

/// Returned when trying to look up a plugin by reference and the plugin reference is missing or invalid.
#[derive(thiserror::Error, Debug)]
pub enum PluginReferenceError {
    #[error("plugin reference is missing: {0}")]
    MissingReference(String),
    #[error("plugin reference is invalid: {0}")]
    InvalidReference(String),
}

/// Returned when an attempt to execute a function within a plugin environment fails.
#[derive(thiserror::Error, Debug)]
pub enum PluginExecutionError {
    #[error(transparent)]
    HandlerError(#[from] crate::bindings::exports::bulwark::plugin::http_handlers::Error),
    #[error(transparent)]
    WasiError(#[from] wasi_common::Error),
    #[error(transparent)]
    StringArray(#[from] wasi_common::StringArrayError),
    #[error("function not implemented '{expected}'")]
    NotImplementedError { expected: String },
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
}

/// Returned when attempting to create a [`PluginCtx`](crate::PluginCtx) fails.
#[derive(thiserror::Error, Debug)]
pub enum ContextInstantiationError {
    #[error(transparent)]
    StringArray(#[from] wasi_common::StringArrayError),
    #[error(transparent)]
    ConfigSerialization(#[from] bulwark_config::ConfigSerializationError),
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
}
