use serde::{Deserialize, Serialize};

use super::ai_tools::ToolToCall;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole{
    USER,
    ASSISTANT,
    SYSTEM,
    TOOL,
    ERROR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    role: MessageRole,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolToCall>>,
}

impl Message{
    pub fn new(role: MessageRole,  msg_content: String) -> Self{
        Message{
            role: role,
            content: msg_content,
            tool_calls: None,
        }
    }

    /*pub fn new_from_json(json_message: String) -> Self{
        serde_json::from_str(&json_message).unwrap()
    }*/

    pub fn get_content(&self) -> &String{
        &self.content
    }

    pub fn get_role(&self) -> &MessageRole{
        &self.role
    }

    /*pub fn get_message_json(&self) -> String{
         serde_json::to_string(self).unwrap()
    }*/

    pub fn get_tools(&self) -> Option<Vec<ToolToCall>> {
        self.tool_calls.clone()
    }
}