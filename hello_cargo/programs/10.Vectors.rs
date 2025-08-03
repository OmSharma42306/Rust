/*
Create a Vec<i32> and:

Add numbers to it

Remove the last number

Find the max number using iteration

*/

fn add_to_vector( mut v:Vec<i32>,num : i32)->Vec<i32>{
    v.push(num);
    println!("The Number : {} added to Vector.",num);
    return v;
}

fn remove_last_number(mut v:Vec<i32>)->Vec<i32>{
    v.pop();
    return v;
}

fn find_max_number(v:&Vec<i32>){
    let mut max : i32 = 0;
    for num in v{
        if(*num > max){
            max = *num;
        }
    }
    println!("The Max Number in Vector : {}",max);
}

fn main(){
    let mut v : Vec<i32> = vec![];
    v = add_to_vector(v, 4);
    v = add_to_vector(v, 5);
    v = remove_last_number(v);
    v = add_to_vector(v, 3);
    v = add_to_vector(v, 1);
    v = add_to_vector(v, 0);
    find_max_number(&v);
    println!("{:?}",v);

}