/** Sum of Character Codes.
 * 
 * Time Complexity: O(N^3) Cubic
 * Space Complexity: O(1) Constant
 * 
 * This function takes a string and returns the sum of the character codes of each character.
 */
pub fn sum_of_character_codes(s: &str) -> u32 {
    let mut sum = 0;
    for c in s.chars() {
        sum += c as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_character_codes() {
        assert_eq!(sum_of_character_codes("abc"), 294);
    }
}