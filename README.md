# PMP Workflow

PMP Workflow: A no-code / low-code platform to create your own workflows. Part of Poor Man's Platform (PMP) ecosystem

A Rust-based workflow automation engine similar to n8n, with YAML-based configuration and PostgreSQL persistence. Execute workflows with extensible node types for various integrations and data processing tasks.

## Features

- **YAML Configuration**: Define workflows using simple YAML files
- **Extensible Node System**: Easy-to-extend node architecture for custom integrations
- **PostgreSQL Persistence**: Store workflows and execution history in PostgreSQL
- **Built-in Node Types**:
  - **Start**: Entry point for workflows
  - **HTTP Request**: Make HTTP/REST API calls
  - **Transform**: Transform and manipulate data
  - **Conditional**: Branch based on conditions
  - **Set Variable**: Manage workflow variables
- **Execution Tracking**: Full execution history and detailed logging
- **CLI Interface**: Command-line tool for managing and executing workflows

## Prerequisites

- Rust 1.70+ (for building)
- PostgreSQL 14+
- Docker (optional, for running PostgreSQL)

## Installation

### Clone and Build

```bash
git clone https://github.com/yourusername/pmp-workflow.git
cd pmp-workflow
cargo build --release
```

The binary will be available at `target/release/pmp-workflow`.

### Set Up PostgreSQL

#### Using Docker

```bash
docker run -d \
  --name pmp-workflow-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=pmp_workflow \
  -p 5432:5432 \
  postgres:16
```

#### Using Local PostgreSQL

```bash
createdb pmp_workflow
```

### Initialize Database

Set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL="postgres://postgres:postgres@localhost/pmp_workflow"
```

Initialize the database schema:

```bash
cargo run -- --database-url "$DATABASE_URL" init
```

Or using the built binary:

```bash
./target/release/pmp-workflow --database-url "$DATABASE_URL" init
```

## Usage

### Import a Workflow

Import a workflow from a YAML file:

```bash
cargo run -- --database-url "$DATABASE_URL" import --file examples/simple_workflow.yaml
```

### List Workflows

List all workflows:

```bash
cargo run -- --database-url "$DATABASE_URL" list
```

List only active workflows:

```bash
cargo run -- --database-url "$DATABASE_URL" list --active
```

### Execute a Workflow

Execute by workflow name:

```bash
cargo run -- --database-url "$DATABASE_URL" execute "Simple Workflow"
```

Execute with input data:

```bash
cargo run -- --database-url "$DATABASE_URL" execute "Simple Workflow" --input '{"value": 100, "timestamp": "2024-01-01"}'
```

Execute by workflow ID:

```bash
cargo run -- --database-url "$DATABASE_URL" execute "550e8400-e29b-41d4-a716-446655440000"
```

### View Execution History

```bash
cargo run -- --database-url "$DATABASE_URL" history "Simple Workflow" --limit 10
```

### View Execution Details

```bash
cargo run -- --database-url "$DATABASE_URL" show <execution-id>
```

## Workflow Definition Format

Workflows are defined in YAML with the following structure:

```yaml
name: My Workflow
description: Optional description

nodes:
  - id: unique_node_id
    node_type: start|http_request|transform|conditional|set_variable
    name: Human-readable name
    parameters:
      # Node-specific parameters

edges:
  - from: source_node_id
    to: target_node_id
    from_output: optional
    to_input: optional
```

## Node Types

### Start Node

Entry point for the workflow. Passes through input data.

```yaml
- id: start
  node_type: start
  name: Start
  parameters: {}
```

### HTTP Request Node

Makes HTTP requests to external APIs.

```yaml
- id: fetch_data
  node_type: http_request
  name: Fetch User Data
  parameters:
    url: "https://api.example.com/users"
    method: "GET"  # GET, POST, PUT, DELETE, PATCH
    headers:
      Authorization: "Bearer token"
    body:  # Optional, for POST/PUT/PATCH
      key: "value"
```

### Transform Node

Transforms data using expressions or templates.

**Using Expression** (extract field):

```yaml
- id: extract
  node_type: transform
  name: Extract Field
  parameters:
    expression: "body.users[0].name"
```

**Using Template** (create new structure):

```yaml
- id: transform
  node_type: transform
  name: Transform Data
  parameters:
    template:
      user_name: "{{name}}"
      user_email: "{{email}}"
      processed: true
