use serde::Deserialize;
use cel_interpreter::{Program, Value};
use crate::utils::register_function;

pub mod pocs;

#[derive(Debug, Deserialize, Clone)]
pub struct Poc {
    pub name: String,
    pub description: String,
    pub expression: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub body: Option<String>,
}

impl Poc {
    pub fn compile_and_execute(&self, context: &mut cel_interpreter::context::Context) -> Result<bool, String> {
        let program = Program::compile(&self.expression)
            .map_err(|e| format!("编译CEL表达式失败: {}", e))?;

        register_function(context);
        match program.execute(&context)
            .map_err(|e| format!("执行CEL表达式失败: {}", e))? {
                Value::Bool(b) => Ok(b),
                _ => Err("CEL表达式必须返回布尔值".to_string())
            }
    }
}