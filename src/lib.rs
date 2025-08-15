//! simple_arg_parser: Created by Lincoln Reilly.
//! A simple command-line argument parser for small use cases.
//! For more complex argument parsing, `clap` is highly recommended.

use std::collections::HashMap;
use std::env;

/// Argument (`Enum`): Basic identifier for the different types of arguments a program can provide.
/// Includes Positional, Flag, Option and Variable types. These also include their respective data.
/// Mainly used as an identifier in the returned vector from the `parse_arguments()` function.
/// 
/// # Examples
/// 
/// ```
/// let positional_argument = Argument::Positional("file.txt");     // Positional argument
/// let flag = Argument::Flag('o');                                 // Flag - notice the single character
/// let option = Argument::Option("quiet");                         // Option - notice the full name
/// let variable = Argument::Variable {                             // Variable - contains a name and value, both `String`s.
///     name: String::from("ignore-warnings"),                      // ...
///     value: String::from("true")                                 // ...
/// }
/// ```
#[derive(Debug)]
pub enum Argument {
    /// A positional argument - the most standard type, e.g. `myprogram file.txt` has the positional argument 'file.txt'.
    /// Takes a `String` - the name of the positional argument.
    Positional(String),
    /// A flag - given after a single '-' symbol, can be grouped together. e.g. `myprogram -fo file.txt` has the flags 'f' and 'o'. Seperated for clarity in the Argument enum.
    /// Takes a `char`: the character as the flag.
    Flag(char),
    /// An option - expanded out version of a flag given after two '-' symbols, and does not include an '=' sign. e.g. `myprogram file.txt --quiet` has the option 'quiet'.
    /// Takes a `String`: the name of the option.
    Option(String),
    /// A variable: flag and a value given by an '=' sign. e.g. `myprogram file.txt --output-type=quiet` has the variable '--output-type' set to 'quiet'.
    /// Takes a `name: String` and a `value: String` - The name and value of the variable.
    Variable { name: String, value: String },
}

/// ParsedArguments (`struct`): Ordered arguments structure for developer access.
/// Provides a higher level access to arguments of a program, including the arguments as a `Vec<Argument>`, the positionals, flags, options, and variables.
/// Returned by the `parse_arguments()` function.
/// 
/// # Examples
/// 
/// ```
/// let testing_arguments: Vec<String> = String::from("file.txt -o file.o --quiet --on-warnings=exit")  // Example arguments
///     .split_whitespace()                                                                             // ...
///     .map(|s| s.to_string())                                                                         // ...
///     .collect();                                                                                     // ...
/// let parsed_arguments: ParsedArguments = parse_arguments();                                          // Parse the arguments, returning a ParsedArguments struct.
/// println!(parsed_arguments.positionals)                                                              // Prints `["file.txt", "file.o"]`
/// println!(parsed_arguments.flags)                                                                    // Prints `[-o']`
/// println!(parsed_arguments.options)                                                                  // Prints `["quiet"]`
/// println!(parsed_arguments.variables)                                                                // Prints `[{ "on-warnings": "exit"}]`
/// ```
#[derive(Debug)]
pub struct ParsedArguments {
    arguments: Vec<Argument>,
    positionals: Vec<String>,
    flags: Vec<char>,
    options: Vec<String>,
    variables: HashMap<String, String>,
}

trait CheckableIfArgument {
    fn is_positional(&self) -> bool;
    fn is_flag(&self) -> bool;
    fn is_option(&self) -> bool;
    fn is_variable(&self) -> bool;
}

impl CheckableIfArgument for String {
    fn is_positional(&self) -> bool {
        return !self.starts_with("-");
    }

    fn is_flag(&self) -> bool {
        return self.starts_with("-") && !self.starts_with("--");
    }

    fn is_option(&self) -> bool {
        return self.starts_with("--") && !self.contains("=");
    }

    fn is_variable(&self) -> bool {
        return self.starts_with("--") && self.contains("=");
    }
}

pub fn parse_arguments(args: Vec<String>) -> ParsedArguments {
    let mut arguments: Vec<Argument> = Vec::new();
    let mut positionals: Vec<String> = Vec::new();
    let mut options: Vec<String> = Vec::new();
    let mut flags: Vec<char> = Vec::new();
    let mut variables: HashMap<String, String> = HashMap::new();

    for item in args {
        if item.is_positional() {
            // Check for a positional argument
            positionals.push(item.clone());
            arguments.push(Argument::Positional(item.clone()));
        }

        else if item.is_flag() {
            // Check for flags
            let original = item.clone();
            let trimmed = original[1..].to_string();
            flags.append(&mut trimmed.clone().chars().collect());
            for flag in trimmed.chars().into_iter() {
                arguments.push(Argument::Flag(flag));
            }
        } 

        else if item.is_option() {
            // Check for a non-variable option
            let original = item.clone();
            let trimmed = original[2..].to_string();
            options.push(trimmed.clone());
            arguments.push(Argument::Option(trimmed));
        } 

        else if item.is_variable() {
            // Check for a variable
            let original = item.clone();
            let trimmed = original[2..].to_string();
            let split = trimmed.split_once('=');
            match split {
                Some(value) => {
                    let (before, after) = value;
                    variables.insert(before.to_string(), after.to_string());
                    arguments.push(Argument::Variable {
                        name: before.to_string(),
                        value: after.to_string(),
                    })
                }
                None => continue,
            }
        }
    }

    return ParsedArguments {
        arguments,
        positionals,
        flags,
        options,
        variables,
    };
}

pub fn collect_args() -> Vec<String> {
    return env::args().collect();
}

pub fn get_raw_args_string() -> String {
    return collect_args().join(" ");
}

pub fn collect_args_and_parse() -> ParsedArguments {
    return parse_arguments(collect_args());
}