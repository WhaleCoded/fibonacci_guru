//Calculate the fibonacci sequence one value at a time
//This function starts at one and works its way up to n

use num_bigint::ToBigUint;

//we use unwrap only in cases where it is impossible to fail
pub fn calculate_fib_seq_dynamically(n: u64) -> (num_bigint::BigUint, u64) {
    let start_time = chrono::Utc::now().timestamp_nanos();

    //check for edge cases
    match n {
        0 => {
            return (
                (0 as u32).to_biguint().unwrap(),
                (chrono::Utc::now().timestamp_nanos() - start_time)
                    .try_into()
                    .unwrap(),
            );
        }
        1 => {
            return (
                (1 as u32).to_biguint().unwrap(),
                (chrono::Utc::now().timestamp_nanos() - start_time)
                    .try_into()
                    .unwrap(),
            );
        }
        _ => {
            let mut prev_term = (0 as u32).to_biguint().unwrap();
            let mut fib = (1 as u32).to_biguint().unwrap();

            for _ in 2..(n + 1) {
                prev_term = prev_term + &fib;
                std::mem::swap(&mut prev_term, &mut fib);
            }

            return (
                fib,
                (chrono::Utc::now().timestamp_nanos() - start_time)
                    .try_into()
                    .unwrap(),
            );
        }
    }
}
