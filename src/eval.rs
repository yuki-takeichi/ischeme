use data::*;
use data::Object::*;

use std::result::Result;

type RuntimeError = &'static str;

pub fn eval(ast: Object) -> Result<Object, RuntimeError> {
    match ast {
        Atom(s) => Ok(Atom(s)),
        Number(n) => Ok(Number(n)),
        Nil => Ok(Nil),
        Closure(s) => Ok(Closure(s)),
        Cons(box car, box cdr) => match car { 
            Closure(_) => apply(&car, &cdr),
            _ => Err("hoge"),
        }
    }
}

pub fn apply(car: &Object, _: &Object) -> Result<Object, RuntimeError> {
    match car {
        Closure(_) => Ok(Nil),
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

    #[test]
    fn eval_one_plus_two() {
        let ast = Cons(box Closure("+".to_string()), box Cons(box Number(1), box Cons(box Number(2), box Nil)));
        assert_eq!(eval(ast.clone()), Ok(Number(3)));
    }
}
