use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AwsLambdaParams {
    /// Credentials name to use for AWS Lambda
    pub credentials_name: String,
    /// Lambda operation to perform
    pub operation: String,
    /// Function name for Lambda operations
    pub function_name: Option<String>,
    /// Payload to send to Lambda function (JSON)
    pub payload: Option<serde_json::Value>,
    /// Invocation type: RequestResponse, Event, DryRun
    pub invocation_type: Option<String>,
    /// Log type: None, Tail
    pub log_type: Option<String>,
    /// Function version or alias
    pub qualifier: Option<String>,
    /// Runtime for function creation (e.g., python3.12, nodejs20.x)
    pub runtime: Option<String>,
    /// Handler for function (e.g., index.handler, lambda_function.lambda_handler)
    pub handler: Option<String>,
    /// Function code configuration
    pub code: Option<serde_json::Value>,
    /// IAM role ARN for function execution
    pub role: Option<String>,
    /// Function timeout in seconds (1-900)
    pub timeout: Option<u32>,
    /// Memory size in MB (128-10240)
    pub memory_size: Option<u32>,
    /// Environment variables configuration
    pub environment: Option<serde_json::Value>,
    /// Function tags
    pub tags: Option<serde_json::Value>,
}

/// AWS Lambda node - performs AWS Lambda API operations
pub struct AwsLambdaNode;

