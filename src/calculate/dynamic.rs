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
            //we keep at most 3 terms in memory at a time
            let mut term_0: num_bigint::BigUint = (0 as u32).to_biguint().unwrap();
            let mut term_1: num_bigint::BigUint = (1 as u32).to_biguint().unwrap();
            let mut term_placeholder: num_bigint::BigUint;

            for _ in 2..(n + 1) {
                term_placeholder = term_0 + &term_1;
                term_0 = term_1;
                term_1 = term_placeholder;
            }

            return (
                term_1,
                (chrono::Utc::now().timestamp_nanos() - start_time)
                    .try_into()
                    .unwrap(),
            );
        }
    }
}
