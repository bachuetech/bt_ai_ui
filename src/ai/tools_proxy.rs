use std::collections::HashMap;

use bt_logger::log_trace;
use bt_math::evaluate_expression;



pub fn tool_proxy(func_name: &String, parameters: &HashMap<String,String>) -> Result<String, String>{
    log_trace!("tool_proxy","Calling {} with {:?}",func_name,parameters);
    match func_name.as_str(){
        "get_current_weather" => Ok("15".to_owned()),
        "do_basic_math" => evaluate_expression(format!("{:?}{:?}{:?}",
                parameters.get("a").unwrap(),parameters.get("op").unwrap(),parameters.get("b").unwrap() ).as_str()).map(|x| x.to_string()),
        "do_math_expressions" => evaluate_expression(parameters.get("expression").unwrap_or(&"".to_owned())).map(|x| x.to_string()),
        _ => Ok("".to_owned())
    }
}