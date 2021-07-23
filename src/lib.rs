/// Compute the Levenshtein distance between to strings.
/// 
/// See the [Wikipedia article](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// # Example
/// ```
/// use levenshtein::lev;
/// let a = "Hello";
/// let b = "World";
/// assert_eq!(4, lev(a, b));
/// ```
pub fn lev(a: &str, b: &str) -> usize {
    const MAX_WORD_SIZE: usize = 256;
    match (a.len(), b.len()) {
        (0, b_len) => b_len,
        (a_len, 0) => a_len,
        (a_len, b_len) => {
            let mut d: [[usize;MAX_WORD_SIZE];MAX_WORD_SIZE] = [[0;MAX_WORD_SIZE];MAX_WORD_SIZE];
            for i in 1..=a_len {
                d[i][0] = i;
            }
            for j in 1..=b_len {
                d[0][j] = j;
            }

            let mut b_iter = b.chars();
            for j in 1..=b_len {
                let b_char = b_iter.next();
                let mut a_iter = a.chars();
                for i in 1..=a_len {
                    let a_char = a_iter.next();
                    let cost = if a_char == b_char {
                        0
                    } else {
                        1
                    };

                    d[i][j] = usize::min(
                        d[i-1][j] + 1,
                        usize::min(
                            d[i][j-1] + 1,
                            d[i-1][j-1] + cost
                        )
                    );
                }
            }
            d[a_len][b_len]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
