use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Request};

pub struct RemoveParams;

impl RemoveParams {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RemoveParams {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hook for RemoveParams {
    async fn on_request(&self, request: Request) -> HookResult {
        let mut request = request;
        request.params = None;
        Ok(HookOutput::new(Message::Request(request)))
    }
}
