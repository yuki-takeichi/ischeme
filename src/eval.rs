use data::*;
use data::Object::*;

use std::result::Result;

type RuntimeError = &'static str;

pub fn eval(ast: Object) -> Result<Object, RuntimeError> {
    match ast {
        Atom(s) => Ok(Atom(s)),
        Number(n) => Ok(Number(n)),
        Nil => Ok(Nil),
        Closure(s, e) => Ok(Closure(s, e)),
        Cons(box car, box cdr) => match car { 
            Closure(_, _) => apply(&car, &cdr),
            _ => Err("hoge"),
        }
    }
}

pub fn apply(car: &Object, _: &Object) -> Result<Object, RuntimeError> {
    match car {
        Closure(_, _) => Ok(Nil),
        _ => Ok(Nil),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eval_atom() {
        let ast = Atom("hoge".to_string());
        assert_eq!(eval(ast.clone()), Ok(ast));
    }

    #[test]
    fn eval_number() {
        let ast = Number(777);
        assert_eq!(eval(ast.clone()), Ok(ast));
    }

    fn list3(o1: Object, o2: Object, o3: Object) -> Object {
        Cons(box o1, box Cons(box o2, box Cons(box o3, box Nil)))
    }

    #[test]
    fn eval_one_plus_two() {
        let ast = list3(Closure("+".to_string(), None),
                        Number(1),
                        Number(2));
        assert_eq!(eval(ast.clone()), Ok(Number(3)));
    }
}
