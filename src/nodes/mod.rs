pub mod start;
pub mod http_request;
pub mod transform;
pub mod conditional;
pub mod set_variable;

pub use start::StartNode;
pub use http_request::HttpRequestNode;
pub use transform::TransformNode;
pub use conditional::ConditionalNode;
pub use set_variable::SetVariableNode;

use crate::models::NodeRegistry;

/// Register all built-in node types
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    registry.register("start", || Box::new(StartNode));
    registry.register("http_request", || Box::new(HttpRequestNode));
    registry.register("transform", || Box::new(TransformNode));
    registry.register("conditional", || Box::new(ConditionalNode));
    registry.register("set_variable", || Box::new(SetVariableNode));
}
