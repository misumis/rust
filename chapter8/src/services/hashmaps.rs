use std::collections::HashMap;

pub fn display_hash_maps() {
    // Hash maps
    let mut hm = HashMap::new();

    hm.insert(String::from("Blue"), 10);
    let team_name = String::from("Blue");
    let score = hm.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);
    println!("{:?}", hm);
}
