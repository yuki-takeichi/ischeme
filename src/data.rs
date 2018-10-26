#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atom(String),
    Number(i32),
}
