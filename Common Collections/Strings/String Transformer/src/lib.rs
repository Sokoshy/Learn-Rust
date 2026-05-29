pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, command) in input {
            match command {
            Command::Uppercase => { output.push(string.to_uppercase()); },
            Command::Trim => { output.push(string.trim().to_string()) }
            Command::Append(n) => {
                let bar= "bar".repeat(n);
                let result = string.to_owned() + &bar;
                output.push(result)
                }
            }
        }
        output
    }
}