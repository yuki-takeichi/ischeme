#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atom(String),
    Number(i32),
    Closure(String),
    Cons(Box<Object>, Box<Object>),
    Nil,
}
