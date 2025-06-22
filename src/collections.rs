use std::collections::{HashMap, HashSet};

#[cfg(test)]

mod test{
    use super::*;
    
    #[test]
    fn tests_hashmap() {
        // Hash maps are KVP
        
        // &str -> Person
        // & u8 -> &str
        // &str -> u32
        
        let person_1 = "Alice";
        let person_2 = "Bob";
        
        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 51);
        
        let test_score: Option<&u32> = results_hm.get(person_1);
        dbg!(test_score.unwrap());
        
        if results_hm.contains_key("Alice") { 
            dbg!("Alice is Present");
        }
        
    }

    #[test]
    fn tests_hashset() {
        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Alice");
        names_hs.insert("Bob");
        names_hs.insert("Jane");
        
        if names_hs.contains("Bob") { 
            dbg!("Bob is here!");
        }
    }
}
