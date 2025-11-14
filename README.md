# PMP Workflow Engine

**A powerful, Rust-based workflow automation engine** - Part of the Poor Man's Platform (PMP) ecosystem

Build complex workflow automations with 80+ built-in integrations, RBAC security, comprehensive audit logging, and YAML-based configuration. Similar to n8n, with PostgreSQL persistence and enterprise-grade features.

## 🚀 Key Features

### Core Capabilities
- **80+ Built-in Integrations**: AI providers, cloud services, databases, communication tools, and more
- **YAML Configuration**: Define workflows using simple, readable YAML files
- **PostgreSQL Persistence**: Store workflows and execution history with full audit trails
- **Parallel & Sequential Execution**: True async execution with tokio for optimal performance
- **Sub-Workflows**: Compose complex workflows from reusable components
- **Extensible Architecture**: Easy-to-extend node system for custom integrations

### Security & Compliance
- **Role-Based Access Control (RBAC)**: Fine-grained permissions (11 permission types, 4 pre-defined roles)
- **Enhanced Audit Logging**: Track all actions with 27 event types, severity levels, and rich metadata
- **Workflow-Specific ACLs**: Control access at the workflow level
- **Credential Management**: Secure credential storage per integration

### Developer Experience
- **CLI Interface**: Comprehensive command-line tool for all operations
- **Workflow Visualization**: Generate Mermaid, DOT, PlantUML, and ASCII diagrams
- **Interactive Debugger**: Set breakpoints, inspect variables, step through executions
- **Template System**: Reusable workflow templates with variable substitution
- **Test Tools**: Mock servers, test data generators, integration test runners

### Enterprise Features
- **Timeout Configuration**: Workflow and node-level timeouts
- **Error Handling**: Try/catch, retry logic, circuit breakers
- **Control Flow**: Loops, switches, merges, splits, delays
- **Data Transformation**: Filter, map, reduce, sort, group by, flatten
- **Monitoring**: Metrics, logging, tracing, performance dashboards

## 📦 Installation

### Prerequisites
- Rust 1.70+ (for building from source)
- PostgreSQL 14+
- Docker (optional, for PostgreSQL)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/pmp-workflow.git
cd pmp-workflow

# Build the project
cargo build --release

# Set up PostgreSQL with Docker
docker run -d \
  --name pmp-workflow-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=pmp_workflow \
  -p 5432:5432 \
  postgres:16

# Set DATABASE_URL environment variable
export DATABASE_URL="postgres://postgres:postgres@localhost/pmp_workflow"

# Initialize the database
cargo run -- init

# Import an example workflow
cargo run -- import --file examples/simple_workflow.yaml

