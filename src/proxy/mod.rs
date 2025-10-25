mod transport;

pub use transport::LspTransport;

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

/// Hook function type for modifying requests/responses
pub type HookFn =
    Box<dyn Fn(&mut Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

/// Represents a hook that can modify LSP messages
pub struct Hook {
    /// The LSP method this hook should be applied to
    method: String,
    /// The function to execute when the method matches
    handler: HookFn,
}

impl Hook {
    pub fn new<S: Into<String>>(method: S, handler: HookFn) -> Self {
        Self {
            method: method.into(),
            handler,
        }
    }
}

/// LSP proxy that can intercept and modify messages
pub struct LspProxy {
    request_hooks: Vec<Hook>,
    response_hooks: Vec<Hook>,
    /// Maps request IDs to their methods for response correlation
    request_methods: Mutex<HashMap<Value, String>>,
}

impl Default for LspProxy {
    fn default() -> Self {
        Self::new()
    }
}

impl LspProxy {
    pub fn new() -> Self {
        Self {
            request_hooks: Vec::new(),
            response_hooks: Vec::new(),
            request_methods: Mutex::new(HashMap::new()),
        }
    }

    /// Add a hook for intercepting and modifying requests
    pub fn add_request_hook(&mut self, hook: Hook) {
        self.request_hooks.push(hook);
    }

    /// Add a hook for intercepting and modifying responses
    pub fn add_response_hook(&mut self, hook: Hook) {
        self.response_hooks.push(hook);
    }

    /// Process an incoming request, applying any matching hooks
    pub async fn process_request(&self, mut request: Value) -> Result<Value> {
        let method_str = request
            .get("method")
            .and_then(Value::as_str)
            .map(String::from);

        if let (Some(id), Some(method)) = (request.get("id").cloned(), &method_str) {
            let mut request_methods = self.request_methods.lock().unwrap();
            request_methods.insert(id, method.clone());
        }

        // Apply request hooks if method matches
        if let Some(method) = method_str {
            for hook in &self.request_hooks {
                if hook.method == method {
                    (hook.handler)(&mut request).await?;
                }
            }
        }

        Ok(request)
    }

    /// Process an outgoing response, applying any matching hooks
    pub async fn process_response(&self, mut response: Value) -> Result<Value> {
        // Look up the method from the stored request ID
        let method = if let Some(id) = response.get("id") {
            let mut request_methods = self.request_methods.lock().unwrap();
            request_methods.remove(id)
        } else {
            None
        };

        // Apply response hooks if we found a matching method
        if let Some(method) = method {
            for hook in &self.response_hooks {
                if hook.method == method {
                    (hook.handler)(&mut response).await?;
                }
            }
        }

        Ok(response)
    }
}
