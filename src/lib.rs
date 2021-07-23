/// Compute the Levenshtein distance between to strings.
/// 
/// See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// # Example
/// ```
/// use levenshtein::lev;
/// let a = String::from("Hello");
/// let b = String::from("World");
/// assert_eq!(4, lev(&a, &b));
/// ```
pub fn lev(a: &str, b: &str) -> usize {
    match (a.len(), b.len()) {
        (0, b_len) => b_len,
        (a_len, 0) => a_len,
        (_, _) => {
            if a.chars().next() == b.chars().next() {
                lev(&a[1..], &b[1..]) // Warning case a.len() == 1 !
            } else {
                1 + usize::min(
                    lev(&a[1..], b),
                    usize::min(
                        lev(a, &b[1..]),
                        lev(&a[1..], &b[1..])
                    )
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings() {
        let a = String::from("");
        let b = String::from("");
        assert_eq!(0, lev(&a, &b));
    }

    #[test]
    fn same_strings_len_1() {
        let a = String::from("e");
        let b = String::from("e");
        assert_eq!(0, lev(&a, &b));
    }

    #[test]
    fn different_strings_len_1() {
        let a = String::from("e");
        let b = String::from("p");
        assert_eq!(1, lev(&a, &b));
    }

    #[test]
    fn same_strings() {
        let a = String::from("examen");
        let b = String::from("examen");
        assert_eq!(0, lev(&a, &b));
    }
    
    #[test]
    fn distance_1_strings() {
        let a = String::from("examen");
        let b = String::from("examan");
        assert_eq!(1, lev(&a, &b));
    }
    
    #[test]
    fn distance_4_strings() {
        let a = String::from("Hello");
        let b = String::from("World");
        assert_eq!(4, lev(&a, &b));
    }
}