# Execute the workflow
cargo run -- execute "Simple Workflow"
```

## 🎯 Built-in Node Types (80+)

### Trigger Nodes
Start your workflows with flexible trigger options:

- **Manual Trigger** - CLI or API-triggered execution
- **Webhook Trigger** - HTTP endpoint triggers
- **Schedule Trigger** - Cron-based scheduling

### AI & Machine Learning (7 nodes)
- **OpenAI** - GPT-4, GPT-3.5-Turbo, embeddings, DALL-E
- **Anthropic Claude** - Claude 3.5 Sonnet/Opus/Haiku, function calling
- **Google Gemini** - Gemini Pro/Flash, multimodal AI
- **Amazon Bedrock** - Access to Claude, Llama, Titan models
- **Mistral AI** - Mistral Large/Medium/Small, embeddings
- **Cohere** - Generate, chat, classify, summarize, rerank
- **Hugging Face** - 100k+ models, inference, embeddings

### Cloud Services

#### AWS (6 nodes)
- **AWS S3** - Object storage (multipart uploads, presigned URLs, lifecycle policies)
- **AWS Lambda** - Function invocation, deployment, version management
- **AWS SQS/SNS** - Message queuing and pub/sub (18 operations)
- **AWS DynamoDB** - NoSQL database operations (CRUD, query, scan, batch)
- **AWS CloudWatch** - Metrics, logs, alarms, statistics
- **AWS Secrets Manager** - Secret rotation, versioning, tagging

#### Azure (1 node)
- **Azure Key Vault** - Secrets, keys, and certificates management

### Secret Management (3 nodes)
- **HashiCorp Vault** - Enterprise secret management (11 operations)
- **AWS Secrets Manager** - AWS-native secrets with rotation
- **Azure Key Vault** - Azure-integrated secret storage

### Communication (6 nodes)
- **Slack** - Send messages, create channels, manage users
- **Gmail** - Send emails, read inbox, manage labels
- **SendGrid** - Transactional email delivery
- **Telegram** - Bot integration, send messages
- **Twilio** - SMS, voice calls, WhatsApp
- **Discord** - Bot integration, webhooks

### Databases (5 nodes)
- **PostgreSQL** - SQL queries, transactions
- **MongoDB** - Document operations
- **MySQL** - SQL queries
- **Redis** - Cache operations, pub/sub
- **Elasticsearch** - Search, indexing, aggregations

### Development Tools (3 nodes)
- **GitHub** - Repos, issues, pull requests, actions
- **GitLab** - Projects, merge requests, CI/CD
- **Jira** - Issue tracking, project management

### File & Storage (4 nodes)
- **Google Drive** - File upload/download, sharing
- **Dropbox** - File operations
- **FTP** - File transfer
- **File Operations** - Read/write/convert local files

### Google Workspace (3 nodes)
- **Google Calendar** - Event management
- **Google Sheets** - Spreadsheet operations
- **Gmail** - Email management

### Monitoring & Observability (5 nodes)
- **PagerDuty** - Incident management, alerting
- **Datadog** - Metrics, monitoring, alerts
- **Metrics Node** - Custom metrics emission
- **Logging Node** - Structured logging
- **Tracing Node** - OpenTelemetry integration

### Payment & Billing (1 node)
- **Stripe** - Payments, subscriptions, customers (16 operations)

### Data Processing (11 nodes)
- **Transform** - Data manipulation with templates
- **Filter** - Filter arrays by conditions
- **Map** - Transform array elements
- **Reduce** - Aggregate data
- **Sort** - Sort arrays
- **Group By** - Group data by key
- **Flatten** - Flatten nested arrays
- **CSV/Excel Parser** - Parse and generate spreadsheets
- **JSON/XML Converter** - Convert between formats
- **PDF Generator** - Create PDFs from templates
- **Image Processor** - Resize, crop, optimize images

### Control Flow (6 nodes)
- **Conditional** - Branch based on conditions
- **Loop** - Iterate over arrays
- **Switch** - Multi-way branching
- **Merge** - Combine multiple inputs
- **Split** - Split data into multiple outputs
- **Delay** - Wait for specified duration
- **Wait Webhook** - Pause until webhook received

### Error Handling & Resilience (4 nodes)
- **Try/Catch** - Exception handling
- **Retry** - Automatic retries with backoff
- **Timeout** - Operation timeouts
- **Circuit Breaker** - Fault tolerance

### HTTP & Webhooks (2 nodes)
- **HTTP Request** - Make REST API calls
- **HTTP Webhook Sender** - Trigger external webhooks

### Testing & Validation (5 nodes)
- **Test Data Generator** - Generate realistic test data
- **Workflow Validator** - Validate workflow YAML
- **Integration Test Runner** - End-to-end testing
- **Mock Server** - Simulate external APIs
- **Assertion Node** - Validate data in workflows

### Developer Experience (4 nodes)
- **Workflow Runner** - Execute workflows programmatically
- **Workflow Visualizer** - Generate diagrams (Mermaid, DOT, PlantUML, ASCII)
- **Workflow Debugger** - Interactive debugging with breakpoints
- **Workflow Template** - Template management and generation

### Workflow Management (2 nodes)
- **Execute Workflow** - Run sub-workflows
- **Set Variable** - Manage workflow variables

## 🔐 Role-Based Access Control (RBAC)

### Permission Types (11)
- `view_workflow` - View workflow definitions
- `create_workflow` - Create new workflows
- `edit_workflow` - Modify workflows
- `delete_workflow` - Delete workflows
- `execute_workflow` - Run workflows
- `cancel_execution` - Cancel running executions
- `view_execution` - View execution results
- `manage_credentials` - Manage credentials
- `manage_roles` - Manage user roles
- `view_audit` - View audit logs
- `manage_system` - System administration

### Pre-defined Roles

**Admin**
- All permissions
- Full system control

**Developer**
- Create, edit, and execute workflows
- View executions
- Limited administrative access

**Operator**
- Execute and cancel workflows
- View workflows and executions
- No modification permissions

**Viewer**
- Read-only access
- View workflows and executions

### Workflow-Specific ACLs

Grant permissions at the workflow level:

```yaml
# Example: Grant user specific permissions for one workflow
acl:
  - user_id: "user@example.com"
    permissions:
      - execute_workflow
      - view_execution
