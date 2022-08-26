// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    fn trim_string (x : &String) -> String {
        let out = x.trim();
        out.to_string()
    }

    fn add_bar_n_times (x: &String, n: usize) -> String {
        let mut input = x.to_string();
        for i in 0..n {
            input.push_str("bar");
        }
        input
    }

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    let uppercase = string.to_ascii_uppercase();
                    output.push(uppercase.to_string());
                },
                Command::Trim => {
                    let trimmed_string = trim_string(string);
                    output.push(trimmed_string);
                },
                Command::Append(usize) => {
                    let bar_string = add_bar_n_times(string, *usize);
                    output.push(bar_string);
                }
            };
        }
        println!("{:#?}", output);
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What to we have to import to have `transformer` in scope?
    use my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