impl AwsLambdaNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AwsLambdaNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AwsLambdaNode {
    fn type_name(&self) -> &str {
        "aws_lambda"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("aws")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the AWS credentials to use for Lambda",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Lambda operation to perform",
                    "enum": [
                        "invoke",
                        "invoke_async",
                        "create_function",
                        "update_function_code",
                        "update_function_configuration",
                        "delete_function",
                        "get_function",
                        "list_functions",
                        "publish_version",
                        "create_alias",
                        "get_alias",
                        "list_aliases"
                    ]
                },
                "function_name": {
                    "type": "string",
                    "description": "Lambda function name or ARN",
                    "minLength": 1
                },
                "payload": {
                    "description": "Payload to send to Lambda function (JSON)"
                },
                "invocation_type": {
                    "type": "string",
                    "description": "Invocation type for Lambda function",
                    "enum": ["RequestResponse", "Event", "DryRun"],
                    "default": "RequestResponse"
                },
                "log_type": {
                    "type": "string",
                    "description": "Log type to retrieve",
                    "enum": ["None", "Tail"],
                    "default": "None"
                },
                "qualifier": {
                    "type": "string",
                    "description": "Function version or alias to invoke"
                },
                "runtime": {
                    "type": "string",
                    "description": "Lambda runtime environment",
                    "examples": [
                        "python3.12",
                        "python3.11",
                        "nodejs20.x",
                        "nodejs18.x",
                        "java21",
                        "java17",
                        "dotnet8",
                        "go1.x",
                        "ruby3.2",
                        "provided.al2023"
                    ]
                },
                "handler": {
                    "type": "string",
                    "description": "Function handler (e.g., index.handler, lambda_function.lambda_handler)"
                },
                "code": {
                    "description": "Function code configuration (ZipFile, S3Bucket/S3Key, ImageUri)",
                    "type": "object",
                    "properties": {
                        "ZipFile": {
                            "type": "string",
                            "description": "Base64-encoded zip file"
                        },
                        "S3Bucket": {
                            "type": "string",
                            "description": "S3 bucket containing the code"
                        },
                        "S3Key": {
                            "type": "string",
                            "description": "S3 key of the code file"
                        },
                        "S3ObjectVersion": {
                            "type": "string",
                            "description": "S3 object version"
                        },
                        "ImageUri": {
                            "type": "string",
                            "description": "Container image URI"
                        }
                    }
                },
                "role": {
                    "type": "string",
                    "description": "IAM role ARN for function execution"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Function timeout in seconds",
                    "minimum": 1,
                    "maximum": 900,
                    "default": 3
                },
                "memory_size": {
                    "type": "integer",
                    "description": "Memory size in MB",
                    "minimum": 128,
                    "maximum": 10240,
                    "default": 128
                },
                "environment": {
                    "type": "object",
                    "description": "Environment variables configuration",
                    "properties": {
                        "Variables": {
                            "type": "object",
                            "description": "Environment variable key-value pairs"
                        }
                    }
                },
                "tags": {
                    "type": "object",
                    "description": "Tags for the Lambda function"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for AwsLambdaNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AwsLambdaParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get AWS access key, secret key, and region
        // 3. Create an AWS Lambda client using the credentials
        // 4. Execute the Lambda operation based on params.operation:
        //
        //    For invoke operations:
        //    - Build InvokeRequest with function_name, payload, invocation_type, log_type, qualifier
        //    - Call lambda_client.invoke() or invoke_async()
        //    - Parse response including StatusCode, FunctionError, LogResult, Payload
        //
        //    For create_function:
        //    - Build CreateFunctionRequest with function_name, runtime, handler, code, role
        //    - Include optional parameters: timeout, memory_size, environment, tags
        //    - Call lambda_client.create_function()
        //
        //    For update_function_code:
        //    - Build UpdateFunctionCodeRequest with function_name and code
        //    - Call lambda_client.update_function_code()
        //
        //    For update_function_configuration:
        //    - Build UpdateFunctionConfigurationRequest with function_name
        //    - Include parameters: runtime, handler, role, timeout, memory_size, environment
        //    - Call lambda_client.update_function_configuration()
        //
        //    For delete_function:
        //    - Build DeleteFunctionRequest with function_name and optional qualifier
        //    - Call lambda_client.delete_function()
        //
        //    For get_function:
        //    - Build GetFunctionRequest with function_name and optional qualifier
        //    - Call lambda_client.get_function()
        //    - Return function configuration and code location
        //
        //    For list_functions:
        //    - Build ListFunctionsRequest with optional max_items and marker
        //    - Call lambda_client.list_functions()
        //    - Return list of function configurations
        //
        //    For publish_version:
        //    - Build PublishVersionRequest with function_name
        //    - Call lambda_client.publish_version()
        //    - Return new version information
        //
        //    For create_alias/get_alias/list_aliases:
        //    - Build appropriate alias request
        //    - Call corresponding lambda_client method
        //    - Return alias information
        //
        // 5. Handle errors and return appropriate error messages
        // 6. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "AWS Lambda operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "function_name": &params.function_name,
            "invocation_type": &params.invocation_type,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AwsLambdaParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "invoke",
            "invoke_async",
            "create_function",
            "update_function_code",
            "update_function_configuration",
            "delete_function",
            "get_function",
            "list_functions",
            "publish_version",
            "create_alias",
            "get_alias",
            "list_aliases",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate function_name for operations that require it
        let operations_requiring_function_name = [
            "invoke",
            "invoke_async",
            "create_function",
            "update_function_code",
            "update_function_configuration",
            "delete_function",
            "get_function",
            "publish_version",
            "create_alias",
            "get_alias",
            "list_aliases",
        ];

        if operations_requiring_function_name.contains(&params.operation.as_str()) {
            if params.function_name.is_none() {
                anyhow::bail!(
                    "{} operation requires 'function_name' parameter",
                    params.operation
                );
            }
            if let Some(ref name) = params.function_name {
                if name.trim().is_empty() {
                    anyhow::bail!("function_name cannot be empty");
                }
            }
        }

        // Validate create_function requirements
        if params.operation == "create_function" {
            if params.runtime.is_none() {
                anyhow::bail!("create_function operation requires 'runtime' parameter");
            }
            if params.handler.is_none() {
                anyhow::bail!("create_function operation requires 'handler' parameter");
            }
            if params.code.is_none() {
                anyhow::bail!("create_function operation requires 'code' parameter");
            }
            if params.role.is_none() {
                anyhow::bail!("create_function operation requires 'role' parameter");
            }
        }

        // Validate invocation_type values
        if let Some(ref invocation_type) = params.invocation_type {
            let valid_invocation_types = ["RequestResponse", "Event", "DryRun"];
            if !valid_invocation_types.contains(&invocation_type.as_str()) {
                anyhow::bail!(
                    "Invalid invocation_type: {}. Must be one of: {}",
                    invocation_type,
                    valid_invocation_types.join(", ")
                );
            }
        }

        // Validate log_type values
        if let Some(ref log_type) = params.log_type {
            let valid_log_types = ["None", "Tail"];
            if !valid_log_types.contains(&log_type.as_str()) {
                anyhow::bail!(
                    "Invalid log_type: {}. Must be one of: {}",
                    log_type,
                    valid_log_types.join(", ")
                );
            }
        }

        // Validate timeout range
        if let Some(timeout) = params.timeout {
            if !(1..=900).contains(&timeout) {
                anyhow::bail!("timeout must be between 1 and 900 seconds");
            }
        }

        // Validate memory_size range
        if let Some(memory_size) = params.memory_size {
            if !(128..=10240).contains(&memory_size) {
                anyhow::bail!("memory_size must be between 128 and 10240 MB");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lambda_invoke() {
        let node = AwsLambdaNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke",
            "function_name": "my-function",
            "payload": {"key": "value"},
            "invocation_type": "RequestResponse",
            "log_type": "Tail"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "invoke");
        assert_eq!(result.data["function_name"], "my-function");
    }

    #[tokio::test]
    async fn test_lambda_create_function() {
        let node = AwsLambdaNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-new-function",
            "runtime": "python3.12",
            "handler": "lambda_function.lambda_handler",
            "code": {
                "S3Bucket": "my-bucket",
                "S3Key": "lambda.zip"
            },
            "role": "arn:aws:iam::123456789012:role/lambda-role",
            "timeout": 30,
            "memory_size": 256,
            "environment": {
                "Variables": {
                    "ENV": "production"
                }
            }
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_function");
        assert_eq!(result.data["function_name"], "my-new-function");
    }

    #[tokio::test]
    async fn test_lambda_update_function_code() {
        let node = AwsLambdaNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "update_function_code",
            "function_name": "my-function",
            "code": {
                "S3Bucket": "my-bucket",
                "S3Key": "lambda-v2.zip"
            }
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "update_function_code");
    }

    #[tokio::test]
    async fn test_lambda_list_functions() {
        let node = AwsLambdaNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "list_functions"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "list_functions");
    }