```

## 📊 Enhanced Audit Logging

### Audit Event Types (27)

**Workflow Events:**
- `workflow_created`, `workflow_updated`, `workflow_deleted`, `workflow_executed`

**Execution Events:**
- `execution_started`, `execution_completed`, `execution_failed`, `execution_cancelled`
- `node_execution_started`, `node_execution_completed`, `node_execution_failed`

**Credential Events:**
- `credential_created`, `credential_updated`, `credential_deleted`, `credential_accessed`

**RBAC Events:**
- `role_created`, `role_updated`, `role_deleted`, `role_assigned`, `role_revoked`
- `permission_granted`, `permission_revoked`

**System Events:**
- `system_config_changed`, `user_login`, `user_logout`, `unauthorized_access`

### Severity Levels
- **Info** - Normal operations
- **Warning** - Important events
- **Error** - Error conditions
- **Critical** - Critical failures

### Audit Metadata
Track comprehensive details:
- User ID, IP address, user agent
- Resource type and ID
- Workflow and execution IDs
- Duration in milliseconds
- Error messages
- Custom metadata (JSON)

## 📝 Workflow Definition Format

### Basic Structure

```yaml
name: My Workflow
description: Optional description

# Execution configuration
execution_mode: sequential  # or "parallel"
timeout_seconds: 300

nodes:
  - id: trigger
    node_type: manual_trigger
    name: Start Workflow
    parameters:
      description: "Manually start this workflow"

  - id: process
    node_type: transform
    name: Process Data
    timeout_seconds: 30
    parameters:
      template:
        result: "{{input.value * 2}}"

edges:
  - from: trigger
    to: process
```

### Execution Modes

**Sequential** (default)
```yaml
execution_mode: sequential
```
Nodes execute one at a time in topological order.

**Parallel**
```yaml
execution_mode: parallel
```
Independent nodes execute concurrently using tokio tasks for better performance.

### Timeout Configuration

**Workflow-level timeout** (applies to all nodes):
```yaml
timeout_seconds: 300
```

**Node-level timeout** (overrides workflow timeout):
```yaml
nodes:
  - id: fast_operation
    node_type: http_request
    timeout_seconds: 10  # This node gets 10 seconds
    parameters:
      url: "https://api.example.com/quick"
```

## 🎨 Example Workflows

### Example 1: AI-Powered Content Generation

```yaml
name: AI Content Generator
description: Generate blog posts with AI

nodes:
  - id: trigger
    node_type: manual_trigger
    name: Manual Trigger
    parameters:
      description: "Start content generation"

  - id: generate
    node_type: anthropic
    name: Generate Content
    parameters:
      operation: messages
      model: claude-3-5-sonnet-20241022
      messages:
        - role: user
          content: "Write a blog post about {{input.topic}}"
      max_tokens: 2000

  - id: save
    node_type: file_operations
    name: Save to File
    parameters:
      operation: write_file
      path: "output/{{input.topic}}.md"
      content: "{{generate.content}}"

