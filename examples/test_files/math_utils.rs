//! Math Utilities Module
/*startsummary
This module provides a collection of mathematical utility functions
for common operations like arithmetic, trigonometry, and statistical
calculations. All functions are designed to be fast and memory-efficient.

The module includes both basic operations (add, subtract, multiply) and
more advanced functions like factorial calculation and prime number
checking.
endsummary*/

/// Adds two numbers together
/// 
/// This is a simple addition function that takes two integers and
/// returns their sum. It's designed to be fast and handle edge cases
/// gracefully.
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
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! String Processing
/*startsummary
String manipulation utilities for common text processing tasks.
These functions help with formatting, validation, and transformation
of string data.
endsummary*/

/// Converts a string to uppercase
/// 
/// Takes any string slice and returns a new String with all
/// characters converted to uppercase. Handles Unicode correctly.
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// A new String with all characters in uppercase
fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// Calculates the factorial of a number
/// 
/// Computes n! (n factorial) using an iterative approach for
/// better performance than recursion.
/// 
/// # Arguments
/// * `n` - The number to calculate factorial for
/// 
/// # Returns
/// The factorial of n, or 0 if n is negative
/// 
/// # Panics
/// This function will panic if the result overflows a u64
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        (1..=n).product()
    }
}
// ENDVEXDOC