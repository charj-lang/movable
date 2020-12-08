use serde::{Deserialize, Serialize};

use crate::domain::delimiter::DelimiterSymbol;

pub mod delimiter;
pub mod lang_builtin;
pub mod lang_default;
pub mod type_alias;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovableDefine {
    pub language: String,
    pub delimiter: DelimiterSymbol,
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
  ident: 
    Space: 2
";

        let pair: MovableDefine = serde_yaml::from_str(&str).unwrap();
        println!("{:?}", pair);
    }
}
