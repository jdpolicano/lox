//use lox::cli::{Cli, CliError};
use rlox::interpreter::Interpreter;
use rlox::parser::{ParseError, Parser};
use rlox::scanner::Scanner;

fn main() {
    let input = r#"
        // prints from 0 -> 9
        var name = "John";

        fun sayHiTo(age) {
            print "Hi " + name + " I heard your age is " + age;
        }

        sayHiTo("30");
    "#;
    let tokens = Scanner::new(input).scan_tokens();

    if tokens.is_err() {
        println!("Error scanning tokens: {:?}", tokens.err().unwrap());
        return;
    }

    let ast = Parser::new(tokens.unwrap()).parse();

    if ast.is_err() {
        print_ast_errors(ast.err().unwrap());
        return;
    }

    let result = Interpreter::new().interpret(&ast.unwrap());

    if result.is_err() {
        // println!("Error interpreting ast");
        println!("{}", result.err().unwrap());
        return;
    }

    //println!("{:?}", result.unwrap());
    // let cli = Cli::new();

    // if cli.is_err() {
    //     handle_cli_error(cli.err().unwrap());
    //     return;
    // }

    // let cli = cli.unwrap();
    // println!("{}", cli.source);
}

fn print_ast_errors(errors: Vec<ParseError>) {
    for error in errors {
        println!("{}", error);
    }
}

// fn print_usage() {
//     println!("Usage: lox [script]");
// }

// fn handle_cli_error(cli_err: CliError) {
//     match cli_err {
//         CliError::InvalidArgumentsLength => {
//             println!("Invalid number of arguments");
//             print_usage();
//         }
//         CliError::NoArguments => {
//             println!("No arguments provided");
//             print_usage();
//         }
//         CliError::FileReadError { path, error } => {
//             println!("Error reading file \"{}\": {}", path, error);
//         }
//     }
// }
