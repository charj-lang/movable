use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LangBuiltin {
    pub pkgs: Vec<BuiltinPackage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuiltinPackage {
    pub name: String,
    pub exports: Vec<String>,
}
