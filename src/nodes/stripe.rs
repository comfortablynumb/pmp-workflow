use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StripeParams {
    /// Credentials name to use for Stripe API
    pub credentials_name: String,
    /// Stripe operation to perform
    pub operation: String,
    /// Amount in cents (for payment operations)
    pub amount: Option<i64>,
    /// Currency (ISO 4217 code, e.g., "usd", "eur")
    pub currency: Option<String>,
    /// Customer ID
    pub customer_id: Option<String>,
    /// Customer email
    pub customer_email: Option<String>,
    /// Customer name
    pub customer_name: Option<String>,
    /// Payment method ID
    pub payment_method: Option<String>,
    /// Charge/payment intent ID
    pub charge_id: Option<String>,
    /// Payment intent ID
    pub payment_intent_id: Option<String>,
    /// Subscription ID
    pub subscription_id: Option<String>,
    /// Price ID for subscription
    pub price_id: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Stripe node - performs Stripe payment processing operations
pub struct StripeNode;

impl StripeNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StripeNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for StripeNode {
    fn type_name(&self) -> &str {
        "stripe"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Stripe API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Stripe operation to perform",
                    "enum": [
                        "create_charge",
                        "create_payment_intent",
                        "confirm_payment_intent",
                        "capture_payment_intent",
                        "create_refund",
                        "create_customer",
                        "update_customer",
                        "retrieve_customer",
                        "delete_customer",
                        "create_subscription",
                        "update_subscription",
                        "cancel_subscription",
                        "retrieve_subscription",
                        "list_charges",
                        "list_customers",
                        "list_subscriptions"
                    ]
                },
                "amount": {
                    "type": "integer",
                    "description": "Amount in cents (e.g., 1000 for $10.00)",
                    "minimum": 1
                },
                "currency": {
                    "type": "string",
                    "description": "Three-letter ISO currency code (e.g., 'usd', 'eur')",
                    "minLength": 3,
                    "maxLength": 3
                },
                "customer_id": {
                    "type": "string",
                    "description": "Stripe customer ID (e.g., 'cus_...')"
                },
                "customer_email": {
                    "type": "string",
                    "description": "Customer email address",
                    "format": "email"
                },
                "customer_name": {
                    "type": "string",
                    "description": "Customer name"
                },
                "payment_method": {
                    "type": "string",
                    "description": "Payment method ID (e.g., 'pm_...')"
                },
                "charge_id": {
                    "type": "string",
                    "description": "Charge ID for refund operations"
                },
                "payment_intent_id": {
                    "type": "string",
                    "description": "Payment intent ID"
                },
                "subscription_id": {
                    "type": "string",
                    "description": "Subscription ID"
                },
                "price_id": {
                    "type": "string",
                    "description": "Price ID for subscription"
                },
                "description": {
                    "type": "string",
                    "description": "Description of the transaction"
                },
                "metadata": {
                    "type": "object",
                    "description": "Additional metadata for the operation"
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional parameters to pass to the Stripe API"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("stripe_api")
    }
}

#[async_trait]
impl Node for StripeNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: StripeParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the Stripe API key
        // 3. Create a Stripe client using the API key
        // 4. Execute the operation based on params.operation:
        //    - create_charge: Create a charge
        //    - create_payment_intent: Create a payment intent
        //    - confirm_payment_intent: Confirm a payment intent
        //    - capture_payment_intent: Capture a payment intent
        //    - create_refund: Refund a charge
        //    - create_customer: Create a customer
        //    - update_customer: Update customer details
        //    - retrieve_customer: Get customer details
        //    - delete_customer: Delete a customer
        //    - create_subscription: Create a subscription
        //    - update_subscription: Update subscription
        //    - cancel_subscription: Cancel subscription
        //    - retrieve_subscription: Get subscription details
        //    - list_charges: List charges
        //    - list_customers: List customers
        //    - list_subscriptions: List subscriptions
        // 5. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Stripe operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "amount": params.amount,
            "currency": params.currency,
            "customer_id": params.customer_id,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: StripeParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "create_charge",
            "create_payment_intent",
            "confirm_payment_intent",
            "capture_payment_intent",
            "create_refund",
            "create_customer",
            "update_customer",
            "retrieve_customer",
            "delete_customer",
            "create_subscription",
            "update_subscription",
            "cancel_subscription",
            "retrieve_subscription",
            "list_charges",
            "list_customers",
            "list_subscriptions",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate amount and currency for payment operations
        let payment_ops = [
            "create_charge",
            "create_payment_intent",
            "create_subscription",
        ];
        if payment_ops.contains(&params.operation.as_str()) {
            if params.amount.is_none() {
                anyhow::bail!("{} operation requires 'amount' parameter", params.operation);
            }
            if params.currency.is_none() {
                anyhow::bail!(
                    "{} operation requires 'currency' parameter",
                    params.operation
                );
            }
        }

        // Validate customer_id for customer operations
        let customer_ops = ["update_customer", "retrieve_customer", "delete_customer"];
        if customer_ops.contains(&params.operation.as_str()) && params.customer_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'customer_id' parameter",
                params.operation
            );
        }

        // Validate subscription_id for subscription operations
        let subscription_ops = [
            "update_subscription",
            "cancel_subscription",
            "retrieve_subscription",
        ];
        if subscription_ops.contains(&params.operation.as_str()) && params.subscription_id.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'subscription_id' parameter",
                params.operation
            );
        }

        // Validate charge_id for refund
        if params.operation == "create_refund" && params.charge_id.is_none() {
            anyhow::bail!("create_refund operation requires 'charge_id' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_stripe_node_type() {
        let node = StripeNode::new();
        assert_eq!(node.type_name(), "stripe");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert_eq!(node.required_credential_type(), Some("stripe_api"));
    }

    #[test]
    fn test_stripe_parameter_schema() {
        let node = StripeNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
    }

    #[tokio::test]
    async fn test_stripe_create_charge() {
        let node = StripeNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "create_charge",
            "amount": 1000,
            "currency": "usd",
            "description": "Test charge"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_stripe_create_customer() {
        let node = StripeNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "create_customer",
            "customer_email": "test@example.com",
            "customer_name": "Test User"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_stripe_validation() {
        let node = StripeNode::new();

        // Valid create_charge
        let valid_params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "create_charge",
            "amount": 1000,
            "currency": "usd"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing amount for create_charge
        let invalid_params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "create_charge",
            "currency": "usd"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing customer_id for update_customer
        let invalid_params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "update_customer"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "credentials_name": "my_stripe_creds",
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
