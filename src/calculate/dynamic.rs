//Calculate the fibonacci sequence one value at a time
//This function starts at one and works its way up to n

//we use unwrap only in cases where it is impossible to fail
pub fn calculate_fib_seq_dynamically(n: u64) -> (u128, u64) {
    let start_time = chrono::Utc::now().timestamp_millis();
    let mut fib_seq = vec![0; (n + 1).try_into().unwrap()];

    for index in 1..(n + 1) {
        if index == 1 {
            *fib_seq.get_mut(index as usize).unwrap() = 1;
        } else {
            *fib_seq.get_mut(index as usize).unwrap() = fib_seq.get((index - 1) as usize).unwrap()
                + fib_seq.get((index - 2) as usize).unwrap();
        }
    }

    return (
        *fib_seq.get(n as usize).unwrap(),
        (chrono::Utc::now().timestamp_millis() - start_time)
            .try_into()
            .unwrap(),
    );
}
