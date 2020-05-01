use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process::exit;

fn prompt(prompt: &str) -> String {
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

fn edit_prompt(prompt: &str, value: &str) -> String {
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
