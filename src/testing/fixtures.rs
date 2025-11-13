use serde_json::{Value, json};

/// Test data fixtures for common scenarios

/// Sample user data
pub fn user_fixture() -> Value {
    json!({
        "id": "user-123",
        "email": "john.doe@example.com",
        "name": "John Doe",
        "role": "developer",
        "created_at": "2024-01-15T10:30:00Z"
    })
}

/// Sample customer data
pub fn customer_fixture() -> Value {
    json!({
        "id": "cust-456",
        "email": "customer@company.com",
        "company": "Acme Corp",
        "plan": "enterprise",
        "status": "active",
        "metadata": {
            "industry": "technology",
            "size": "500-1000"
        }
    })
}

/// Sample order data
pub fn order_fixture() -> Value {
    json!({
        "id": "order-789",
        "customer_id": "cust-456",
        "items": [
            {
                "sku": "PROD-001",
                "name": "Widget Pro",
                "quantity": 5,
                "price": 99.99
            },
            {
                "sku": "PROD-002",
                "name": "Widget Plus",
                "quantity": 3,
                "price": 149.99
            }
        ],
        "total": 949.92,
        "status": "pending",
        "created_at": "2024-03-20T14:22:00Z"
    })
}

/// Sample incident data
pub fn incident_fixture() -> Value {
    json!({
        "id": "INC-001",
        "title": "Database connection timeout",
        "description": "Users experiencing slow response times",
        "severity": "high",
        "status": "open",
        "affected_services": ["api", "database"],
        "reported_by": "monitoring-system",
        "created_at": "2024-03-20T15:45:00Z"
    })
}

/// Sample GitHub issue data
pub fn github_issue_fixture() -> Value {
    json!({
        "number": 42,
        "title": "Feature request: Add dark mode",
        "body": "It would be great to have a dark mode option.",
        "state": "open",
        "user": {
            "login": "contributor123",
            "id": 98765
        },
        "labels": ["enhancement", "ui"],
        "created_at": "2024-03-19T09:00:00Z"
    })
}

/// Sample Slack message data
pub fn slack_message_fixture() -> Value {
    json!({
        "type": "message",
        "user": "U123456",
        "text": "Hello from Slack!",
        "channel": "C789012",
        "ts": "1234567890.123456"
    })
}

/// Sample email data
pub fn email_fixture() -> Value {
    json!({
        "from": "sender@example.com",
        "to": ["recipient@example.com"],
        "subject": "Test Email Subject",
        "body": "This is a test email body.",
        "html_body": "<p>This is a <strong>test</strong> email body.</p>",
        "attachments": []
    })
}

/// Sample file data
pub fn file_fixture() -> Value {
    json!({
        "id": "file-xyz",
        "name": "document.pdf",
        "path": "/documents/2024/document.pdf",
        "size": 102400,
        "mime_type": "application/pdf",
        "created_at": "2024-03-20T10:00:00Z",
        "modified_at": "2024-03-20T11:30:00Z"
    })
}

/// Sample calendar event data
pub fn calendar_event_fixture() -> Value {
    json!({
        "id": "event-123",
        "summary": "Team Meeting",
        "description": "Weekly team sync",
        "start": {
            "dateTime": "2024-03-25T14:00:00Z",
            "timeZone": "UTC"
        },
        "end": {
            "dateTime": "2024-03-25T15:00:00Z",
            "timeZone": "UTC"
        },
        "attendees": [
            {"email": "alice@example.com", "responseStatus": "accepted"},
            {"email": "bob@example.com", "responseStatus": "tentative"}
        ],
        "status": "confirmed"
    })
}

/// Sample spreadsheet data
pub fn spreadsheet_fixture() -> Value {
    json!({
        "spreadsheet_id": "abc123xyz",
        "title": "Sales Data 2024",
        "sheets": [
            {
                "title": "Q1",
                "data": [
                    ["Date", "Revenue", "Expenses"],
                    ["2024-01-01", "10000", "5000"],
                    ["2024-02-01", "12000", "5500"],
                    ["2024-03-01", "15000", "6000"]
                ]
            }
        ]
    })
}

/// Sample API response data
pub fn api_response_fixture() -> Value {
    json!({
        "status": "success",
        "data": {
            "items": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"},
                {"id": 3, "name": "Item 3"}
            ],
            "total": 3,
            "page": 1
        },
        "metadata": {
            "timestamp": "2024-03-20T12:00:00Z",
            "version": "1.0"
        }
    })
}

/// Sample database query result
pub fn database_result_fixture() -> Value {
    json!({
        "rows": [
            {"id": 1, "name": "Alice", "age": 30},
            {"id": 2, "name": "Bob", "age": 25},
            {"id": 3, "name": "Charlie", "age": 35}
        ],
        "row_count": 3
    })
}

/// Sample webhook payload
pub fn webhook_payload_fixture() -> Value {
    json!({
        "event": "user.created",
        "data": {
            "user_id": "usr_abc123",
            "email": "newuser@example.com",
            "created_at": "2024-03-20T16:00:00Z"
        },
        "delivery_id": "del_xyz789"
    })
}

/// Array of items for testing loop operations
pub fn items_array_fixture() -> Value {
    json!([
        {"id": 1, "name": "Product A", "price": 29.99},
        {"id": 2, "name": "Product B", "price": 39.99},
        {"id": 3, "name": "Product C", "price": 49.99}
    ])
}

/// Error response fixture
pub fn error_response_fixture() -> Value {
    json!({
        "error": {
            "code": "ERR_INVALID_INPUT",
            "message": "The provided input is invalid",
            "details": "Field 'email' is required"
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_are_valid_json() {
        assert!(user_fixture().is_object());
        assert!(customer_fixture().is_object());
        assert!(order_fixture().is_object());
        assert!(incident_fixture().is_object());
        assert!(github_issue_fixture().is_object());
        assert!(slack_message_fixture().is_object());
        assert!(email_fixture().is_object());
        assert!(file_fixture().is_object());
        assert!(calendar_event_fixture().is_object());
        assert!(spreadsheet_fixture().is_object());
        assert!(api_response_fixture().is_object());
        assert!(database_result_fixture().is_object());
        assert!(webhook_payload_fixture().is_object());
        assert!(items_array_fixture().is_array());
        assert!(error_response_fixture().is_object());
    }
}
