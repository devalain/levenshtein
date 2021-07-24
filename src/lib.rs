/// Compute the Levenshtein distance between two strings.
///
/// See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance) and the 
/// [Wagner–Fischer algorithm](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm).
/// This actual algorithm was inspired by [wooorm](https://github.com/wooorm/levenshtein-rs).
///
/// # Example
/// ```
/// use levenshtein::lev;
/// let a = "Hello";
/// let b = "World";
/// assert_eq!(4, lev(a, b));
/// ```
pub fn lev(a: &str, b: &str) -> usize {
    let b_len = match (a.len(), b.len()) {
        (0, b_len) => return b_len,
        (a_len, 0) => return a_len,
        (_, b_len) => b_len
    };
    if a == b {
        return 0;
    }

    let mut current = 0_usize;
    let mut left;
    let mut up;
    let mut left_up = 0_usize;
    let mut row_up = (1..).take(b_len).collect::<Vec<usize>>();

    for (a_idx, a_char) in a.chars().enumerate() {
        left = a_idx + 1;
        for (b_idx, b_char) in b.chars().enumerate() {
            up = row_up[b_idx];
            let cost = if a_char == b_char { 0 } else { 1 };
            current = usize::min(
                left + 1,
                usize::min(
                    up + 1,
                    left_up + cost
                )
            );
            row_up[b_idx] = current;
            left = current;
            left_up = up;
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::lev;

    #[test]
    fn empty_strings() {
        let a = "";
        let b = "";
        assert_eq!(0, lev(a, b));
    }

    #[test]
    fn same_strings_len_1() {
        let a = "e";
        let b = "e";
        assert_eq!(0, lev(a, b));
    }

    #[test]
    fn different_strings_len_1() {
        let a = "e";
        let b = "p";
        assert_eq!(1, lev(a, b));
    }

    #[test]
    fn same_strings() {
        let a = "examen";
        let b = "examen";
        assert_eq!(0, lev(a, b));
    }
    
    #[test]
    fn distance_1_strings() {
        let a = "examen";
        let b = "examan";
        assert_eq!(1, lev(a, b));
    }
    
    #[test]
    fn distance_4_strings() {
        let a = "Hello";
        let b = "World";
        assert_eq!(4, lev(a, b));
    }
}
