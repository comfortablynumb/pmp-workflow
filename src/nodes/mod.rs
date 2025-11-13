pub mod anthropic;
pub mod assertion;
pub mod audit_trail;
pub mod bedrock;
pub mod circuit_breaker;
pub mod conditional;
pub mod csv_excel;
pub mod database_query;
pub mod datadog;
pub mod delay_node;
pub mod discord;
pub mod dropbox;
pub mod dynamodb;
pub mod elasticsearch;
pub mod execute_workflow;
pub mod file_operations;
pub mod filter;
pub mod flatten;
pub mod ftp;
pub mod gemini;
pub mod github;
pub mod gitlab;
pub mod gmail;
pub mod google_calendar;
pub mod google_drive;
pub mod google_sheets;
pub mod group_by;
pub mod http_request;
pub mod http_webhook;
pub mod image_processor;
pub mod integration_test_runner;
pub mod jira;
pub mod json_xml_converter;
pub mod logging;
pub mod loop_node;
pub mod manual_trigger;
pub mod map;
pub mod merge_node;
pub mod metrics;
pub mod mock_server;
pub mod mongodb;
pub mod mysql;
pub mod openai;
pub mod pagerduty;
pub mod pdf_generator;
pub mod performance_dashboard;
pub mod redis;
pub mod reduce;
pub mod retry;
pub mod s3;
pub mod schedule_trigger;
pub mod sendgrid;
pub mod set_variable;
pub mod slack;
pub mod sort;
pub mod split_node;
pub mod start;
pub mod stripe;
pub mod switch_node;
pub mod telegram;
pub mod test_data_generator;
pub mod timeout;
pub mod tracing;
pub mod transform;
pub mod try_catch;
pub mod twilio;
pub mod wait_webhook;
pub mod webhook_trigger;
pub mod workflow_debugger;
pub mod workflow_runner;
pub mod workflow_template;
pub mod workflow_validator;
pub mod workflow_visualizer;

