pub mod config;
pub mod db;
pub mod execution;
pub mod models;
pub mod nodes;
pub mod server;

pub use execution::*;
pub use models::*;
pub use server::*;

use sqlx::PgPool;

/// Initialize the node registry with built-in nodes
pub fn create_node_registry(pool: &PgPool) -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    nodes::register_builtin_nodes(&mut registry, pool);
    registry
}
