use data::*;
use data::Object::*;

use std::result::Result;

type RuntimeError = &'static str;

pub fn eval(ast: Object, env: Env) -> Result<Object, RuntimeError> {
    match ast {
        Cons(box car, box cdr) => match car { 
            Atom(s) => {
                match env.map.get(&s) {
                    Some(o) if o.is_func() => apply(o, &cdr),
                    _ => Err("TypeError"),
                }
            },
            _ => Err("hoge"),
        },
        _ => Ok(ast),
    }
}

fn get_args(list: &Object) -> Option<Vec<&Box<Object>>> {
    let mut args = Vec::new();
    let mut current = list;
    loop {
        match current {
            Cons(car, cdr) => {
                args.push(car);
                current = cdr;
            },
            Nil => { return Some(args); },
            _ => { return None; },
        }
    }
}

pub fn apply(f: &Object, list: &Object) -> Result<Object, RuntimeError> {
    if let Some(args) = get_args(list) {
        match f {
            Primitive(p) => (p.body)(&args[..]),
            _ => Err("TODO"),
        }
    } else {
        Err("args should be a list")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eval_atom() {
        let ast = Atom("hoge".to_string());
        assert_eq!(eval(ast.clone(), Env::root()), Ok(ast));
    }

    #[test]
    fn eval_number() {
        let ast = Number(777);
        assert_eq!(eval(ast.clone(), Env::root()), Ok(ast));
    }

    fn list3(o1: Object, o2: Object, o3: Object) -> Object {
        Cons(box o1, box Cons(box o2, box Cons(box o3, box Nil)))
    }

    #[test]
    fn apply_one_plus_two() {
        let ast = list3(Atom("+".to_string()),
                        Number(1),
                        Number(2));
        assert_eq!(eval(ast.clone(), Env::root()), Ok(Number(3)));
    }
}
