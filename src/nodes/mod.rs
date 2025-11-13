pub mod bedrock;
pub mod conditional;
pub mod database_query;
pub mod discord;
pub mod dropbox;
pub mod dynamodb;
pub mod elasticsearch;
pub mod execute_workflow;
pub mod ftp;
pub mod gemini;
pub mod github;
pub mod gitlab;
pub mod gmail;
pub mod google_drive;
pub mod http_request;
pub mod jira;
pub mod manual_trigger;
pub mod mongodb;
pub mod mysql;
pub mod openai;
pub mod redis;
pub mod s3;
pub mod schedule_trigger;
pub mod set_variable;
pub mod slack;
pub mod start;
pub mod telegram;
pub mod transform;
pub mod twilio;
pub mod webhook_trigger;

pub use bedrock::BedrockNode;
pub use conditional::ConditionalNode;
pub use database_query::DatabaseQueryNode;
pub use discord::DiscordNode;
pub use dropbox::DropboxNode;
pub use dynamodb::DynamoDBNode;
pub use elasticsearch::ElasticsearchNode;
pub use execute_workflow::ExecuteWorkflowNode;
pub use ftp::FtpNode;
pub use gemini::GeminiNode;
pub use github::GitHubNode;
pub use gitlab::GitLabNode;
pub use gmail::GmailNode;
pub use google_drive::GoogleDriveNode;
pub use http_request::HttpRequestNode;
pub use jira::JiraNode;
pub use manual_trigger::ManualTriggerNode;
pub use mongodb::MongoDBNode;
pub use mysql::MySQLNode;
pub use openai::OpenAINode;
pub use redis::RedisNode;
pub use s3::S3Node;
pub use schedule_trigger::ScheduleTriggerNode;
pub use set_variable::SetVariableNode;
pub use slack::SlackNode;
pub use start::StartNode;
pub use telegram::TelegramNode;
pub use transform::TransformNode;
pub use twilio::TwilioNode;
pub use webhook_trigger::WebhookTriggerNode;

use crate::models::NodeRegistry;
use sqlx::PgPool;

/// Register all built-in node types
pub fn register_builtin_nodes(registry: &mut NodeRegistry, pool: &PgPool) {
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
    registry.register("database_query", || Box::new(DatabaseQueryNode::new()));
    registry.register("redis", || Box::new(RedisNode::new()));
    registry.register("s3", || Box::new(S3Node::new()));

    // AI nodes
    registry.register("openai", || Box::new(OpenAINode::new()));
    registry.register("gemini", || Box::new(GeminiNode::new()));
    registry.register("bedrock", || Box::new(BedrockNode::new()));

    // Communication nodes
    registry.register("slack", || Box::new(SlackNode::new()));
    registry.register("gmail", || Box::new(GmailNode::new()));
    registry.register("telegram", || Box::new(TelegramNode::new()));
    registry.register("twilio", || Box::new(TwilioNode::new()));
    registry.register("discord", || Box::new(DiscordNode::new()));

    // Developer tools
    registry.register("github", || Box::new(GitHubNode::new()));
    registry.register("gitlab", || Box::new(GitLabNode::new()));
    registry.register("jira", || Box::new(JiraNode::new()));

    // Storage/File operations
    registry.register("google_drive", || Box::new(GoogleDriveNode::new()));
    registry.register("dropbox", || Box::new(DropboxNode::new()));
    registry.register("ftp", || Box::new(FtpNode::new()));

    // Database nodes
    registry.register("mysql", || Box::new(MySQLNode::new()));
    registry.register("mongodb", || Box::new(MongoDBNode::new()));
    registry.register("elasticsearch", || Box::new(ElasticsearchNode::new()));
    registry.register("dynamodb", || Box::new(DynamoDBNode::new()));

    // Sub-workflow execution (requires dependencies)
    let pool_clone = pool.clone();
    registry.register("execute_workflow", move || {
        Box::new(ExecuteWorkflowNode::new(pool_clone.clone()))
    });
}
