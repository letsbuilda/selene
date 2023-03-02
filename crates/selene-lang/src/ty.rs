use std::fmt;

#[expect(dead_code)]
pub enum Type {
    Unit,
    String,
    Int,
    Float,
    Bool,
    List(Box<Self>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::String => write!(f, "String"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::List(_) => write!(f, "List"),
        }
    }
}
