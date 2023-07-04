//! Just some types to represent `Address`es and 
//! `Label`s, instead of just using `usize` and `String`

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub usize);
