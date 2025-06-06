fn main(){
    println!("Function Calling");
    println!("{}",is_even(41));
}

fn is_even(num : i32) -> bool{
    if num%2 == 0 {
        return true;
    }else{
        return false;
    }

}
