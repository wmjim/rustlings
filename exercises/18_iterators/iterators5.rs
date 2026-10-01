// 我们来定义一个简单的模型，用来跟踪 Rustlings 的练习进度。
// 进度会用哈希表来表示：练习的名字是键，进度是值。
// 我们已经写了两个计数函数，用来统计处于某种进度的练习有多少个。
// 请用迭代器重新实现这两个计数功能。尽量不要使用命令式循环（for/while）。

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// TODO: 实现与 `count_for` 相同的功能，但要用迭代器代替 `for` 循环。
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` 是一个以 `String` 为键、以 `Progress` 为值的哈希表。
    // map = { "variables1": Complete, "from_str": None, … }
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// TODO: 实现与 `count_collection_for` 相同的功能，但要用迭代器代替 `for` 循环。
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` 是一个由哈希表组成的切片。
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
        }
    }
}
