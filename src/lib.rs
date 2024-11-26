//! game engine for FizzBuzz

/// play FizzBuzz game
/// # Arguments
/// * `count` - i32: number of times to play the game
/// # Returns
/// * `Vec<String>`: list of strings representing the game
/// # Example
/// ```
/// let result = fizzbuzz::play(5);
/// assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
/// ```
pub fn play(count: i32) -> Vec<String> {
    (1..=count).map(|i| fizz_buzz(i)).collect()
}

/// calculate FizzBuzz for a number
/// # Arguments
/// * `n` - i32: number to calculate FizzBuzz for
/// # Returns
/// * `String`: Fizz, Buzz, FizzBuzz or the number as a string
/// # Example
/// ```
/// let result = fizzbuzz::fizz_buzz(3);
/// assert_eq!(result, "Fizz");
/// ```
pub fn fizz_buzz(n: i32) -> String {
    if n % 15 == 0 {
        return "FizzBuzz".to_string();
    } else if n % 3 == 0 {
        return "Fizz".to_string();
    } else if n % 5 == 0 {
        return "Buzz".to_string();
    } else {
        return n.to_string();
    }
}
