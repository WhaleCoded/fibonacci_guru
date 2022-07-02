use num_bigint::ToBigUint;

use crate::utils::FibonacciError;

pub fn calculate_fib_seq_recursively(
    n: u64,
    max_recursion_calls: u64,
) -> Result<(num_bigint::BigUint, u64), FibonacciError> {
    let start_time = chrono::Utc::now().timestamp_nanos();

    let (nth_term, _) = calculate_fib_seq(n, 0, max_recursion_calls)?;

    return Ok((
        nth_term,
        (chrono::Utc::now().timestamp_nanos() - start_time)
            .try_into()
            .unwrap(),
    ));
}

fn calculate_fib_seq(
    n: u64,
    curr_num_recursion_calls: u64,
    max_recursion_calls: u64,
) -> Result<(num_bigint::BigUint, u64), FibonacciError> {
    if curr_num_recursion_calls >= max_recursion_calls {
        return Err(FibonacciError::RecursionLimitReached);
    }

    if n == 0 {
        return Ok((
            (0 as u32).to_biguint().unwrap(),
            curr_num_recursion_calls + 1,
        ));
    } else if n == 1 {
        return Ok((
            (1 as u32).to_biguint().unwrap(),
            curr_num_recursion_calls + 1,
        ));
    } else {
        let (term_1, num_calls) =
            calculate_fib_seq(n - 1, curr_num_recursion_calls + 1, max_recursion_calls)?;
        let (term_2, num_calls) = calculate_fib_seq(n - 2, num_calls + 1, max_recursion_calls)?;

        return Ok((term_1 + term_2, num_calls));
    }
}
