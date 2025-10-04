# Mytodo
This is a todo cli tool inspired by [`sioodmy/todo`](https://github.com/sioodmy/todo)

## Usage
```
Example: cargo run list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: cargo run add "buy carrots"
    - list
        lists all tasks
        Example: cargo run list
    - done [INDEX]
        marks task as done
        Example: cargo run done 2 3 (marks second and third tasks as completed)
    - remove [INDEX]
        removes a task
        Example: cargo run remove 4
    - reset
        deletes all tasks
        Example: cargo run reset
```