struct Rect{
    width:i32,
    height:i32
}

impl Rect{
    // adding &self like in js this keyword
    fn area(&self) -> i32{
        self.width * self.height
    }
    
    // accecpting &self , with another variable you can use.
    fn perimeter(&self,num1:i32) -> i32{
        2*self.width*self.height+num1
    }
    
    // without &self you cannot use that method for a particular instance of struct.
    // if you want to use this function debug you just need to call the function with struct.
    // not instance of struct ..
    // example : Rect::debug(). this will work.
    // rect1.debug() ... this will give error.
    fn debug()->i32{
        return 1;
    }
}

fn main(){

let rect= Rect{
        width:20,
        height:30,
};

    println!("Rectangle Area : {}",rect.area());
    println!("Perimeter Area : {}",rect.perimeter(4));
    println!("Debug Function Use : {}",Rect::debug()); // you are using debug function
                                                       //  by Using struct not an instance of struct.
}