edges:
  - from: trigger
    to: generate
  - from: generate
    to: save
```

### Example 2: E-commerce Order Processing

See `examples/ecommerce_order_processing.yaml` for a complete 22-node workflow including:
- Fraud detection with circuit breaker
- Stripe payment processing with retry logic
- Inventory management
- Parallel fulfillment (packing, invoice, shipping label)
- Email notifications
- Metrics tracking

### Example 3: Data Pipeline ETL

See `examples/data_pipeline_etl.yaml` for a complete 30-node workflow including:
- Parallel extraction from PostgreSQL, MySQL, MongoDB, S3
- Data transformation (filter, map, group, sort)
- Quality checks and validation
- Staging area loading
- Production merge with upsert
- S3 archival

### Example 4: Lead Nurturing Campaign

See `examples/lead_nurturing_campaign.yaml` for automation including:
- CRM integration for lead enrichment
- Lead scoring based on engagement
- Routing (hot/warm/cold leads)
- 7-email nurture sequence over 30 days
- Engagement tracking
- Sales qualification

## 🔧 CLI Usage

### Environment Setup

```bash
# Required: Set DATABASE_URL
export DATABASE_URL="postgres://postgres:postgres@localhost/pmp_workflow"
```

### Database Management

```bash
# Initialize database schema
./pmp-workflow init

# Run migrations
./pmp-workflow migrate
```

### Workflow Management

```bash
# Import workflow from YAML
./pmp-workflow import --file workflow.yaml

# List all workflows
./pmp-workflow list

# List only active workflows
./pmp-workflow list --active

# Delete a workflow
./pmp-workflow delete "Workflow Name"
```

### Execution

```bash
# Execute by name
./pmp-workflow execute "My Workflow"

# Execute with input data
./pmp-workflow execute "My Workflow" --input '{"key": "value"}'

# Execute by ID
./pmp-workflow execute "550e8400-e29b-41d4-a716-446655440000"
```

### Viewing Results

```bash
# View execution history
./pmp-workflow history "My Workflow" --limit 10

# View detailed execution
./pmp-workflow show <execution-id>

# View workflow definition
./pmp-workflow get "My Workflow"
```

### Webhook Server

```bash
# Start webhook server
./pmp-workflow serve --host 0.0.0.0 --port 3000

# Trigger via HTTP
curl -X POST http://localhost:3000/api/v1/webhook/{workflow-id}/trigger/{trigger-id} \
  -H "Content-Type: application/json" \
  -d '{"data": "value"}'
```

### Credentials Management

```bash
# Add credentials
./pmp-workflow credentials add \
  --name "my-api-key" \
  --type "api_key" \
  --value '{"api_key": "secret"}'

# List credentials
./pmp-workflow credentials list

# Delete credentials
./pmp-workflow credentials delete "my-api-key"
```

## 🏗️ Architecture

### Project Structure

```
pmp-workflow/
├── src/
│   ├── models/          # Data models
│   │   ├── workflow.rs  # Workflow definition
│   │   ├── execution.rs # Execution tracking
│   │   ├── rbac.rs      # Role-based access control
│   │   ├── audit.rs     # Audit logging
│   │   └── node.rs      # Node trait definitions
│   ├── nodes/           # 80+ node implementations
│   ├── execution/       # Workflow execution engine
│   ├── db/              # PostgreSQL layer
│   ├── config/          # YAML configuration
│   └── server/          # Webhook HTTP server
├── examples/            # Example workflows
├── migrations/          # Database migrations
└── README.md           # This file
```

### Execution Flow

1. **Workflow Loading** - Load from YAML or database
2. **Topological Sort** - Determine execution order
3. **Mode Selection** - Sequential or parallel execution
4. **Node Execution** - Execute with timeout and error handling
5. **Output Propagation** - Pass data through edges
6. **Audit Logging** - Track all actions and events
7. **Result Storage** - Store execution results

### Database Schema

Key tables:
- `workflows` - Workflow definitions
- `workflow_executions` - Execution records
- `node_executions` - Individual node execution details
- `credentials` - Secure credential storage
- `roles` - RBAC role definitions
- `user_roles` - User-role assignments
- `workflow_acls` - Workflow-specific permissions
- `audit_logs` - Comprehensive audit trail

## 🛠️ Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Check for security issues
cargo audit
```

