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



// other ways
/*Define a struct called User with fields: username: String, email: String, age: u8.

Create a function print_user(user: &User) that prints user details.

Add a method on User called is_adult() that returns true if age ≥ 18.*/

struct user{
    username : String,
    email : String,
    age : i32
}

fn main(){
    let user1 = user{
        username : String :: from("Om Sharma"),
        email : String :: from("omsharma.83173@gmail.com"),
        age : 20
    };

    print_struct(user1);
    
}

fn print_struct(userx : user){
    println!("Username :  {}",userx.username);
    println!("Email : {}",userx.email);
    println!("Age : {}",userx.age);
}



// another way
/*Define a struct called User with fields: username: String, email: String, age: u8.

Create a function print_user(user: &User) that prints user details.

Add a method on User called is_adult() that returns true if age ≥ 18.*/

struct user{
    username : String,
    email : String,
    age : i32
}

// to implement method to an struct use `impl`

impl user{
    fn is_adult(&self)->bool{
        if(self.age > 18){
            return true;
        }else{
            return false;
        }
    }
}

fn main(){
    let user1 = user{
        username : String :: from("Om Sharma"),
        email : String :: from("omsharma.83173@gmail.com"),
        age : 20
    };

    print_struct(user1);
    
    
}

fn print_struct(userx : user){
    println!("Username :  {}",userx.username);
    println!("Email : {}",userx.email);
    println!("Age : {}",userx.age);
    
    println!("Is Adult ? {}",userx.is_adult())
}