pub use env_var::{EnvVar, bool_env_var, env_var};
use std::sync::LazyLock;

/// Whether Zed is running in stateless mode.
/// When true, Zed will use in-memory databases instead of persistent storage.
pub static ZED_STATELESS: LazyLock<bool> = bool_env_var!("ZED_STATELESS");

/// Whether a dev build should participate in the single-instance check and
/// listen for CLI connections, like preview/stable builds do.
///
/// Dev builds normally skip this so that multiple dev instances can run side by
/// side. Setting this lets `zed .` from the CLI hand off to an already-running
/// dev instance instead of always opening a new window.
pub static ZED_DEV_SINGLE_INSTANCE: LazyLock<bool> = bool_env_var!("ZED_DEV_SINGLE_INSTANCE");
