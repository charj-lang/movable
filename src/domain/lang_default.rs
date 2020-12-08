use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LangDefault {
    pub main_func: String,
    pub imports: Vec<String>,
}
