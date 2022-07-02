use clap::{self, clap_app};

mod calculate;
mod utils;

use utils::{FibonacciError, FibonacciResult, ImplementationResult, UserCommand};

//this limit allows them to calculate up to n = 30
const DEFAULT_RECURSION_LIMIT: u64 = 1073741824;

fn main() {
    //retrieve command line flags and values
    let commandline_arg_matches = clap::clap_app!(fibonacci_guru =>
        (about: "A tool to calculate the nth term of the fibonacci sequence. 
It can calculate terms that are well above 128 bits and will keep track of how long the calculation cost. 
This tool provides access to a dynamic programming implementation as well as recursive implementation.")
        (version: "1.0")
        (@arg DYNAMIC: -d --dynamic + takes_value "Uses the dynamic programming implementation. Takes the desired term number, n value, of the Fibonacci sequence.")
        (@arg RECURSIVE: -r --recursive + takes_value "Uses the recursive implementation. Takes the desired term number, n value, of the Fibonacci sequence.")
        (@arg BOTH: -b --both + takes_value "Uses both the dynamic programming and the recursive implementation. Takes the desired term number, n value, of the Fibonacci sequence.")
        (@arg LIMIT: -l --limit + takes_value "Overrides the max recursion call limit by taking a number greater than 0. The default the limit is 1073741824 which is enough to calculate n=30 of the Fibonacci sequence.")
    )
    .get_matches();

    match execute_user_commands(commandline_arg_matches) {
        Ok(fib_result) => {
            println!("{}", fib_result.to_string())
        }
        Err(err) => {
            println!("{}", err.to_string())
        }
    }
}

fn execute_user_commands(arg_matches: clap::ArgMatches) -> Result<FibonacciResult, FibonacciError> {
    //get the command
    let command = get_desired_command(arg_matches)?;

    //execute the command
    let fib_result = match command {
        UserCommand::RECURSIVE(n, max_recursion_calls) => {
            let (recursive_seq, recursive_time) =
                calculate::calculate_fib_seq_recursively(n, max_recursion_calls)?;

            let recursive_result = ImplementationResult::new(recursive_seq, n, recursive_time);

            FibonacciResult::new(Some(recursive_result), None)
        }
        UserCommand::DYNAMIC(n) => {
            let (dynamic_seq, dynamic_time) = calculate::calculate_fib_seq_dynamically(n);

            let dynamic_result = ImplementationResult::new(dynamic_seq, n, dynamic_time);

            FibonacciResult::new(None, Some(dynamic_result))
        }
        UserCommand::BOTH(((recursive_n, max_recursion_calls), dynamic_n)) => {
            let (recursive_seq, recursive_time) =
                calculate::calculate_fib_seq_recursively(recursive_n, max_recursion_calls)?;
            let (dynamic_seq, dynamic_time) = calculate::calculate_fib_seq_dynamically(dynamic_n);

            let recursive_result =
                ImplementationResult::new(recursive_seq, recursive_n, recursive_time);
            let dynamic_result = ImplementationResult::new(dynamic_seq, dynamic_n, dynamic_time);

            FibonacciResult::new(Some(recursive_result), Some(dynamic_result))
        }
    };

    return Ok(fib_result);
}

fn get_desired_command(arg_matches: clap::ArgMatches) -> Result<UserCommand, FibonacciError> {
    let max_recursion_calls = match arg_matches.value_of("LIMIT") {
        Some(limit_str) => clean_user_input(limit_str)?,
        None => DEFAULT_RECURSION_LIMIT,
    };

    let command = match (
        arg_matches.value_of("RECURSIVE"),
        arg_matches.value_of("DYNAMIC"),
        arg_matches.value_of("BOTH"),
    ) {
        (_, _, Some(both_input)) => {
            let n = clean_user_input(both_input)?;
            UserCommand::BOTH(((n, max_recursion_calls), n))
        }
        (Some(recursive_input), Some(dynamic_input), _) => {
            let recursive_n = clean_user_input(recursive_input)?;
            let dynamic_n = clean_user_input(dynamic_input)?;

            UserCommand::BOTH(((recursive_n, max_recursion_calls), dynamic_n))
        }
        (Some(recursive_input), _, _) => {
            UserCommand::RECURSIVE(clean_user_input(recursive_input)?, max_recursion_calls)
        }
        (_, Some(dynamic_input), _) => UserCommand::DYNAMIC(clean_user_input(dynamic_input)?),
        _ => return Err(FibonacciError::MissingImplementation),
    };

    return Ok(command);
}

//this function checks that the user input a valid number
fn clean_user_input(user_input: &str) -> Result<u64, FibonacciError> {
    match user_input.parse() {
        Ok(user_value) => return Ok(user_value),
        Err(_) => return Err(FibonacciError::UserInputError),
    }
}
