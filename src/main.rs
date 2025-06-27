mod todo;
mod entry;
use std::env::args;
use crate::todo::Todo;

fn main() {
    let args: Vec<String> = args().collect();

    let todo = Todo::new();
    todo.enact(&args);
}
