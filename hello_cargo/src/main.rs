// ownership concept and move

// ownership : one variable have ownership of it's value.
// after ending of program when variable is removed from stack 
// automatically the assigned value on the heap by that variable would be removed.


// move : moving variable value that stored in heap to pointing to new variable.

fn main(){
    
    let s1 = String::from("Om Sharma");
    
    print_string(s1);
    // so below s1 gives an error because when print_string function called
    // we send the s1 variable value to print_string function so there s3 is 
    // a new variable that assigned to that value so the value is moved...so below giving an error...
    println!("{}",s1);

}

fn print_string(sentence:String){
    println!("{}",sentence);
}

fn do_something(s3:String){
    println!("{}",s3);
}