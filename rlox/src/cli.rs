use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct CliArgs {
    pub source: String,
}

#[derive(Debug)]
pub enum CliError {
    InvalidArgumentsLength,
    NoArguments,
    FileReadError { path: String, error: std::io::Error },
}

impl CliArgs {
    pub fn new() -> Result<CliArgs, CliError> {
        let mut args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            return Err(CliError::NoArguments);
        }

        if args.len() > 1 {
            return Err(CliError::InvalidArgumentsLength);
        }

        let source = args.pop().unwrap();

        Ok(CliArgs { source })
    }
}

#[derive(Debug)]
pub struct Cli {
    pub args: CliArgs,
    pub source: String,
}

impl Cli {
    pub fn new() -> Result<Cli, CliError> {
        let args = CliArgs::new()?;
        let source = read_to_string(&args.source).map_err(|error| CliError::FileReadError {
            path: args.source.clone(),
            error,
        })?;
        Ok(Cli { args, source })
    }
}
