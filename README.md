Simple Prompts
==============
This is a very basic wrapper around the [Rustyline crate](https://github.com/kkawakam/rustyline) that provides just 2 simple and easy to use functions.  The function ``prompt(&str)``, takes a string as a parameter to display as an input prompt and returns the keyboard input (sans the newline).  The function ``edit_prompt(&str, &str)`` takes 2 parameters: the first, is just the prompt string, the second is an existing value you want to edit (be it a default or some other value) and returns any input.

That's it.

If you need more flexibility or features, just drop back to [Rustyline](https://github.com/kkawakam/rustyline).  This library was just designed to save ~50 lines of code and make my source code cleaner on other projects.

Example
-------

*Cargo.toml*
```
[dependencies]
simple_prompts = "0.1.2"
```

*main.rs*
```
use simple_prompts::{prompt, edit_prompt};

fn main() {
    let mut our_input: String = prompt("Enter some text: ");
    our_input = edit_prompt("You entered this, change it: ", &our_input);
    println!("Final value is: {}", our_input)
}
```
