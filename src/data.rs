use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atom(String),
    Number(i32),
    Closure(String, Option<Box<Env>>),
    Cons(Box<Object>, Box<Object>),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    parent: Option<Box<Env>>,
    map: HashMap<String, Box<Object>>,
}
