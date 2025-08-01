use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    events::AppEvent,
    state::AppState,
    tools::{Tool, ToolInput},
};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PassToolInputSchema {
    /// Optional reason for passing if you want to explain why you decided to pass.
    pub reason: Option<String>,
}

pub struct PassTool;

impl Tool for PassTool {
    const NAME: &'static str = "pass";

    fn get_tool_input(&self) -> ToolInput {
        ToolInput {
            name: Self::NAME,
            description: "Do nothing. Use this if you want want to do nothing in response to a prompt.",
            input_schema: schema_for!(PassToolInputSchema),
        }
    }

    async fn execute(
        &self,
        _input: &str,
        _state: &AppState,
        _event_sender: mpsc::Sender<AppEvent>,
    ) -> Result<String, anyhow::Error> {
        Ok(String::from("Passed successfully"))
    }
}
