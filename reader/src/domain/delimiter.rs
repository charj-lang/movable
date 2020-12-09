use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DelimiterSymbol {
    /// also `Class` in Java or Python
    pub structs: PairDelimiter,
    /// for **Kotlin** or **JavaScript**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<PairDelimiter>,
    pub parameter: ParameterDelimiter,
    /// e.g. `;` in C, Java
    pub statement: String,
    // todo: thinking in move to editorConfig like
    pub ident: Ident,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Ident {
    Space(i8),
    Tab(String),
}

// todo: change `;` or `end` to LineDelimiter
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LineDelimiter {
    Semicolon,
    Empty,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PairDelimiter {
    pub start: String,
    pub end: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDelimiter {
    /// can be empty in Ruby, but had a `end` symbol
    pub start: String,
    pub end: String,
    /// e.g. `String a, String b` in Java
    pub spacer: String,
    /// e.g. : `String hello` in Java, `path: PathBuf` in Rust
    pub type_spacer: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::delimiter::DelimiterSymbol;

    #[test]
    fn should_convert_obj() {
        let str = "
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
";

        let symbol: DelimiterSymbol = serde_yaml::from_str(&str).unwrap();
        assert_eq!(";", symbol.statement);
    }
}