pub use anthropic::AnthropicNode;
pub use assertion::AssertionNode;
pub use audit_trail::AuditTrailNode;
pub use bedrock::BedrockNode;
pub use circuit_breaker::CircuitBreakerNode;
pub use conditional::ConditionalNode;
pub use csv_excel::CsvExcelNode;
pub use database_query::DatabaseQueryNode;
pub use datadog::DatadogNode;
pub use delay_node::DelayNode;
pub use discord::DiscordNode;
pub use dropbox::DropboxNode;
pub use dynamodb::DynamoDBNode;
pub use elasticsearch::ElasticsearchNode;
pub use execute_workflow::ExecuteWorkflowNode;
pub use file_operations::FileOperationsNode;
pub use filter::FilterNode;
pub use flatten::FlattenNode;
pub use ftp::FtpNode;
pub use gemini::GeminiNode;
pub use github::GitHubNode;
pub use gitlab::GitLabNode;
pub use gmail::GmailNode;
pub use google_calendar::GoogleCalendarNode;
pub use google_drive::GoogleDriveNode;
pub use google_sheets::GoogleSheetsNode;
pub use group_by::GroupByNode;
pub use http_request::HttpRequestNode;
pub use http_webhook::HttpWebhookNode;
pub use image_processor::ImageProcessorNode;
pub use integration_test_runner::IntegrationTestRunnerNode;
pub use jira::JiraNode;
pub use json_xml_converter::JsonXmlConverterNode;
pub use logging::LoggingNode;
pub use loop_node::LoopNode;
pub use manual_trigger::ManualTriggerNode;
pub use map::MapNode;
pub use merge_node::MergeNode;
pub use metrics::MetricsNode;
pub use mock_server::MockServerNode;
pub use mongodb::MongoDBNode;
pub use mysql::MySQLNode;
pub use openai::OpenAINode;
pub use pagerduty::PagerDutyNode;
pub use pdf_generator::PdfGeneratorNode;
pub use performance_dashboard::PerformanceDashboardNode;
pub use redis::RedisNode;
pub use reduce::ReduceNode;
pub use retry::RetryNode;
pub use s3::S3Node;
pub use schedule_trigger::ScheduleTriggerNode;
pub use sendgrid::SendGridNode;
pub use set_variable::SetVariableNode;
pub use slack::SlackNode;
pub use sort::SortNode;
pub use split_node::SplitNode;
pub use start::StartNode;
pub use stripe::StripeNode;
pub use switch_node::SwitchNode;
pub use telegram::TelegramNode;
pub use test_data_generator::TestDataGeneratorNode;
pub use timeout::TimeoutNode;
pub use tracing::TracingNode;
pub use transform::TransformNode;
pub use try_catch::TryCatchNode;
pub use twilio::TwilioNode;
pub use wait_webhook::WaitWebhookNode;
pub use webhook_trigger::WebhookTriggerNode;
pub use workflow_debugger::WorkflowDebuggerNode;
pub use workflow_runner::WorkflowRunnerNode;
pub use workflow_template::WorkflowTemplateNode;
pub use workflow_validator::WorkflowValidatorNode;
pub use workflow_visualizer::WorkflowVisualizerNode;

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

    // Data transformation nodes
    registry.register("filter", || Box::new(FilterNode::new()));
    registry.register("map", || Box::new(MapNode::new()));
    registry.register("reduce", || Box::new(ReduceNode::new()));
    registry.register("sort", || Box::new(SortNode::new()));
    registry.register("group_by", || Box::new(GroupByNode::new()));
    registry.register("flatten", || Box::new(FlattenNode::new()));

    // AI nodes
    registry.register("openai", || Box::new(OpenAINode::new()));
    registry.register("gemini", || Box::new(GeminiNode::new()));
    registry.register("bedrock", || Box::new(BedrockNode::new()));
    registry.register("anthropic", || Box::new(AnthropicNode::new()));

    // Communication nodes
    registry.register("slack", || Box::new(SlackNode::new()));
    registry.register("gmail", || Box::new(GmailNode::new()));
    registry.register("telegram", || Box::new(TelegramNode::new()));
    registry.register("twilio", || Box::new(TwilioNode::new()));
    registry.register("discord", || Box::new(DiscordNode::new()));
    registry.register("sendgrid", || Box::new(SendGridNode::new()));

    // Developer tools
    registry.register("github", || Box::new(GitHubNode::new()));
    registry.register("gitlab", || Box::new(GitLabNode::new()));
    registry.register("jira", || Box::new(JiraNode::new()));

    // Storage/File operations
    registry.register("google_drive", || Box::new(GoogleDriveNode::new()));
    registry.register("dropbox", || Box::new(DropboxNode::new()));
    registry.register("ftp", || Box::new(FtpNode::new()));

    // Google Workspace
    registry.register("google_calendar", || Box::new(GoogleCalendarNode::new()));
    registry.register("google_sheets", || Box::new(GoogleSheetsNode::new()));

    // Database nodes
    registry.register("mysql", || Box::new(MySQLNode::new()));
    registry.register("mongodb", || Box::new(MongoDBNode::new()));
    registry.register("elasticsearch", || Box::new(ElasticsearchNode::new()));
    registry.register("dynamodb", || Box::new(DynamoDBNode::new()));

    // Control flow nodes
    registry.register("loop", || Box::new(LoopNode::new()));
    registry.register("switch", || Box::new(SwitchNode::new()));
    registry.register("merge", || Box::new(MergeNode::new()));
    registry.register("split", || Box::new(SplitNode::new()));
    registry.register("delay", || Box::new(DelayNode::new()));
    registry.register("wait_webhook", || Box::new(WaitWebhookNode::new()));

    // Error handling & resilience nodes
    registry.register("try_catch", || Box::new(TryCatchNode::new()));
    registry.register("retry", || Box::new(RetryNode::new()));
    registry.register("timeout", || Box::new(TimeoutNode::new()));
    registry.register("circuit_breaker", || Box::new(CircuitBreakerNode::new()));

    // Payment & billing
    registry.register("stripe", || Box::new(StripeNode::new()));

    // Monitoring & incident management
    registry.register("pagerduty", || Box::new(PagerDutyNode::new()));
    registry.register("datadog", || Box::new(DatadogNode::new()));

    // Webhooks & HTTP
    registry.register("http_webhook", || Box::new(HttpWebhookNode::new()));

    // File & document processing
    registry.register("file_operations", || Box::new(FileOperationsNode::new()));
    registry.register("pdf_generator", || Box::new(PdfGeneratorNode::new()));
    registry.register("image_processor", || Box::new(ImageProcessorNode::new()));
    registry.register("csv_excel", || Box::new(CsvExcelNode::new()));
    registry.register("json_xml_converter", || {
        Box::new(JsonXmlConverterNode::new())
    });

    // Testing & validation
    registry.register("test_data_generator", || {
        Box::new(TestDataGeneratorNode::new())
    });
    registry.register("workflow_validator", || {
        Box::new(WorkflowValidatorNode::new())
    });
    registry.register("integration_test_runner", || {
        Box::new(IntegrationTestRunnerNode::new())
    });
    registry.register("mock_server", || Box::new(MockServerNode::new()));
    registry.register("assertion", || Box::new(AssertionNode::new()));

    // Monitoring & observability
    registry.register("metrics", || Box::new(MetricsNode::new()));
    registry.register("logging", || Box::new(LoggingNode::new()));
    registry.register("tracing", || Box::new(TracingNode::new()));
    registry.register("audit_trail", || Box::new(AuditTrailNode::new()));
    registry.register("performance_dashboard", || {
        Box::new(PerformanceDashboardNode::new())
    });

    // Developer experience
    registry.register("workflow_runner", || Box::new(WorkflowRunnerNode::new()));
    registry.register("workflow_visualizer", || {
        Box::new(WorkflowVisualizerNode::new())
    });
    registry.register("workflow_debugger", || {
        Box::new(WorkflowDebuggerNode::new())
    });
    registry.register("workflow_template", || {
        Box::new(WorkflowTemplateNode::new())
    });

    // Sub-workflow execution (requires dependencies)
    let pool_clone = pool.clone();
    registry.register("execute_workflow", move || {
        Box::new(ExecuteWorkflowNode::new(pool_clone.clone()))
    });
}
