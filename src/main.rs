extern crate ischeme;
use ischeme::eval::*;
use ischeme::data::*;

fn main() {
    let o = Object::Number(777);
    eval(o, Env::root()).is_ok();
    println!("done");
}
