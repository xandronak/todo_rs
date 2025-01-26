use std::env;
use todo_rs::run;

fn main() {
    let mut args = env::args();

    args.next();

    run(args);
}