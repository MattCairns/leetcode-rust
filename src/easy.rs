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
    if s.len() != t.len() {
        false
    } else {
        let mut sl = 0;
        let mut tl = 0;
        let mut sm: HashMap<u8, u32> = HashMap::new();
        let mut tm: HashMap<u8, u32> = HashMap::new();
        let mut sv = Vec::new();
        let mut tv = Vec::new();
        sv.push(sl);
        tv.push(sl);
        (1..s.len()).for_each(|i| {
            if s.as_bytes()[i] != s.as_bytes()[i - 1] {
                if sm.contains_key(&s.as_bytes()[i]) {
                    let op = sm.get(&s.as_bytes()[i]);
                    match op {
                        Some(x) => sv.push(*x),
                        None => (),
                    }
                } else {
                    sl += 1;
                    sv.push(sl);
                    sm.insert(s.as_bytes()[i], sl);
                }
            } else {
                sv.push(sl);
            }
            if t.as_bytes()[i] != t.as_bytes()[i - 1] {
                if tm.contains_key(&t.as_bytes()[i]) {
                    let op = tm.get(&t.as_bytes()[i]);
                    match op {
                        Some(x) => tv.push(*x),
                        None => (),
                    }
                } else {
                    tl += 1;
                    tv.push(tl);
                }
            } else {
                tv.push(tl);
            }
            sm.insert(s.as_bytes()[i], sl);
            tm.insert(t.as_bytes()[i], tl);
        });

        if sv == tv {
            true
        } else {
            false
        }
    }
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
                s_index = i+1;
                break;
            }
        }
        println!("{}", s_index);
        if !found {
            return false;
        } else if s_index >= sb.len() -1 {
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

    // #[test]
    // fn test_is_isomorphic() {
    //     assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
    //     assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    //     assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false);
    //     assert_eq!(
    //         is_isomorphic("aaeaa".to_string(), "uuxyy".to_string()),
    //         false
    //     );
    //     assert_eq!(
    //         is_isomorphic("paper".to_string(), "title".to_string()),
    //         true
    //     );
    //     assert_eq!(
    //         is_isomorphic("papap".to_string(), "titii".to_string()),
    //         false
    //     );
    // }

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
        assert_eq!(
            is_isomorphic("".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
