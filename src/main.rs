use std::env;
use std::fs;

struct PrivateMessage {}
struct NormalMessage{}

pub trait Message{
    fn send(&self, content: &str){
        println!("{}", content);
    }
}

impl Message for NormalMessage{}

impl Message for PrivateMessage{
    fn send(&self, content: &str){ //overiding
        println!("private: {}", content);
    }
}

// enumeration
enum Coin{
    Penny,
    Nickel
}


//pub fn factorial(n: int32){
//    if(n == 1) {
//        return 1;
//    } else {
//        return n * factorial(n-1);
//    }
//}

pub fn main(){
    let filename = env::args().nth(1).unwrap();

    let file_contents = fs::read_to_string(filename).expect("Error reading file {}");

    let number_of_words = file_contents.split_whitespace().count();

    println!("{}", number_of_words);
}
