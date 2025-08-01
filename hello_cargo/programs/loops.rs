// Exersise : Print numbers 1 to 100 using a for loop.

use std::array;

fn print_numbers_from_1_100(){  
    for i in 1..101{
        println!("{}",i);
    }
}

// iterating from list for loops
fn iterate_fruits(){
let fruits = vec!["apple","banana","orange"];
for f in fruits{
    println!("I Love {}",f);
}
}

/*  FizzBuzz: For numbers 1 to 100:

Print "Fizz" if divisible by 3.

Print "Buzz" if divisible by 5.

Print "FizzBuzz" if divisible by both */

fn fizz_buzz(){
    for i in 1..101{
        if(i % 3 == 0){
            println!(" {} Fizz",i)
        }else if( i % 5 == 0){
            println!("{} Buzz",i)
        }else{
            println!("{} FizzBuzz",i)
        }
    }
}


// Reverse an Array Using While Loop

fn reverse_array(arr:[i32;10]){
    let mut size : i32 = 10;
    while size != 0 {
        println!("{}",size);
        size-=1;
    }
}


fn main(){
    print_numbers_from_1_100();
    iterate_fruits();
    fizz_buzz();
    let arr : [i32;10] = [1,2,3,4,5,6,7,8,9,10];
    reverse_array(arr);
}