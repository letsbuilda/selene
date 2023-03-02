use num_bigint::BigInt;

#[expect(unused)]
pub enum Value {
    Unit,
    String(String),
    Int(BigInt),
    Float(f64),
    Bool(bool),
    List(Vec<Value>),
}
