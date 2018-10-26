extern crate ischeme;
use ischeme::eval::*;

fn main() {
    let o = Object::Number(777);
    eval(o);
    println!("done");
}
