//! The `svg` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, svg::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use svg::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_one() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_works() {
        assert!(true);
        assert_eq!("one", "one");
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("something");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn assertion() {
        assert_eq!("Hello", "world");
    }
}
