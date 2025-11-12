# PMP Workflow

PMP Workflow: A no-code / low-code platform to create your own workflows. Part of Poor Man's Platform (PMP) ecosystem

A Rust-based workflow automation engine similar to n8n, with YAML-based configuration and PostgreSQL persistence. Execute workflows with extensible node types for various integrations and data processing tasks.

## Features

- **YAML Configuration**: Define workflows using simple YAML files
- **Extensible Node System**: Easy-to-extend node architecture for custom integrations
- **PostgreSQL Persistence**: Store workflows and execution history in PostgreSQL
- **Multiple Trigger Types**:
  - **Manual**: CLI or API-triggered execution
  - **Webhook**: HTTP endpoint triggers
  - **Schedule**: Cron-based scheduling (external scheduler required)
- **Built-in Node Types**:
  - **HTTP Request**: Make HTTP/REST API calls
  - **Transform**: Transform and manipulate data
  - **Conditional**: Branch based on conditions
  - **Set Variable**: Manage workflow variables
- **Webhook Server**: Built-in HTTP server for webhook endpoints
- **Execution Tracking**: Full execution history with detailed logging, including input/output for each node
- **Execution Modes**: Sequential and parallel execution support
- **Timeout Configuration**: Configurable timeouts at workflow and node level
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

# Execution configuration (optional)
execution_mode: sequential  # or "parallel" (default: sequential)
timeout_seconds: 300        # Global timeout in seconds (optional)

nodes:
  - id: unique_node_id
    node_type: start|http_request|transform|conditional|set_variable
    name: Human-readable name
    timeout_seconds: 30     # Node-specific timeout (optional, overrides global)
    parameters:
      # Node-specific parameters

edges:
  - from: source_node_id
    to: target_node_id
    from_output: optional
    to_input: optional
```

### Execution Configuration

#### Execution Modes

- **sequential** (default): Nodes execute one at a time in topological order
- **parallel**: Nodes at the same dependency level can execute concurrently

```yaml
execution_mode: parallel
```

#### Timeout Configuration

Configure timeouts to prevent long-running operations:

- **Workflow-level timeout**: Applies to all nodes by default
- **Node-level timeout**: Overrides workflow timeout for specific nodes

```yaml
# Global timeout for all nodes
timeout_seconds: 300

nodes:
  - id: fast_node
    node_type: http_request
    timeout_seconds: 10  # This node gets 10 seconds
    # ...

  - id: regular_node
    node_type: transform
    # No timeout specified - uses workflow timeout (300s)
    # ...
```

If a node execution exceeds its timeout, the workflow fails with a timeout error.

## Node Types

### Trigger Nodes

Trigger nodes define how a workflow is started. Every workflow should have one trigger node as the entry point.

#### Manual Trigger

Manually execute a workflow via CLI or API with custom input data.

```yaml
- id: manual_trigger
  node_type: manual_trigger
  name: Manual Trigger
  parameters:
    description: "Manually execute this workflow"
    input_schema:  # Optional schema definition
      type: object
      properties:
        order_id:
          type: number
```

Execute manually:
```bash
cargo run -- execute "My Workflow" --input '{"order_id": 123}'
```

#### Webhook Trigger

Trigger a workflow via HTTP webhook endpoint.

```yaml
- id: webhook_trigger
  node_type: webhook_trigger
  name: Webhook Trigger
  parameters:
    method: POST  # GET, POST, PUT, DELETE, PATCH
    description: "Accepts webhook requests"
```

Start the webhook server:
```bash
cargo run -- serve --host 0.0.0.0 --port 3000
```

Trigger the workflow via HTTP:
```bash
curl -X POST http://localhost:3000/api/v1/webhook/{workflow-id}/trigger/{trigger-node-id} \
  -H "Content-Type: application/json" \
  -d '{"user_id": 123, "action": "create"}'
```

#### Schedule Trigger

Trigger a workflow based on a cron schedule. The workflow must be triggered externally by a scheduler.

```yaml
- id: schedule_trigger
  node_type: schedule_trigger
  name: Daily Schedule
  parameters:
    cron: "0 0 0 * * *"  # Daily at midnight (format: sec min hour day month day-of-week)
    timezone: "UTC"
    description: "Runs daily data sync"
```

Cron format: `second minute hour day-of-month month day-of-week`

Common patterns:
- `0 0 0 * * *` - Daily at midnight
- `0 0 */2 * * *` - Every 2 hours
- `0 */5 * * * *` - Every 5 minutes
- `0 0 9-17 * * MON-FRI` - Weekdays 9am-5pm

### Start Node (Legacy)

Entry point for the workflow. Passes through input data. **Note:** Consider using trigger nodes instead.

```yaml
- id: start
  node_type: start
  name: Start
  parameters: {}
```

### Action Nodes

#### HTTP Request Node

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

**Basic Workflows:**
- `simple_workflow.yaml`: Basic workflow demonstrating variables and transforms
- `http_workflow.yaml`: Fetching data from an API
- `conditional_workflow.yaml`: Branching based on conditions
- `complex_workflow.yaml`: Complex multi-step workflow

**Trigger Workflows:**
- `manual_trigger_workflow.yaml`: Manual workflow execution with input data
- `webhook_trigger_workflow.yaml`: Webhook-triggered data processing
- `schedule_trigger_workflow.yaml`: Scheduled data synchronization

**Advanced Examples:**
- `parallel_workflow.yaml`: Parallel execution with timeouts

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
3. Nodes execute according to execution_mode (sequential or parallel)
4. Each node execution is tracked in the database with:
   - Input data received from predecessor nodes
   - Output data produced by the node
   - Start time, end time, and last update time
   - Execution status (Running, Success, Failed)
5. Timeouts are applied at node and workflow level
6. Outputs flow through edges to downstream nodes
7. Final result is stored in the execution record

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
- [x] Webhook triggers for workflows
- [x] Scheduled workflow execution (cron-like configuration)
- [x] Execution mode configuration (sequential/parallel)
- [x] Timeout configuration
- [x] Enhanced execution tracking with input/output
- [ ] True parallel node execution (currently executes sequentially per level)
- [ ] Sub-workflows and workflow composition
- [ ] Error handling and retry logic
- [ ] Variable interpolation in all node parameters
- [ ] Authentication and authorization
- [ ] Workflow versioning

## Related Projects

- [n8n](https://n8n.io/) - Free and open fair-code licensed workflow automation tool
- [Temporal](https://temporal.io/) - Open source durable execution platform
- [Apache Airflow](https://airflow.apache.org/) - Platform to programmatically author, schedule and monitor workflows
