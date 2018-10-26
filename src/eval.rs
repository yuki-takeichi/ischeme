use data::*;

pub fn eval(ast: Object) -> Object {
    match ast {
        Object::Atom(s) => Object::Atom(s),
        Object::Number(n) => Object::Number(n),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eval_number() {
        let ast = Object::Atom("hoge".to_string());
        assert_eq!(eval(ast.clone()), ast);
    }
}
