use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process::exit;


/// Creates a CLI prompt from a &str for input and returns a String.
///
/// Example
/// ```
/// use simple_prompts::{prompt};
///
/// pub fn do_this() {
///   let value = prompt("Enter a value: ");
///   println!("{}", value)
/// }
/// ```
pub fn prompt(prompt: &str) -> String {
    let mut editor = Editor::<()>::new();
    let readline = editor.readline(&prompt);
    match readline {
        Ok(line) => {
            line
        },
        Err(ReadlineError::Interrupted) => {
            exit(0)
        },
        Err(ReadlineError::Eof) => {
            exit(1)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1)
        }
    }
}

/// Creates a CLI prompt from a &str for input and prepopulates a value from another &str for
/// potential editing and returns a String.
///
/// Example
/// ```
/// use simple_prompts::{edit_prompt};
///
/// pub fn do_that() {
///   let some_value = "Anything";
///   let value = edit_prompt("Enter a value: ", &some_value);
///   println!("{}", some_value)
/// }
/// ```
pub fn edit_prompt(prompt: &str, value: &str) -> String {
    let mut editor = Editor::<()>::new();
    let readline = editor.readline_with_initial(prompt, (value, ""));
    match readline {
        Ok(line) => {
            line
        },
        Err(ReadlineError::Interrupted) => {
            exit(0)
        },
        Err(ReadlineError::Eof) => {
            exit(1)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1)
        }
    }
}
