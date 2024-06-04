/// Convert celsius to farenheit
///
/// ```
/// use p22::calc::celsius2farenheit;
///
/// assert_eq!(celsius2farenheit(0), 32);
/// assert_eq!(celsius2farenheit(100), 212);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    (9 * celsius) / 5 + 32
}

/// Convert farenheit to celsius
///
/// ```
/// use p22::calc::farenheit2celsius;
///
/// assert_eq!(farenheit2celsius(-40), -40);
/// assert_eq!(farenheit2celsius(50), 10);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

/// Print nth fibonacci number using a loop
///
/// ```
/// use p22::calc::fibonacci_loop;
///
/// assert_eq!(fibonacci_loop(5), 5);
/// assert_eq!(fibonacci_loop(10), 55);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut i: u32 = 1;
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    loop {
        if i >= n {
            break;
        }
        second += first;
        first = second - first;
        i += 1;
    }
    second
}

/// Print nth fibonacci number using a loop
///
/// ```
/// use p22::calc::fibonacci_rec;
///
/// assert_eq!(fibonacci_rec(8), 21);
/// assert_eq!(fibonacci_rec(11), 89);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        m => fibonacci_rec(m - 1) + fibonacci_rec(m - 2),
    }
}
