mod dynamic;
mod recursive;

// Fn = Fn-1 + Fn-2
// F0 = 0
// F1 = F2 = 1

pub use dynamic::calculate_fib_seq_dynamically;
pub use recursive::calculate_fib_seq_recursively;
