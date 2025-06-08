struct User{
    user_name : String,
    last_name : String, 
    age : u32
}

fn main(){
    let new_user = User{
        user_name : String :: from("Om"),
        last_name : String :: from("Sharma"),
        age : 34
    };

    println!("new User First Name : {}",new_user.first_name);
    println!("new User last Name : {} ",new_user.last_name);
    println!("new User age is : {} ",new_user.age);
}

