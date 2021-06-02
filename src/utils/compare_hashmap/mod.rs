use std::collections::HashMap;
use std::hash::Hash;

pub fn hashmap_eq<I: Eq + Hash, T: PartialEq>(left: &HashMap<I, T>, right: &HashMap<I, T>) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for i in left.keys().into_iter() {
        match right.get(i) {
            None => return false,
            Some(right) => {
                let left = left.get(i).unwrap();
                if right != left {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::utils::compare_hashmap::hashmap_eq;

    #[test]
    fn test_hashmap_eq() {
        let map_0_1: HashMap<&str, &str> = HashMap::new();

        let mut map_1_1: HashMap<&str, &str> = HashMap::new();
        map_1_1.insert("1", "This is one");

        let mut map_2_1: HashMap<&str, &str> = HashMap::new();
        map_2_1.insert("1", "This is one");
        map_2_1.insert("2", "This is two");
        let mut map_2_2: HashMap<&str, &str> = HashMap::new();
        map_2_2.insert("2", "This is two");
        map_2_2.insert("1", "This is one");

        let mut map_3_1: HashMap<&str, &str> = HashMap::new();
        map_3_1.insert("1", "This is one");
        map_3_1.insert("2", "This is two");
        map_3_1.insert("3", "This is three");
        let mut map_3_2: HashMap<&str, &str> = HashMap::new();
        map_3_2.insert("3", "This is three");
        map_3_2.insert("2", "This is two");
        map_3_2.insert("1", "This is one");

        let mut map_4_1: HashMap<&str, &str> = HashMap::new();
        map_4_1.insert("1", "This is one");
        map_4_1.insert("2", "This is two");
        map_4_1.insert("3", "This is three");
        map_4_1.insert("4", "This is four");
        let mut map_4_2: HashMap<&str, &str> = HashMap::new();
        map_4_2.insert("3", "This is three");
        map_4_2.insert("2", "This is two");
        map_4_2.insert("1", "This is one");
        map_4_2.insert("4", "This is four");

        let mut map_4_2_a: HashMap<&str, &str> = HashMap::new();
        map_4_2_a.insert("3", "This is three");
        map_4_2_a.insert("2", "This is not two");
        map_4_2_a.insert("1", "This is one");
        map_4_2_a.insert("4", "This is four");

        assert!(hashmap_eq(&map_0_1, &map_0_1));

        assert!(hashmap_eq(&map_1_1, &map_1_1));

        assert!(hashmap_eq(&map_2_1, &map_2_1));
        assert!(hashmap_eq(&map_2_1, &map_2_2));

        assert!(hashmap_eq(&map_3_1, &map_3_1));
        assert!(hashmap_eq(&map_3_1, &map_3_2));

        assert!(hashmap_eq(&map_4_1, &map_4_1));
        assert!(hashmap_eq(&map_4_1, &map_4_2));

        assert!(!hashmap_eq(&map_4_1, &map_4_2_a));
        assert!(!hashmap_eq(&map_0_1, &map_1_1));
        assert!(!hashmap_eq(&map_1_1, &map_2_1));
        assert!(!hashmap_eq(&map_2_1, &map_3_1));
        assert!(!hashmap_eq(&map_3_1, &map_4_1));

        //Testing the other way around

        assert!(hashmap_eq(&map_2_2, &map_2_1));

        assert!(hashmap_eq(&map_3_2, &map_3_1));

        assert!(hashmap_eq(&map_4_2, &map_4_1));

        assert!(!hashmap_eq(&map_4_2_a, &map_4_1));
        assert!(!hashmap_eq(&map_1_1, &map_0_1));
        assert!(!hashmap_eq(&map_2_1, &map_1_1));
        assert!(!hashmap_eq(&map_3_1, &map_2_1));
        assert!(!hashmap_eq(&map_4_1, &map_3_1));
    }
}