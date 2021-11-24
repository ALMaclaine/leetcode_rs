pub fn solution(s: String) -> i32 {
    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();
    let (mut start, mut end, mut max) = (0, 0, 0);

    while end < len {
        for idx in start..end {
            if seq[end] == seq[idx] {
                start = idx + 1;
                break;
            }
        }
        let curr = end - start + 1;
        if curr > max {
            max = curr
        }
        end += 1
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            solution("abcabcbb".to_string()),
            3
        );
        assert_eq!(solution("bbbb".to_string()), 1);
        assert_eq!(
            solution("pwwkew".to_string()),
            3
        );
    }
}
