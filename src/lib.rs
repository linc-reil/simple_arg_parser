use std::collections::HashMap;
use std::env;
use std::hash::Hash;

#[derive(Debug)]
pub enum Argument {
    Positional(String),
    Flag(char),
    Option(String),
    Variable { name: String, value: String },
}

#[derive(Debug)]
pub struct ParsedArguments {
    arguments: Vec<Argument>,
    positionals: Vec<String>,
    flags: Vec<char>,
    options: Vec<String>,
    variables: HashMap<String, String>,
}

pub trait CheckableIfArgument {
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