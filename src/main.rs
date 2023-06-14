mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod generics;
mod hashmaps;
mod loops;
mod pointer_ref;
mod print; // https://stackoverflow.com/questions/66915951/rust-use-vs-mod
mod strings;
mod structs;
mod test;
mod tuples;
mod types;
mod vars;
mod vectors;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();
    test::run();
    generics::run();
    hashmaps::run();
    order_food();
}
// ref: https://www.youtube.com/watch?v=zF34dRivLOw&ab_channel=TraversyMedia
