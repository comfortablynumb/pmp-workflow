use anyhow::Result;
use clap::{Parser, Subcommand};
use pmp_workflow::{config, create_node_registry, db, WorkflowExecutor};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "pmp-workflow")]
#[command(about = "A workflow automation engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database
    Init,

    /// Import a workflow from a YAML file
    Import {
        /// Path to the workflow YAML file
        #[arg(short, long)]
        file: PathBuf,
    },

    /// List all workflows
    List {
        /// Show only active workflows
        #[arg(short, long)]
        active: bool,
    },

    /// Execute a workflow
    Execute {
        /// Workflow name or ID
        workflow: String,

        /// Input data as JSON
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Show workflow execution history
    History {
        /// Workflow name or ID
        workflow: String,

        /// Number of executions to show
        #[arg(short, long, default_value = "10")]
        limit: i64,
    },

    /// Show execution details
    Show {
        /// Execution ID
        execution_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pmp_workflow=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Create database pool
    let pool = db::create_pool(&cli.database_url).await?;

    match cli.command {
        Commands::Init => {
            tracing::info!("Initializing database...");
            db::run_migrations(&pool).await?;
            println!("✓ Database initialized successfully");
        }

        Commands::Import { file } => {
            tracing::info!("Importing workflow from: {}", file.display());

            // Load workflow definition
            let definition = config::load_workflow_from_file(&file).await?;

            // Import to database
            let workflow = db::import_workflow(&pool, &definition).await?;

            println!("✓ Workflow imported successfully");
            println!("  ID: {}", workflow.id);
            println!("  Name: {}", workflow.name);
        }

        Commands::List { active } => {
            let workflows = db::list_workflows(&pool, active).await?;

            if workflows.is_empty() {
                println!("No workflows found");
                return Ok(());
            }

            println!("Workflows:");
            for workflow in workflows {
                let status = if workflow.active {
                    "active"
                } else {
                    "inactive"
                };
                println!("  {} - {} ({})", workflow.id, workflow.name, status);
                if let Some(desc) = workflow.description {
                    println!("    {}", desc);
                }
            }
        }

        Commands::Execute { workflow, input } => {
            tracing::info!("Executing workflow: {}", workflow);

            // Parse input data
            let input_data = if let Some(input_str) = input {
                Some(serde_json::from_str(&input_str)?)
            } else {
                None
            };

            // Create executor
            let registry = create_node_registry();
            let executor = WorkflowExecutor::new(pool.clone(), registry);

            // Try to parse as UUID first, otherwise treat as name
            let execution = if let Ok(workflow_id) = workflow.parse::<uuid::Uuid>() {
                executor.execute_by_id(workflow_id, input_data).await?
            } else {
                executor.execute_by_name(&workflow, input_data).await?
            };

            println!("✓ Workflow execution completed");
            println!("  Execution ID: {}", execution.id);
            println!("  Status: {}", execution.status);
            println!("  Started: {}", execution.started_at);
            if let Some(finished) = execution.finished_at {
                println!("  Finished: {}", finished);
            }
            if let Some(output) = execution.output_data {
                println!("  Output: {}", serde_json::to_string_pretty(&output)?);
            }
            if let Some(error) = execution.error {
                println!("  Error: {}", error);
            }
        }

        Commands::History { workflow, limit } => {
            // Try to parse as UUID first, otherwise lookup by name
            let workflow_id = if let Ok(id) = workflow.parse::<uuid::Uuid>() {
                id
            } else {
                let wf = db::get_workflow_by_name(&pool, &workflow).await?;
                wf.id
            };

            let executions = db::list_workflow_executions(&pool, workflow_id, Some(limit)).await?;

            if executions.is_empty() {
                println!("No executions found");
                return Ok(());
            }

            println!("Execution history:");
            for execution in executions {
                println!(
                    "  {} - {} ({})",
                    execution.id, execution.status, execution.started_at
                );
                if let Some(error) = execution.error {
                    println!("    Error: {}", error);
                }
            }
        }

        Commands::Show { execution_id } => {
            let execution_uuid = execution_id.parse::<uuid::Uuid>()?;
            let execution = db::get_workflow_execution(&pool, execution_uuid).await?;
            let node_executions = db::list_node_executions(&pool, execution_uuid).await?;

            println!("Execution: {}", execution.id);
            println!("  Workflow ID: {}", execution.workflow_id);
            println!("  Status: {}", execution.status);
            println!("  Started: {}", execution.started_at);
            if let Some(finished) = execution.finished_at {
                println!("  Finished: {}", finished);
            }

            if let Some(input) = execution.input_data {
                println!("  Input: {}", serde_json::to_string_pretty(&input)?);
            }

            if let Some(output) = execution.output_data {
                println!("  Output: {}", serde_json::to_string_pretty(&output)?);
            }

            if let Some(error) = execution.error {
                println!("  Error: {}", error);
            }

            println!("\nNode executions:");
            for node_exec in node_executions {
                println!(
                    "  {} - {} ({})",
                    node_exec.node_id, node_exec.status, node_exec.started_at
                );
                if let Some(error) = node_exec.error {
                    println!("    Error: {}", error);
                }
            }
        }
    }

    Ok(())
}
