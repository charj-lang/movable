use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    TypographyDefinition(Box<TypographyDefinition>),
}

#[derive(Debug, PartialEq)]
pub struct TypographyDefinition {
    pub location: Location,
}

impl TypographyDefinition {}
