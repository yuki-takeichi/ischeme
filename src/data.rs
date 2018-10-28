use std::collections::HashMap;
use std::fmt;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atom(String),
    Number(i32),
    Closure(String, Option<Box<Env>>),
    Primitive(PrimFunc),
    Cons(Box<Object>, Box<Object>),
    Nil,
}

impl Object {
    pub fn is_func(&self) -> bool {
        match self {
            Object::Closure(_, _) => true,
            Object::Primitive(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Object::Cons(_, _) => true,
            Object::Nil => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct PrimFunc {
    name: &'static str,
    body: fn(args: &[Object]) -> Result<Object, &'static str>,
}

impl fmt::Debug for PrimFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl PartialEq for PrimFunc {
    fn eq(&self, other: &PrimFunc) -> bool {
        self.name.eq(other.name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    pub parent: Option<Box<Env>>,
    pub map: HashMap<String, Box<Object>>,
}

fn __plus(args: &[Object]) -> Result<Object, &'static str> {
    let mut sum = 0;
    for arg in args {
        match arg {
            Object::Number(n) => sum += n,
            _ => return Err("TypeError: args should be number"),
        }
    };
    Ok(Object::Number(sum))
}

impl Env {
    pub fn root() -> Env {
        let mut map = HashMap::new();
        let prim = Object::Primitive(PrimFunc {name: "+", body: __plus});
        map.insert("+".to_string(), box prim);
        Env {
            parent: None,
            map: map,
        }
    }
}
