//! Permission-based sandbox isolation for WASM dApps.

use std::time::Duration;
use thiserror::Error;
use wasmtime::{Instance, Store};

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("execution timeout exceeded ({0:?})")]
    Timeout(Duration),

    #[error("memory limit exceeded: {used} bytes > {limit} bytes")]
    MemoryLimit { used: usize, limit: usize },

    #[error("permission denied: {0:?}")]
    PermissionDenied(Permission),

    #[error("no WASM instance loaded")]
    NoInstance,

    #[error("runtime error: {0}")]
    Runtime(String),
}

/// Permissions that can be granted to a dApp.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Read wallet addresses.
    ReadWallet,
    /// Sign transactions (dangerous — requires explicit grant).
    SignTransaction,
    /// Read on-chain balances.
    ReadBalance,
    /// Outbound network access (RPC calls).
    Network,
    /// Persistent storage access.
    Storage,
}

/// Sandboxed execution environment for a single dApp.
pub struct Sandbox {
    instance: Option<Instance>,
    permissions: Vec<Permission>,
    memory_limit: usize,
    time_limit: Duration,
}

impl Sandbox {
    /// Create a new sandbox with safe defaults:
    /// - ReadWallet + ReadBalance permissions only
    /// - 32 MB memory limit
    /// - 30 second execution timeout
    pub fn new() -> Self {
        Self {
            instance: None,
            permissions: vec![Permission::ReadWallet, Permission::ReadBalance],
            memory_limit: 32 * 1024 * 1024,
            time_limit: Duration::from_secs(30),
        }
    }

    /// Set the compiled WASM instance.
    pub fn set_instance(&mut self, instance: Instance) {
        self.instance = Some(instance);
    }

    /// Grant an additional permission to this sandbox.
    pub fn grant_permission(&mut self, perm: Permission) {
        if !self.permissions.contains(&perm) {
            self.permissions.push(perm);
        }
    }

    /// Revoke a previously granted permission.
    pub fn revoke_permission(&mut self, perm: &Permission) {
        self.permissions.retain(|p| p != perm);
    }

    /// Check whether a permission is granted.
    pub fn has_permission(&self, perm: &Permission) -> bool {
        self.permissions.contains(perm)
    }

    /// Return all currently granted permissions.
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Override the memory limit (bytes).
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.memory_limit = limit;
        self
    }

    /// Override the execution time limit.
    pub fn with_time_limit(mut self, limit: Duration) -> Self {
        self.time_limit = limit;
        self
    }

    /// Return the configured memory limit.
    pub fn memory_limit(&self) -> usize {
        self.memory_limit
    }

    /// Return the configured time limit.
    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }

    /// Execute the loaded dApp by calling its `_start` entry point.
    pub fn run<T>(&self, store: &mut Store<T>) -> Result<(), SandboxError> {
        let instance = self.instance.as_ref().ok_or(SandboxError::NoInstance)?;

        let start_fn = instance
            .get_typed_func::<(), ()>(&mut *store, "_start")
            .map_err(|e| SandboxError::Runtime(format!("no _start export: {e}")))?;

        start_fn
            .call(store, ())
            .map_err(|e| SandboxError::Runtime(e.to_string()))?;

        Ok(())
    }

    /// Call a named exported function that takes no args and returns an i32.
    pub fn call_i32<T>(
        &self,
        store: &mut Store<T>,
        name: &str,
    ) -> Result<i32, SandboxError> {
        let instance = self.instance.as_ref().ok_or(SandboxError::NoInstance)?;

        let func = instance
            .get_typed_func::<(), i32>(&mut *store, name)
            .map_err(|e| SandboxError::Runtime(format!("export '{name}' not found: {e}")))?;

        func.call(store, ())
            .map_err(|e| SandboxError::Runtime(e.to_string()))
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_permissions() {
        let sb = Sandbox::new();
        assert!(sb.has_permission(&Permission::ReadWallet));
        assert!(sb.has_permission(&Permission::ReadBalance));
        assert!(!sb.has_permission(&Permission::SignTransaction));
        assert!(!sb.has_permission(&Permission::Network));
        assert!(!sb.has_permission(&Permission::Storage));
    }

    #[test]
    fn grant_and_revoke() {
        let mut sb = Sandbox::new();
        sb.grant_permission(Permission::SignTransaction);
        assert!(sb.has_permission(&Permission::SignTransaction));

        sb.revoke_permission(&Permission::SignTransaction);
        assert!(!sb.has_permission(&Permission::SignTransaction));
    }

    #[test]
    fn grant_is_idempotent() {
        let mut sb = Sandbox::new();
        sb.grant_permission(Permission::ReadWallet);
        sb.grant_permission(Permission::ReadWallet);
        assert_eq!(
            sb.permissions()
                .iter()
                .filter(|p| **p == Permission::ReadWallet)
                .count(),
            1
        );
    }

    #[test]
    fn no_instance_returns_error() {
        let sb = Sandbox::new();
        let engine = wasmtime::Engine::default();
        let mut store = Store::new(&engine, ());
        let err = sb.run(&mut store).unwrap_err();
        assert!(matches!(err, SandboxError::NoInstance));
    }

    #[test]
    fn custom_limits() {
        let sb = Sandbox::new()
            .with_memory_limit(64 * 1024 * 1024)
            .with_time_limit(Duration::from_secs(60));
        assert_eq!(sb.memory_limit(), 64 * 1024 * 1024);
        assert_eq!(sb.time_limit(), Duration::from_secs(60));
    }
}
