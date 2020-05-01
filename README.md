Simple Prompts
==============
This is a very basic wrapper around the [Rustyline crate](https://github.com/kkawakam/rustyline) that provides just 2 simple and easy to use functions.  The function ``prompt(String)``, takes a string as a parameter to display as an input prompt and returns the keyboard input (sans the newline).  The function ``edit_prompt(String, String)`` takes 2 parameters: the first, is just the prompt string, the second is an existing value you want to edit (be it a default or some other value) and returns any input.

That's it.

If you need more flexibility or features, just drop back to [Rustyline](https://github.com/kkawakam/rustyline).  This library was just designed to save ~50 lines of code and make my source code cleaner on other projects.
