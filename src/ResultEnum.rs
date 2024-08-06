use std::fs;
// try catch like block for error resolve: using Result Enum
fn main(){
    // there is a function that error out/stop the thread
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content) => {
            println!("file content: {}",content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    println!("after the error");
}