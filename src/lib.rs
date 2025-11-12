pub mod config;
pub mod db;
pub mod execution;
pub mod models;
pub mod nodes;

pub use execution::*;
pub use models::*;

/// Initialize the node registry with built-in nodes
pub fn create_node_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    nodes::register_builtin_nodes(&mut registry);
    registry
}
