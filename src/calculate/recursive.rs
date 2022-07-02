use crate::utils::FibonacciError;

pub fn calculate_fib_seq_recursively(
    n: u64,
    max_recursion_limit: u64,
) -> Result<(Vec<u64>, u64), FibonacciError> {
    //setup a way to keep track of recursive_calls

    return Err(FibonacciError::RecursionLimitReached);
}
