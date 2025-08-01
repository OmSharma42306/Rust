use std::string;

fn main(){
    println!("Hello Rust!");
    let name:String = String :: from("Om Sharma");
    let mut name2 : String = name.clone();
    greet(name);
    let final_sum : i32 = sum(4,3);
    println!("Final Sum is : {}",final_sum);
    let final_length:usize= length(&name2);
    println!("Final Length of Name : {}",final_length)
}

fn greet(name : String){
    println!(" Hello Dear,{}",name);
}  

fn sum(num1:i32,num2:i32)->i32{
    return num1+num2;
}

fn length( s : &String)->usize{
    let s  = "om";
    return s.len();
}