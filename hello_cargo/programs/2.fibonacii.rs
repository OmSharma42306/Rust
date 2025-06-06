fn main(){
    println!("Function Calling");
    println!("{}",get_fibonacii_of_num(35));
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

    for _ in 1..num - 2{
        let temp = second;
        second = second+first;
        first = temp;
    }
    println!("{}",second);
    return second;
    

}