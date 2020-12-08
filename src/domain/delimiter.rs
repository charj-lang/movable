use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DelimiterSymbol {
    pub structs: PairDelimiter,
    pub parameter: ParameterDelimiter,
    pub ident: Ident,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Ident {
    Space(i8),
    Tab(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PairDelimiter {
    pub start: String,
    pub end: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDelimiter {
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
ident: 
  Space: 2
";

        let pair: DelimiterSymbol = serde_yaml::from_str(&str).unwrap();
        println!("{:?}", pair);
    }
}
