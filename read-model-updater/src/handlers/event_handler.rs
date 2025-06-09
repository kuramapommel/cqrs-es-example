use super::HandlerResult;

pub trait EventHandler: Send + Sync {
    fn handle_or_none(&self, payload: &str, manifest: &str) -> Option<HandlerResult>;
}