    #[tokio::test]
    async fn test_lambda_delete_function() {
        let node = AwsLambdaNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "delete_function",
            "function_name": "my-old-function"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "delete_function");
    }

    #[test]
    fn test_lambda_validation() {
        let node = AwsLambdaNode::new();

        // Valid invoke operation
        let valid_invoke = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke",
            "function_name": "my-function",
            "payload": {"test": "data"}
        });
        assert!(node.validate_parameters(&valid_invoke).is_ok());

        // Valid create_function
        let valid_create = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "handler": "index.handler",
            "code": {"ZipFile": "base64data"},
            "role": "arn:aws:iam::123456789012:role/lambda-role"
        });
        assert!(node.validate_parameters(&valid_create).is_ok());

        // Invalid: invoke without function_name
        let invalid_invoke = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke",
            "payload": {"test": "data"}
        });
        assert!(node.validate_parameters(&invalid_invoke).is_err());

        // Invalid: create_function without runtime
        let invalid_create = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "handler": "index.handler",
            "code": {"ZipFile": "base64data"},
            "role": "arn:aws:iam::123456789012:role/lambda-role"
        });
        assert!(node.validate_parameters(&invalid_create).is_err());

        // Invalid: create_function without handler
        let invalid_handler = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "code": {"ZipFile": "base64data"},
            "role": "arn:aws:iam::123456789012:role/lambda-role"
        });
        assert!(node.validate_parameters(&invalid_handler).is_err());

        // Invalid: create_function without code
        let invalid_code = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "handler": "index.handler",
            "role": "arn:aws:iam::123456789012:role/lambda-role"
        });
        assert!(node.validate_parameters(&invalid_code).is_err());

        // Invalid: create_function without role
        let invalid_role = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "handler": "index.handler",
            "code": {"ZipFile": "base64data"}
        });
        assert!(node.validate_parameters(&invalid_role).is_err());

        // Invalid: invalid invocation_type
        let invalid_invocation_type = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke",
            "function_name": "my-function",
            "invocation_type": "Invalid"
        });
        assert!(node.validate_parameters(&invalid_invocation_type).is_err());

        // Invalid: invalid log_type
        let invalid_log_type = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke",
            "function_name": "my-function",
            "log_type": "Invalid"
        });
        assert!(node.validate_parameters(&invalid_log_type).is_err());

        // Invalid: timeout out of range
        let invalid_timeout = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "handler": "index.handler",
            "code": {"ZipFile": "base64data"},
            "role": "arn:aws:iam::123456789012:role/lambda-role",
            "timeout": 1000
        });
        assert!(node.validate_parameters(&invalid_timeout).is_err());

        // Invalid: memory_size out of range
        let invalid_memory = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "create_function",
            "function_name": "my-function",
            "runtime": "python3.12",
            "handler": "index.handler",
            "code": {"ZipFile": "base64data"},
            "role": "arn:aws:iam::123456789012:role/lambda-role",
            "memory_size": 10241
        });
        assert!(node.validate_parameters(&invalid_memory).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invalid_op"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_lambda_node_type() {
        let node = AwsLambdaNode::new();
        assert_eq!(node.type_name(), "aws_lambda");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::General);
        assert_eq!(node.required_credential_type(), Some("aws"));
    }

    #[test]
    fn test_lambda_parameter_schema() {
        let node = AwsLambdaNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["function_name"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
