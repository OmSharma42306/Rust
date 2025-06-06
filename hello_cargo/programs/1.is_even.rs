fn main(){
    println!("Function Calling");
    println!("{}",is_even(41));
    println!("{}",get_fibonacii_of_num(35));
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

    for i in 1..num - 2{
        let temp = second;
        second = second+first;
        first = temp;
    }
    println!("{}",second);
    return second;
    

}