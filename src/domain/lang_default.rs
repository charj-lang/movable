use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LangDefault {
    pub main_func: String,
    /// e.g. in JavaScript or Python or Ruby, can just run without main func.
    pub is_script: bool,
}
