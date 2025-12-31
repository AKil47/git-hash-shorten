use git2::Repository;
use std::path::Path;

/// Trait to abstract the source of hashes (Mockable)
pub trait HashRepository {
    fn get_all_hashes(&self) -> Result<Vec<String>, String>;
}

/// Real implementation using git2
pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let repo = Repository::open(path).map_err(|e| e.to_string())?;
        Ok(Self { repo })
    }
}

impl HashRepository for GitRepository {
    fn get_all_hashes(&self) -> Result<Vec<String>, String> {
        let odb = self.repo.odb().map_err(|e| e.to_string())?;
        let mut hashes = Vec::new();
        odb.foreach(|oid| {
            hashes.push(oid.to_string());
            true
        }).map_err(|e| e.to_string())?;
        Ok(hashes)
    }
}

/// High-level function that ties everything together
pub fn resolve(repo: &impl HashRepository, target: &str) -> Result<String, String> {
    let mut hashes = repo.get_all_hashes()?;
    
    // Core logic
    hashes.sort();
    
    // Normalize input hash to lowercase
    let target = target.to_lowercase();
    
    compute_shortest_prefix(&hashes, &target)
        .ok_or_else(|| format!("Hash '{}' not found in repository", target))
}

/// Pure logic function (Internal)
fn compute_shortest_prefix(sorted_hashes: &[String], target: &str) -> Option<String> {
    // 1. Find the index of the target in the sorted list
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

    // Mock implementation
    struct MockRepo {
        hashes: Vec<String>,
    }
    
    impl HashRepository for MockRepo {
        fn get_all_hashes(&self) -> Result<Vec<String>, String> {
            Ok(self.hashes.clone())
        }
    }

    #[test]
    fn test_integration_mocked_success() {
        let repo = MockRepo {
            hashes: vec![
                "aaaa1000".to_string(),
                "bbbb2000".to_string(),
                "cccc3000".to_string()
            ]
        };
        
        // Should find match and shorten
        let res = resolve(&repo, "bbbb2000").expect("Should find hash");
        assert_eq!(res, "bbbb"); // min 4 chars
    }

    #[test]
    fn test_integration_mocked_not_found() {
        let repo = MockRepo {
            hashes: vec!["aaaa1000".to_string()]
        };
        
        let res = resolve(&repo, "zzzz9000");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("not found"));
    }
    
    #[test]
    fn test_integration_mocked_collision_logic() {
         let repo = MockRepo {
            hashes: vec![
                "abcde100".to_string(),
                "abcde200".to_string(), // Shared prefix "abcde" (5 chars)
            ]
        };
        // Target abcde100. Neighbor abcde200. Common abcde (5).
        // Needed 6 -> abcde1
        
        let res = resolve(&repo, "abcde100").unwrap();
        assert_eq!(res, "abcde1");
    }

    // Original Unit Tests for pure logic
    #[test]
    fn test_unique_in_list() {
        let hashes = vec![
            "abc111".to_string(),
            "abc222".to_string(),
            "def111".to_string(),
        ];
        // Sorted: same as above
        // Target: abc1
        // Left: None, Right: abc2
        // Common(abc1, abc2) = 3 ("abc")
        // Needed = 3 + 1 = 4. 
        // Min 4. 
        
        let res = compute_shortest_prefix(&hashes, "abc111");
        assert_eq!(res, Some("abc1".to_string()));
    }
}