```

**Variable References**:

```yaml
parameters:
  template:
    greeting: "{{$myVariable}}"  # Reference workflow variable
```

### Conditional Node

Evaluates conditions for branching logic.

```yaml
- id: check
  node_type: conditional
  name: Check Value
  parameters:
    field: "value"
    operator: "gt"  # eq, ne, gt, lt, gte, lte, contains
    value: 50
```

Operators:
- `eq`: Equal to
- `ne`: Not equal to
- `gt`: Greater than
- `lt`: Less than
- `gte`: Greater than or equal to
- `lte`: Less than or equal to
- `contains`: String contains (for strings)

### Set Variable Node

Sets workflow variables for use in downstream nodes.

```yaml
- id: set_var
  node_type: set_variable
  name: Set Variable
  parameters:
    name: "myVariable"
    value: "Some value"
    # or reference input field:
    value: "{{input.field}}"
```

## Examples

See the `examples/` directory for complete workflow examples:

- `simple_workflow.yaml`: Basic workflow demonstrating variables and transforms
- `http_workflow.yaml`: Fetching data from an API
- `conditional_workflow.yaml`: Branching based on conditions
- `complex_workflow.yaml`: Complex multi-step workflow

### Example: Simple HTTP API Workflow

```yaml
name: Fetch User Data
description: Fetches user data from JSONPlaceholder API

nodes:
  - id: start
    node_type: start
    name: Start
    parameters: {}

  - id: fetch
    node_type: http_request
    name: Fetch Users
    parameters:
      url: "https://jsonplaceholder.typicode.com/users/1"
      method: "GET"

  - id: extract
    node_type: transform
    name: Extract Name
    parameters:
      expression: "body.name"

edges:
  - from: start
    to: fetch
  - from: fetch
    to: extract
```

Import and run:

```bash
cargo run -- --database-url "$DATABASE_URL" import --file my_workflow.yaml
cargo run -- --database-url "$DATABASE_URL" execute "Fetch User Data"
```

## Extending with Custom Nodes

To create custom node types:

1. Create a new module in `src/nodes/`
2. Implement the `Node` trait:

```rust
use crate::models::{Node, NodeContext, NodeOutput};
use async_trait::async_trait;

pub struct MyCustomNode;

#[async_trait]
impl Node for MyCustomNode {
    fn node_type(&self) -> &str {
        "my_custom_node"
    }

    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Your custom logic here
        Ok(NodeOutput::success(serde_json::json!({"result": "success"})))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        // Validate parameters
        Ok(())
    }
}
```

3. Register in `src/nodes/mod.rs`:

```rust
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    // ... existing registrations
    registry.register("my_custom_node", || Box::new(MyCustomNode));
}
```

## Architecture

### Project Structure

```
pmp-workflow/
├── src/
│   ├── models/          # Data models (Workflow, Node, Execution)
│   ├── db/              # Database layer (PostgreSQL)
│   ├── nodes/           # Node implementations
│   ├── execution/       # Workflow execution engine
│   ├── config/          # YAML configuration loader
│   ├── lib.rs           # Library root
│   └── main.rs          # CLI application
├── migrations/          # SQL migrations
├── examples/            # Example workflows
└── Cargo.toml          # Dependencies
```

### Execution Flow

1. Workflow is loaded from YAML or database
2. Engine performs topological sort to determine execution order
3. Nodes execute sequentially based on dependencies
4. Each node execution is tracked in the database
5. Outputs flow through edges to downstream nodes
6. Final result is stored in the execution record

## Development

### Running Tests

```bash
cargo test
```

### Running with Logging

```bash
RUST_LOG=debug cargo run -- --database-url "$DATABASE_URL" execute "My Workflow"
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Web UI for workflow creation and management
- [ ] More built-in node types (Email, Slack, Database, etc.)
- [ ] Webhook triggers for workflows
- [ ] Scheduled workflow execution (cron-like)
- [ ] Parallel node execution
- [ ] Sub-workflows and workflow composition
- [ ] Error handling and retry logic
- [ ] Variable interpolation in all node parameters
- [ ] Authentication and authorization
- [ ] Workflow versioning

## Related Projects

- [n8n](https://n8n.io/) - Free and open fair-code licensed workflow automation tool
- [Temporal](https://temporal.io/) - Open source durable execution platform
- [Apache Airflow](https://airflow.apache.org/) - Platform to programmatically author, schedule and monitor workflows
