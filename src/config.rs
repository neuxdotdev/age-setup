use neuxcfg::Neuxcfg;
use neuxcfg::NeuxcfgError;
use neuxcfg::ProjectConfig;
use toml::Value;

/// The neuxcfg project name used by this crate.
///
/// All configuration is stored under this project identifier in the neuxcfg
/// directory (typically `~/.config/neuxcfg/age-authenticator/`).
pub const PROJECT_NAME: &str = "age-authenticator";

/// Returns a new [`Neuxcfg`] instance using the system configuration directory.
///
/// This is a thin wrapper around [`Neuxcfg::new`] to avoid forcing users to
/// depend directly on `neuxcfg`.
///
/// # Errors
///
/// Returns [`NeuxcfgError::ConfigDirNotFound`] if the system config directory
/// cannot be determined.
pub fn get_neuxcfg() -> Result<Neuxcfg, NeuxcfgError> {
    Neuxcfg::new()
}

/// Initializes the configuration store for this crate.
///
/// Creates the neuxcfg root directory and global config if they do not exist,
/// and ensures the `"age-authenticator"` project is registered.
///
/// Call this once at application startup before using other config functions.
///
/// # Errors
///
/// Returns [`NeuxcfgError`] variants for I/O failures or invalid project names.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::init;
///
/// init()?;
/// println!("Configuration store initialized.");
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn init() -> Result<(), NeuxcfgError> {
    let cfg = get_neuxcfg()?;
    cfg.init()?;
    if !cfg.project_exists(PROJECT_NAME)? {
        cfg.add_project(PROJECT_NAME)?;
    }
    Ok(())
}

/// Loads the complete project configuration from the persistent store.
///
/// # Errors
///
/// Returns [`NeuxcfgError::ProjectNotFound`] if the project has not been
/// initialized. Call [`init`] first.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::{init, load_config};
///
/// init()?;
/// let config = load_config()?;
/// println!("Project path: {}", config.project.path);
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn load_config() -> Result<ProjectConfig, NeuxcfgError> {
    let cfg = get_neuxcfg()?;
    cfg.get_project_config(PROJECT_NAME)
}

/// Overwrites the entire project configuration.
///
/// A backup of the previous configuration is created automatically by neuxcfg.
///
/// # Errors
///
/// Returns [`NeuxcfgError::ProjectNotFound`] if the project has not been
/// initialized, or validation errors for invalid extra fields.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::{init, load_config, save_config};
///
/// init()?;
/// let mut config = load_config()?;
/// config.project.path = "/new/path".into();
/// save_config(&config)?;
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn save_config(config: &ProjectConfig) -> Result<(), NeuxcfgError> {
    let cfg = get_neuxcfg()?;
    cfg.set_project_config(PROJECT_NAME, config)
}

/// Applies a partial update to the project configuration via deep merge.
///
/// Only the provided keys are modified; all other keys remain unchanged.
///
/// # Parameters
///
/// * `delta` – A [`toml::Value`] (usually a table) to merge into the existing config.
///
/// # Errors
///
/// Returns [`NeuxcfgError::ProjectNotFound`] if the project has not been
/// initialized, or validation/TOML parse errors.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::{init, update_config};
/// use toml::toml;
///
/// init()?;
/// let delta = toml! {
///     [project]
///     custom_setting = "enabled"
/// };
/// update_config(toml::Value::Table(delta))?;
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn update_config(delta: toml::Value) -> Result<(), NeuxcfgError> {
    let cfg = get_neuxcfg()?;
    cfg.update_project_config(PROJECT_NAME, delta)
}

/// Sets a single extra field in the project configuration.
///
/// This is a convenience wrapper around [`load_config`] and [`save_config`].
///
/// # Parameters
///
/// * `key` – The extra field name (must not start with `_` or contain `.`).
/// * `value` – A [`toml::Value`] to store.
///
/// # Errors
///
/// Returns [`NeuxcfgError`] variants from loading, validation, or saving.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::{init, set_extra};
/// use toml::Value;
///
/// init()?;
/// set_extra("api_endpoint", Value::String("https://api.example.com".into()))?;
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn set_extra(key: &str, value: Value) -> Result<(), NeuxcfgError> {
    let mut config = load_config()?;
    config.project.extra.insert(key.to_string(), value);
    save_config(&config)
}

/// Retrieves a single extra field from the project configuration.
///
/// Returns `None` if the key does not exist.
///
/// # Parameters
///
/// * `key` – The extra field name to look up.
///
/// # Errors
///
/// Returns [`NeuxcfgError`] variants from loading the configuration.
///
/// # Examples
///
/// ```no_run
/// use age_setup::config::{init, set_extra, get_extra};
/// use toml::Value;
///
/// init()?;
/// set_extra("theme", Value::String("dark".into()))?;
/// let theme = get_extra("theme")?;
/// assert_eq!(theme, Some(Value::String("dark".into())));
/// # Ok::<(), neuxcfg::NeuxcfgError>(())
/// ```
pub fn get_extra(key: &str) -> Result<Option<Value>, NeuxcfgError> {
    let config = load_config()?;
    Ok(config.project.extra.get(key).cloned())
}
