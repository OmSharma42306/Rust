fn main(){
    println!("Function Calling!");
    let my_sentence = String::from("Om Sharma");

    let length = get_string_length(&my_sentence);
    println!("The Length is : {}",length);
}

fn get_string_length(sentence : &str) -> usize {
    sentence.chars().count()
}