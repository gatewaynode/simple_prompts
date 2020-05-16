Simple Prompts
==============
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gatewaynode/simple_prompts/Rust)
![Crates.io](https://img.shields.io/crates/v/simple_prompts)
![Crates.io](https://img.shields.io/crates/l/simple_prompts)


This is a very basic wrapper around the [Rustyline crate](https://github.com/kkawakam/rustyline) that provides just 3 simple and easy to use functions.  The function ``prompt(&str)``, takes a string as a parameter to display as an input prompt and returns the keyboard input (sans the newline).  The function ``edit_prompt(&str, &str)`` takes 2 parameters: the first, is just the prompt string, the second is an existing value you want to edit (be it a default or some other value) and returns any input.  The function ``filter_prompt`` works exactly like prompt but filters HTML control characters as HTML entities (note: this does not protect the string from XSS which can account for the entity encoding).

That's it.

If you need more flexibility or features, just drop back to [Rustyline](https://github.com/kkawakam/rustyline).  This library was just designed to save ~50 lines of code and make my source code cleaner on other projects.

Example
-------

*main.rs*
```
use simple_prompts::{prompt, edit_prompt, filter_prompt};

fn main() {
    let mut our_input: String = prompt("Enter some text: ");
    println!("You entered: {}", our_input);

    our_input = filter_prompt("Now try entering some text to filter like \"  or < ");

    our_input = edit_prompt("You entered was filtered to this, change it: ", &our_input);
    println!("Final value is: {}", our_input)
}
```
