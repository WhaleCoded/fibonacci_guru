pub struct ImplementationResult {
    sequence: Vec<u64>,
    time: u64,
}

impl ImplementationResult {
    pub fn new(sequence: Vec<u64>, time: u64) -> ImplementationResult {
        return ImplementationResult { sequence, time };
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
}
