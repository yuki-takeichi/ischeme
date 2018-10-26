use data::*;
use data::Object::*;

pub fn eval(ast: Object) -> Object {
    match ast {
        Atom(s) => Atom(s),
        Number(n) => Number(n),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eval_atom() {
        let ast = Object::Atom("hoge".to_string());
        assert_eq!(eval(ast.clone()), ast);
    }
}