### Adding Custom Nodes

1. Create new file in `src/nodes/`

```rust
use crate::models::{Node, NodeContext, NodeOutput, NodeType, NodeCategory, NodeSubcategory};
use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;

pub struct MyCustomNode;

impl Default for MyCustomNode {
    fn default() -> Self {
        Self::new()
    }
}

impl MyCustomNode {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for MyCustomNode {
    fn type_name(&self) -> &str {
        "my_custom_node"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }
}

#[async_trait]
impl Node for MyCustomNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &Value,
    ) -> Result<NodeOutput> {
        // Your implementation here
        Ok(NodeOutput::success(serde_json::json!({
            "status": "success"
        })))
    }

    fn validate_parameters(&self, parameters: &Value) -> Result<()> {
        // Validate parameters
        Ok(())
    }
}
```

2. Register in `src/nodes/mod.rs`:

```rust
pub mod my_custom_node;
pub use my_custom_node::MyCustomNode;

pub fn register_builtin_nodes(registry: &mut NodeRegistry, pool: &PgPool) {
    // ... existing registrations
    registry.register("my_custom_node", || Box::new(MyCustomNode::new()));
}
```

3. Add tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use serde_json::json;

    #[tokio::test]
    async fn test_my_custom_node() {
        let node = MyCustomNode::new();
        let params = json!({});
        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test".to_string()
        );

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["status"], "success");
    }
}
```

## 📚 Additional Resources

### Example Workflows

The `examples/` directory contains comprehensive examples:

**Basic Examples:**
- `simple_workflow.yaml` - Basic workflow with variables
- `http_workflow.yaml` - API integration
- `conditional_workflow.yaml` - Branching logic
- `parallel_workflow.yaml` - Parallel execution

**Advanced Examples:**
- `ecommerce_order_processing.yaml` - 22-node e-commerce workflow
- `lead_nurturing_campaign.yaml` - Marketing automation
- `data_pipeline_etl.yaml` - 30-node data pipeline
- `approval_chain.yaml` - Multi-level approval workflow
- `ab_testing_framework.yaml` - A/B testing infrastructure

### API Documentation

Full API documentation is available in the code:
- See trait definitions in `src/models/node.rs`
- Each node type includes comprehensive examples
- Parameter schemas are defined in each node

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🗺️ Roadmap

### Completed
- [x] 80+ built-in node types
- [x] RBAC with 11 permissions
- [x] Enhanced audit logging (27 event types)
- [x] Parallel execution with tokio
- [x] Timeout configuration
- [x] Error handling (try/catch, retry, circuit breaker)
- [x] Data transformation nodes
- [x] Developer tools (visualizer, debugger, templates)
- [x] Webhook triggers
- [x] Schedule triggers
- [x] Sub-workflows

### In Progress
- [ ] Web UI for workflow creation
- [ ] Real-time execution monitoring
- [ ] Workflow versioning
- [ ] Multi-tenancy support

### Planned
- [ ] Workflow marketplace
- [ ] Cloud deployment options
- [ ] Mobile app for monitoring
- [ ] Advanced analytics dashboard
- [ ] Git integration for workflows
- [ ] CI/CD integration

## 🙏 Acknowledgments

Inspired by:
- [n8n](https://n8n.io/) - Workflow automation
- [Temporal](https://temporal.io/) - Durable execution
- [Apache Airflow](https://airflow.apache.org/) - Workflow orchestration

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/pmp-workflow/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/pmp-workflow/discussions)
- **Documentation**: [Wiki](https://github.com/yourusername/pmp-workflow/wiki)

---

**Built with ❤️ in Rust**
