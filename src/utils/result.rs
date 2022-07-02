pub struct ImplementationResult {
    nth_term: u64,
    n: u64,
    time: u64,
}

impl ImplementationResult {
    pub fn new(nth_term: u64, n: u64, time: u64) -> ImplementationResult {
        return ImplementationResult { nth_term, n, time };
    }

    pub fn to_string(&self) -> String {
        return format! {"The {}th term of the Fibonacci sequence is {}. It took {} miliseconds to calculate it.", self.nth_term, self.n, self.time};
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
                format! {"Recursive:\n{}\nDynamic:\n{}\nThe Dynamic Programming implementation was {} faster than the Recursive implementation."
                , rec_result.to_string(), dyn_result.to_string(), rec_result.time - dyn_result.time}
            }
            (Some(rec_result), _) => {
                format! {"Recursive:\n{}\n", rec_result.to_string()}
            }
            (_, Some(dyn_result)) => {
                format! {"Dynamic:\n{}\n", dyn_result.to_string()}
            }
            _ => {
                format!("No implementation was provided. This state should be impossible to reach.")
            }
        };

        return display_text;
    }
}
