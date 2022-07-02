# fibonacci_guru
A tool to calculate the nth term of the fibonacci sequence. 
It can calculate terms that are well above 128 bits and will keep track of how long the calculation cost. 
This tool provides access to a dynamic programming implementation as well as recursive implementation.

## Installation 
**Use Cargo

`cargo install fibonacci-guru`

**Manually downloading a release package from Github Releases [page](https://github.com/WhaleCoded/fibonacci_guru/releases)

## Usage

### Options

`A tool to calculate the nth term of the fibonacci sequence. 
It can calculate terms that are well above 128 bits and will keep track of how long the calculation cost. 
This tool provides access to a dynamic programming implementation as well as recursive implementation.

USAGE:
    fibonacci_guru [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --both <BOTH>              Uses both the dynamic programming and the recursive implementation. Takes the desired
                                   term number, n value, of the Fibonacci sequence.
    -d, --dynamic <DYNAMIC>        Uses the dynamic programming implementation. Takes the desired term number, n value,
                                   of the Fibonacci sequence.
    -l, --limit <LIMIT>            Overrides the max recursion call limit by taking a number greater than 0. The default
                                   the limit is 1073741824 which is enough to calculate n=30 of the Fibonacci sequence.
    -r, --recursive <RECURSIVE>    Uses the recursive implementation. Takes the desired term number, n value, of the
                                   Fibonacci sequence.`

### Examples

`$ fibonacci_guru -d 100

Dynamic:
The 100th term of the Fibonacci sequence is 354224848179261915075.
It took 1031 nanoseconds to calculate it.
`

`$ fibonacci_guru -r 10

Recursive:
The 10th term of the Fibonacci sequence is 55.
It took 19430 nanoseconds to calculate it.
`

`$ fibonacci_guru -b 30

Recursive:
The 30th term of the Fibonacci sequence is 832040.
It took 22 miliseconds to calculate it.

Dynamic:
The 30th term of the Fibonacci sequence is 832040.
It took 721 nanoseconds to calculate it.

The Dynamic Programming implementation was 22 miliseconds faster than the Recursive implementation.`

`fibonacci_guru -r 35 -l 34359738368

Recursive:
The 35th term of the Fibonacci sequence is 9227465.
It took 237 miliseconds to calculate it.
`