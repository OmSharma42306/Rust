
/* Task: Match on a Tuple (i32, i32) and print:
Both Positive

One Positive

Both Negative

Else case  */

// It's Just like Switch Statement.

fn analyze_numbers( a : i32 , b : i32){
    match(a,b){
        (x,y) if x > 0 && y > 0 => println!("Both Are Positive!"),
        (x,y) if x > 0 && y < 0 => println!("A is Positive B is Negative!"),
        (x,y) if x < 0 && y > 0 => println!("A is Negative B is Positive!"),
        _ => println!("Invalid Stuff")
    }
}


fn main(){
    analyze_numbers(1, -1);
    analyze_numbers(19, 12);
    analyze_numbers(-1, 31);
}