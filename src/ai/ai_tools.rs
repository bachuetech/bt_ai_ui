use std::collections::{HashMap, HashSet};

use bt_logger::{log_debug, log_trace, log_verbose};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::{config::ai_config::SupportedFunctions, utils::file_utils};

pub struct AIToolManager{
    tools: Option<Tools>,
    //json_tools: String,
    //tool_count: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tools{
    tools: Vec<Tool>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tool{
    #[serde(rename = "type")]
    type_: String,  // "function"
    function: Function,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Function{
    name: String,
    description: String,
    parameters: FunctionParameters,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionParameters{
    #[serde(rename = "type")]
    type_: String,
    properties: HashMap<String,ToolParamProperty>,
    required: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolParamProperty{
    #[serde(rename = "type")]
    type_ : String,
    description: String
}

const TOOLS_JSON_DEF: &str = "config/tools-def.json";
const TOOLS_JSON_CONFIG_ENV_VAR_NAME: &str = "TOOLSCONFIGYMLFILE";

impl AIToolManager {
    pub fn new() -> Self {
        let tools_def = file_utils::get_file(TOOLS_JSON_CONFIG_ENV_VAR_NAME, TOOLS_JSON_DEF);

        match AIToolManager::load_tools_from_str(&tools_def) {
            Ok(t) => {
                log_trace!("AIToolManager:new","JSON TOOLS in Struct: {:?}", &t);
                let num_tools = t.tools.len();
                log_verbose!("AIToolManager:new", "Total Tools: {}", num_tools);
                Self{ tools: Some(t),} //json_tools: tools_def, tool_count: num_tools}
            }
            Err(e) => {
                log_debug!("AIToolManager:new", "Error loading tools or No tools available: {}", e) ;
                Self{tools: None, }//json_tools: "".to_owned(), tool_count: 0 }
            }
        }
    }

    fn load_tools_from_str(file_contents: &str) -> Result<Tools> {
            //Deserialize the JSON into the Tools struct
            serde_json::from_str(&file_contents)
    }

    pub fn get_common_tools(&self, functions: SupportedFunctions) ->Option<Vec<Tool>>{
        match functions{
            SupportedFunctions::NONE => None,
            SupportedFunctions::ALL =>  Some(self.tools.clone().unwrap().tools),
            SupportedFunctions::Functions(func) => {  let set2: HashSet<String> = func.into_iter().collect();

                Some(self.tools.clone().unwrap().tools.into_iter().filter(|item| set2.contains(&item.function.name)).collect()) } ,
        }
    }
}

///Tools Returned by AI Model that the application needs to call to return an answer to the AI model.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolToCall{
    function: FunctionToCall
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionToCall{
    name: String,
    arguments: HashMap<String,Value>,
}

impl ToolToCall{
    pub fn get_function_name(&self) -> &String{
        &self.function.name
    }

    pub fn get_arguments(&self) -> HashMap<String,String>{
        //&self.function.arguments
        //ToDo: Need a more elegant solution.
        let mut output:HashMap<String,String> = HashMap::new();

        for (key, value) in &self.function.arguments {
            // Convert each `Value` to a `String` using `to_string()`
            output.insert(key.to_owned(), value.to_string());
        }
    
        output
    }
}