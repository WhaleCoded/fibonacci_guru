const NUM_NANOSECONDS_IN_MILI: u64 = 1000000;

pub struct ImplementationResult {
    nth_term: num_bigint::BigUint,
    n: u64,
    time: u64,
}

impl ImplementationResult {
    pub fn new(nth_term: num_bigint::BigUint, n: u64, time: u64) -> ImplementationResult {
        return ImplementationResult { nth_term, n, time };
    }

    pub fn to_string(&self) -> String {
        let final_string = match self.time {
            _ if self.time > (NUM_NANOSECONDS_IN_MILI * 2) => {
                format! {"The {}th term of the Fibonacci sequence is {}.\nIt took {} miliseconds to calculate it.", self.n, self.nth_term, (self.time / NUM_NANOSECONDS_IN_MILI)}
            }
            _ if self.time <= (NUM_NANOSECONDS_IN_MILI * 2) => {
                format! {"The {}th term of the Fibonacci sequence is {}.\nIt took {} nanoseconds to calculate it.", self.n, self.nth_term, self.time}
            }
            _ => {
                //not possible
                String::from("")
            }
        };
        return final_string;
    }
}

pub struct FibonacciResult {
    recursive_result: Option<ImplementationResult>,
    dynamic_result: Option<ImplementationResult>,
}

impl FibonacciResult {
    pub fn new(
        recursive_option: Option<ImplementationResult>,
        dynamic_option: Option<ImplementationResult>,
    ) -> FibonacciResult {
        return FibonacciResult {
            recursive_result: recursive_option,
            dynamic_result: dynamic_option,
        };
    }

    pub fn to_string(&self) -> String {
        let display_text = match (&self.recursive_result, &self.dynamic_result) {
            (Some(rec_result), Some(dyn_result)) => {
                let implementation_strings = format! {"\nRecursive:\n{}\n\nDynamic:\n{}\n\n", rec_result.to_string(), dyn_result.to_string()};
                let time_dif = rec_result.time - dyn_result.time;

                let final_string = match time_dif {
                    time_dif if time_dif > (NUM_NANOSECONDS_IN_MILI * 2) => {
                        format! {"{}The Dynamic Programming implementation was {} miliseconds faster than the Recursive implementation."
                        , implementation_strings, time_dif / NUM_NANOSECONDS_IN_MILI}
                    }
                    time_dif if time_dif <= (NUM_NANOSECONDS_IN_MILI * 2) => {
                        format! {"{}The Dynamic Programming implementation was {} nanoseconds faster than the Recursive implementation."
                        , implementation_strings, time_dif}
                    }
                    _ => {
                        //not possible
                        String::from("")
                    }
                };

                final_string
            }
            (Some(rec_result), _) => {
                format! {"\nRecursive:\n{}\n\n", rec_result.to_string()}
            }
            (_, Some(dyn_result)) => {
                format! {"\nDynamic:\n{}\n\n", dyn_result.to_string()}
            }
            _ => {
                //not possible
                format!(
                    "\nNo implementation was provided. This state should be impossible to reach."
                )
            }
        };

        return display_text;
    }
}
