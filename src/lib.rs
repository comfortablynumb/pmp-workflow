pub mod models;
pub mod db;
pub mod nodes;
pub mod execution;
pub mod config;

pub use models::*;
pub use execution::*;

/// Initialize the node registry with built-in nodes
pub fn create_node_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    nodes::register_builtin_nodes(&mut registry);
    registry
}
