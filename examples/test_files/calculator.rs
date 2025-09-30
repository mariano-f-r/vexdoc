//! Calculator Module
/*startsummary
A simple calculator module that provides basic mathematical operations.
This module demonstrates how to use VexDoc with Rust code and includes
comprehensive documentation with examples.
endsummary*/

/// Adds two numbers together
/// 
/// This function performs basic addition of two integers and returns
/// the result. It's designed to be simple and efficient.
/// 
/// # Arguments
/// * `a` - The first number to add
/// * `b` - The second number to add
/// 
/// # Returns
/// The sum of `a` and `b`
/// 
/// # Examples
/// ```
/// let result = add(5, 3);
/// assert_eq!(result, 8);
/// 
/// let result = add(-5, 3);
/// assert_eq!(result, -2);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! Advanced Operations
/*startsummary
More complex mathematical operations including factorial calculation
and power functions. These operations demonstrate more advanced
Rust features and error handling.
endsummary*/

/// Calculates the factorial of a number
/// 
/// Computes n! (n factorial) using an iterative approach for
/// better performance than recursion. The function handles edge
/// cases gracefully.
/// 
/// # Arguments
/// * `n` - The number to calculate factorial for (must be non-negative)
/// 
/// # Returns
/// The factorial of n, or 1 if n is 0
/// 
/// # Panics
/// This function will panic if the result overflows a u64
/// 
/// # Examples
/// ```
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(10), 3628800);
/// ```
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        (1..=n).product()
    }
}

/// Calculates a number raised to a power
/// 
/// Efficiently computes base^exponent using the binary exponentiation
/// algorithm, which is much faster than naive repeated multiplication
/// for large exponents.
/// 
/// # Arguments
/// * `base` - The base number
/// * `exponent` - The exponent (must be non-negative)
/// 
/// # Returns
/// The result of base^exponent
/// 
/// # Examples
/// ```
/// assert_eq!(power(2, 3), 8);
/// assert_eq!(power(5, 0), 1);
/// assert_eq!(power(3, 4), 81);
/// ```
fn power(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        1
    } else if exponent == 1 {
        base
    } else {
        let half = power(base, exponent / 2);
        if exponent % 2 == 0 {
            half * half
        } else {
            half * half * base
        }
    }
}
// ENDVEXDOC
