struct User{
    first_name : String,
    last_name : String,
    age : u32
}



fn main(){
    println!("Function Calling");
    println!("{}",is_even(41));
    println!("{}",get_fibonacii_of_num(3));
    let my_sentence = String :: from("OM SHARMA LEARNING RUST!");
    let length:usize = get_string_length(&my_sentence);
    println!("Length {}",length);

    // struct stuff.
    let user_1 = User{
        first_name : String :: from("Om") ,
        last_name : String :: from("Sharma"),
        age : 21
    };
    println!("user_1 first name : {} ",user_1.first_name);
    println!("user_1 last name : {} ",user_1.last_name);
    println!("user_1 age is : {} ",user_1.age);

}   

fn is_even(num : i32) -> bool{
    if num%2 == 0 {
        return true;
    }else{
        return false;
    }

}


fn get_fibonacii_of_num(num:i32) -> i32{
    let mut first = 0;
    let mut second = 1;
    println!("{}",second);
    if num == 0 {
        return first;
    }
    if num == 1{
        return 1;
    }

    for _ in 0..num - 1{
        let temp = second;
        second = second+first;
        first = temp;
    }
    println!("{}",second);
    return second;
    

}


fn get_string_length(sentence : &str) -> usize{
    sentence.chars().count()
}