
pub fn compute_shortest_prefix(sorted_hashes: &[String], target: &str) -> Option<String> {
    // 1. Find the index of the target in the sorted list
    // We expect the list to be sorted.
    let idx = match sorted_hashes.binary_search_by(|p| p.as_str().cmp(target)) {
        Ok(i) => i,
        Err(_) => return None, // Target not found
    };

    // 2. Compare with left and right neighbors
    let left_neighbor = if idx > 0 {
        sorted_hashes.get(idx - 1)
    } else {
        None
    };

    let right_neighbor = sorted_hashes.get(idx + 1);

    // 3. Calculate common prefix lengths
    let left_common = left_neighbor.map(|n| common_prefix_len(n, target)).unwrap_or(0);
    let right_common = right_neighbor.map(|n| common_prefix_len(n, target)).unwrap_or(0);

    // 4. The required length is max(left, right) + 1
    // Git requires at least 4 characters
    let needed_len = std::cmp::max(std::cmp::max(left_common, right_common) + 1, 4);

    // Safety check: needed_len shouldn't exceed target length
    let final_len = std::cmp::min(needed_len, target.len());

    Some(target[..final_len].to_string())
}

fn common_prefix_len(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_in_list() {
        let hashes = vec![
            "abc1".to_string(),
            "abc2".to_string(),
            "def1".to_string(),
        ];
        // Sorted: same as above
        // Target: abc1
        // Left: None, Right: abc2
        // Common(abc1, abc2) = 3 ("abc")
        // Needed = 3 + 1 = 4. Wait. "abc1" vs "abc2". "abc" is common. Unique is "abc1". 4 chars.
        
        let res = compute_shortest_prefix(&hashes, "abc1");
        assert_eq!(res, Some("abc1".to_string()));
    }

    #[test]
    fn test_short_prefix() {
        let hashes = vec![
            "a1111".to_string(), // lengthened to allow 4 chars
            "b2222".to_string(),
            "c3333".to_string(),
        ];
        // Target: b2222
        // Left: a1111, Right: c3333. Common 0.
        // Needed: max(0+1, 4) = 4 => "b222"
        let res = compute_shortest_prefix(&hashes, "b2222");
        assert_eq!(res, Some("b222".to_string()));
    }

    #[test]
    fn test_duplicate_prefix() {
        let hashes = vec![
            "aaaa1".to_string(),
            "aaaa2".to_string(),
        ];
        // Target aaaa1
        // Right aaaa2. Common aaaa (4). Needed 5 -> aaaa1
        let res = compute_shortest_prefix(&hashes, "aaaa1");
        assert_eq!(res, Some("aaaa1".to_string()));
    }

    #[test]
    fn test_single_item() {
        let hashes = vec!["abcdef".to_string()];
        // Left None, Right None. Needed max(1, 4) = 4.
        let res = compute_shortest_prefix(&hashes, "abcdef");
        assert_eq!(res, Some("abcd".to_string()));
    }

    #[test]
    fn test_not_found() {
        let hashes = vec!["abc".to_string()];
        let res = compute_shortest_prefix(&hashes, "xyz");
        assert_eq!(res, None);
    }
}
