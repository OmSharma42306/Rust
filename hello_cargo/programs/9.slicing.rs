
// String Manipulation

 // (a) return even list

use std::fmt::format;

fn even() -> Vec<i32> {
    // let vowels = ['a','e','i','o','u'];
    // let vowels = vec!['a','e','i','o','u'];
    // return s.to_lowercase()
    // .chars()
    // .count()
    let numbers = vec![1,2,3,4,5,6,7,8,9,10];
    // filter methods
    let even : Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    return even;
}

 // (b) Count vowels in a string

fn count_vowels( s : &str)-> usize {
    
    let vowels : Vec<char> = vec!['a','e','o','i','u'];

    let x = s.to_lowercase()
    .chars()
    .filter(|c| vowels.contains(c))
    .count();

    return x;
    
    
}


// (c) Reverse a String manually.

fn reverse_string(s : &str){
    let mut result = String::new();
    for c in s.chars(){
        println!("c : {}",c);
        result = format!("{}{}",c,result);
        println!("T : {}",result)
    }
    println!("Final : {}",result);
    // return "";
}

 fn main(){
    // let count = count_vowels("Om Sharma");
    let e = even();
    println!("{:?}",e);
    let l = count_vowels("oeei");
    println!("{}",l);
    reverse_string("Hello");
}