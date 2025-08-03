// Use a HashMap<String, i32> to count word frequencies from a sentence.


use std::collections::HashMap;

fn main(){
    
    let mut my_hash_map:HashMap<String,i32>= HashMap :: new();

    my_hash_map.insert(String::from("Om Sharma"), 423);
    my_hash_map.insert(String::from("Sagar Sharma"),321);
    my_hash_map.insert(String::from("Om Sharma"),406);

    // prints hashmaps
    println!("{:?}",my_hash_map);
    my_hash_map.remove("Om Sharma");
    println!("{:?}",my_hash_map);
    // methods 
    /*
    
    1. .get(key) --> Returns Value.
    2. .insert(key,value) --> Inserts the Key Value Pair into Hashmap.
    3. .remove(key) --> Removes Key Value Pair from Hashmaps.

     */


}