mod todo;
mod entry;
use std::env::args;
use crate::todo::Todo;

fn main() {
    let this_args: Vec<String> = args().collect();

    let mut todo = Todo::new();
    let print_lines = todo.enact(&this_args);
    println!("{}", print_lines);
}
