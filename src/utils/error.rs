#[derive(thiserror::Error, Debug)]
pub enum FibonacciError {
    #[error("We could not calculate the fibonnaci sequence because the value[s] provided were not whole number[s] greater than zero.")]
    UserInputError,
    #[error(
        "While calculating the fibonnaci sequence we reached the max recursion call limit. 
    If you would like to increase the limit, please include the -l flag followed by a larger number.
    By default, the limit is 1024 recursive calls."
    )]
    RecursionLimitReached,
    #[error(
        "We could not calculate the fibonnace sequence because no implementations were specified.
    Please use the flags d,r, or b followed by a number greater than or equal to 0."
    )]
    MissingImplementation,
}
