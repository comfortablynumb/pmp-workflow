pub mod conditional;
pub mod http_request;
pub mod manual_trigger;
pub mod schedule_trigger;
pub mod set_variable;
pub mod start;
pub mod transform;
pub mod webhook_trigger;

pub use conditional::ConditionalNode;
pub use http_request::HttpRequestNode;
pub use manual_trigger::ManualTriggerNode;
pub use schedule_trigger::ScheduleTriggerNode;
pub use set_variable::SetVariableNode;
pub use start::StartNode;
pub use transform::TransformNode;
pub use webhook_trigger::WebhookTriggerNode;

use crate::models::NodeRegistry;

/// Register all built-in node types
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    // Legacy start node
    registry.register("start", || Box::new(StartNode));

    // Trigger nodes
    registry.register("manual_trigger", || Box::new(ManualTriggerNode));
    registry.register("webhook_trigger", || Box::new(WebhookTriggerNode));
    registry.register("schedule_trigger", || Box::new(ScheduleTriggerNode));

    // Action nodes
    registry.register("http_request", || Box::new(HttpRequestNode));
    registry.register("transform", || Box::new(TransformNode));
    registry.register("conditional", || Box::new(ConditionalNode));
    registry.register("set_variable", || Box::new(SetVariableNode));
}
