use serde::{Deserialize, Serialize};

use crate::domain::delimiter::DelimiterSymbol;
use crate::domain::lang_default::LangDefault;
use crate::domain::type_alias::TypeAlias;

pub mod delimiter;
pub mod lang_builtin;
pub mod lang_default;
pub mod type_alias;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovableDefine {
    pub language: String,
    /// for symbol in function and parameters
    pub delimiter: DelimiterSymbol,
    /// for some language default config
    pub lang_default: LangDefault,
    /// for type systems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_alias: Option<TypeAlias>,
}

#[cfg(test)]
mod tests {
    use crate::domain::MovableDefine;

    #[test]
    fn should_convert_obj() {
        let str = "
language: charj
delimiter:
  structs:
    start: '{'
    end: '}'
  parameter:
    start: '{'
    end: '}'
    spacer: ''
    type_spacer: ''
  statement: ;
  ident: 
    Space: 2
lang_default:
  main_func: default$main
  is_script: false
";

        let pair: MovableDefine = serde_yaml::from_str(&str).unwrap();
        println!("{:?}", pair);
    }
}
