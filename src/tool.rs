use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: Value) -> Result<Value, ToolError>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    #[serde(default)]
    pub input: Value,
}

pub fn parse_tool_call(input: &str) -> Result<ToolCall, ToolError> {
    let tool_call: ToolCall =
        serde_json::from_str(input).map_err(ToolError::InvalidToolCallJson)?;

    if tool_call.name.trim().is_empty() {
        return Err(ToolError::EmptyToolName);
    }

    Ok(tool_call)
}

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool call JSON is invalid")]
    InvalidToolCallJson(#[source] serde_json::Error),

    #[error("tool call name cannot be blank")]
    EmptyToolName,

    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),
}

impl PartialEq for ToolError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::InvalidToolCallJson(_), Self::InvalidToolCallJson(_))
                | (Self::EmptyToolName, Self::EmptyToolName)
                | (Self::NotImplemented(_), Self::NotImplemented(_))
        )
    }
}

impl Eq for ToolError {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ToolCall, ToolError, parse_tool_call};

    #[test]
    fn parses_tool_call_json() {
        let tool_call = parse_tool_call(r#"{"name":"schedule_task","input":{"delay":"1h"}}"#)
            .expect("tool call should parse");

        assert_eq!(
            tool_call,
            ToolCall {
                name: "schedule_task".to_string(),
                input: json!({ "delay": "1h" }),
            }
        );
    }

    #[test]
    fn defaults_missing_input_to_null() {
        let tool_call =
            parse_tool_call(r#"{"name":"send_message"}"#).expect("tool call should parse");

        assert_eq!(tool_call.name, "send_message");
        assert_eq!(tool_call.input, serde_json::Value::Null);
    }

    #[test]
    fn rejects_invalid_json() {
        let error = parse_tool_call("not json").expect_err("tool call should fail");

        assert_eq!(
            error,
            ToolError::InvalidToolCallJson(serde_json::Error::io(std::io::Error::other(
                "placeholder"
            )))
        );
    }

    #[test]
    fn rejects_blank_tool_names() {
        let error = parse_tool_call(r#"{"name":"   "}"#).expect_err("tool call should fail");

        assert_eq!(error, ToolError::EmptyToolName);
    }
}
