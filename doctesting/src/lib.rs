/// this is a short description
///
/// This is a longer explanation of what the function does.
/// 
/// # Examples
///
/// ```
/// use doctesting;
/// let result = doctesting::add(2,3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// this is another short description
///
/// this is a bit longer description
///
/// # Examples
///
/// ``` rust
/// use doctesting;
/// let result = doctesting::div(6.0,3.0);
/// assert_eq!(result, 2.0);
/// ```
///
/// # Panics
///
/// This panics if second argument is zero.
///
/// ``` rust, should_panic
/// let result = doctesting::div(2.0,0.0);
/// ```
pub fn div(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        panic!("This is wrong!");
    }
    x / y
}

