/**
 * https://leetcode.com/problems/merge-strings-alternately
 * You are given two strings word1 and word2.
 * Merge the strings by adding letters in alternating order, starting with word1.
 * If a string is longer than the other, append the additional letters onto the end of the merged string.
 * Return the merged string.
 */
pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut w1chars = word1.chars();
    let mut w2chars = word2.chars();
    let mut result = String::with_capacity(word1.len() + word2.len());

    let mut c1 = w1chars.next();
    let mut c2 = w2chars.next();

    while c1.is_some() || c2.is_some() {
        if c1.is_some() {
            result.push(c1.unwrap());
        }
        if c2.is_some() {
            result.push(c2.unwrap());
        }

        c1 = w1chars.next();
        c2 = w2chars.next();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately_case1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
    }

    #[test]
    fn test_merge_alternately_case2() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
    }

    #[test]
    fn test_merge_alternately_case3() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }
}
