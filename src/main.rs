use leetcode::easy;

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }
    let mut shortest_str = 0;
    for i in 1..strs.len() {
        println!("len {}", strs[i].len());
        println!("len {}", strs[i - 1].len());
        if strs[i].len() < strs[i - 1].len() {
            shortest_str = i;
        }
    }

    println!("WHAT {}", strs[shortest_str]);
    println!("LEN {}", strs[0].len());
    if strs[shortest_str].len() == 0 {
        return String::new();
    }

    println!("CONT");
    let mut prefix = String::new();
    let mut found = false;
    for c in strs[shortest_str].chars() {
        prefix.push(c);
        for (i, s) in strs.iter().enumerate() {
            if i == shortest_str {
                continue;
            }
            if !s[0..prefix.len()].contains(&prefix) {
                found = true;
                prefix.pop();
            }
        }
        if found {
            break;
        }
    }

    prefix
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut check = nums[0];
    let mut i = 1;
    while i < nums.len() {
        if nums[i] == check {
            nums.remove(i);
            i -= 1;
        } else {
            check = nums[i];
        }
        i += 1;
    }

    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut v = vec![1, 1, 2, 2];
        assert_eq!(remove_duplicates(&mut v), 2);
        assert_eq!(v, vec![1, 2]);

        let mut v = vec![1, 1, 2, 2, 3, 4, 4];
        assert_eq!(remove_duplicates(&mut v), 4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}

fn main() {
    easy::is_subsequence("abc".to_string(), "ahbgdc".to_string());
}
