// 🧮 Simple Calculator with input and pattern match on operator (+, -, *, /)
use std::io;

// Pattern Match
fn calculate(num1:i32,num2:i32,operator:String){
    match(operator,num1,num2){
        (_o,_x,_y) if _o == "+" => add(num1,num2),
        (_o,_x,_y) if _o == "-" => sub(num1, num2),
        (_o,_x,_y) if _o == "*" => multiply(num1, num2),
        (_o,_x,_y) if _o == "/" => divide(num1, num2),
        _ => println!("Operation Cannot be Done")
    }  
}

fn add(num1:i32,num2:i32){
    let  total : i32 = num1 + num2;
    println!("{} + {} is {}",num1,num2,total);

}

fn sub(num1:i32,num2:i32){
    let  total : i32 = num1 - num2;
    println!("{} - {} is {}",num1,num2,total);
}

fn multiply(num1:i32,num2:i32){
    let  total : i32 = num1 * num2;
    println!("{} X {} is {}",num1,num2,total);
}

fn divide(num1:i32,num2:i32){
    let  total : i32 = num1 / num2;
    println!("{} / {} is {}",num1,num2,total);
}

fn main(){
    
let mut num1 : String = String::new();
let mut num2 : String = String::new();

println!("Enter Your First Number ");
io::stdin().read_line(&mut num1).expect("Unable to Read your Input!");

println!("Enter Your Second Number ");
io::stdin().read_line(&mut num2).expect("Unable to Read your Input!");

let a : i32 = num1.trim().parse::<i32>().expect("please enter a Valid Number");
let b : i32 = num2.trim().parse::<i32>().expect("please enter a valid Number");

println!("{} {}",a,b);

calculate(a, b, String::from("+"));
calculate(a, b, String::from("-"));
calculate(a, b, String::from("*"));
calculate(a, b, String::from("/"));

}