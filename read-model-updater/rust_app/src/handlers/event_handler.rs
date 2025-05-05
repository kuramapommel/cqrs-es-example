use std::sync::Arc;

use crate::event_deserializers::deserializer::DomainEvent;

use super::HandlerResult;

pub trait EventHandler: Send + Sync {
    fn handle_or_none(&self, event: Arc<dyn DomainEvent>) -> Option<HandlerResult>;
}
