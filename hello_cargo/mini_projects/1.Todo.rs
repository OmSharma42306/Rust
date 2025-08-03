// 📅 To-do List App (Struct + Vec + Functions)
#[derive(Debug)]

enum Status{
    Completed,
    InCompleted,
    Processing,
}

#[derive(Debug)]
struct Todo{
    id : usize,
    todo_name : String,
    status : Status
}


fn delete_todo(_todo_id : usize, v : &mut Vec<Todo> ){
    v.remove(_todo_id-1);
    // println!("Deleted Todo : {:?}",v);
}


fn add_todo( v : &mut Vec<Todo>,new:Todo){
    v.push(new);
    println!("Todo Has Been Added!");
    println!("{:?}",v);
}


fn main(){
    let mut v : Vec<Todo> = vec![];
    
    // let todo : String = String::from("Go to Gym!");
    // let status : Status = Status::InCompleted;
    let new_todo : Todo = Todo{id : 1, todo_name : String::from("Go to Gym!"),status:Status::InCompleted};
    let second_todo : Todo = Todo{id : 2, todo_name : String::from("Go to Lunch!"),status:Status::Completed};
    let third_todo : Todo = Todo{id : 3, todo_name : String::from("Go to Temple!"),status:Status::Processing};
    
    add_todo(&mut v, new_todo);
    add_todo(&mut v, second_todo);
    add_todo(&mut v, third_todo);
    println!("Added Todo : {:?}",v);
    delete_todo(1,&mut v);
    println!("Deleted Todo : {:?}",v);

}