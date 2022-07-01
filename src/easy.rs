use std::collections::HashMap;

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);
    nums.len() as i32
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    match haystack.find(&needle) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut sum = 0;
    for val in nums {
        sum += val;
        output.push(sum);
    }
    output
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().sum::<i32>();
    let mut index: i32 = -1;
    let mut left: i32 = 0;
    for (i, v) in nums.iter().enumerate() {
        if left == (sum - nums[i] - left) {
            index = i as i32;
            break;
        }
        left += v;
    }
    index
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    println!("{} == {}", t, s);
    let mut b = true;
    if s.len() != t.len() {
        b = false;
    } else {
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let mut s_map: HashMap<u8, u8> = HashMap::new();
        let mut t_map: HashMap<u8, u8> = HashMap::new();
        // let ret = (1..s.len()).for_each(|i| -> bool {
        for i in 0..s.len() {
            if !s_map.contains_key(&sb[i]) && !t_map.contains_key(&tb[i]) {
                s_map.insert(sb[i], tb[i]);
                t_map.insert(tb[i], sb[i]);
            } 
            if s_map.get(&sb[i]).unwrap() != &tb[i] || t_map.get(&tb[i]).unwrap() != &sb[i] {
                b = false;
            } 
        }
    }
    b
}

pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 || t.len() == 0 {
        return false;
    }
    let sb = s.as_bytes();
    let tb = t.as_bytes();
    let mut s_index = 0;

    loop {
        let mut found = false;
        for i in s_index..tb.len() {
            if sb[s_index] == tb[i] {
                found = true;
                s_index = i + 1;
                break;
            }
        }
        println!("{}", s_index);
        if !found {
            return false;
        } else if s_index >= sb.len() - 1 {
            break;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![1, 2, 3, 4];
        let val = 3;
        assert_eq!(remove_element(&mut nums, val), 3);
        assert_eq!(nums, vec![1, 2, 4]);
    }

    #[test]
    fn test_str_str() {
        let haystack = String::from("hello world");
        let needle = String::from("orld");
        assert_eq!(str_str(haystack, needle), 7);

        let haystack = String::from("hello world");
        let needle = String::from("zz");
        assert_eq!(str_str(haystack, needle), -1);

        let haystack = String::from("hello world");
        let needle = String::from("");
        assert_eq!(str_str(haystack, needle), 0);
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum(nums), vec![1, 3, 6, 10]);

        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(running_sum(nums), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_pivot_index() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);

        let nums = vec![1, 2, 3];
        assert_eq!(pivot_index(nums), -1);

        let nums = vec![2, 1, -1];
        assert_eq!(pivot_index(nums), 0);
    }

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(is_isomorphic("leet".to_string(), "code".to_string()), false);
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
        assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false);
        assert_eq!(
            is_isomorphic("aaeaa".to_string(), "uuxyy".to_string()),
            false
        );
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
        assert_eq!(
            is_isomorphic("papap".to_string(), "titii".to_string()),
            false
        );
    }

    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            is_isomorphic("axc".to_string(), "ahbgdc".to_string()),
            false
        );
        assert_eq!(is_isomorphic("".to_string(), "ahbgdc".to_string()), false);
    }
}
