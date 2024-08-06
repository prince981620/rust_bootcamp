// option enum to handle NULL in rust
fn find_first_string(s: String) -> Option<usize> {
    for(index , charater) in s.chars().enumerate() {
        if charater == 'a' {
            return Some(index);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("raman");
    let res = find_first_string(my_string);
    match res {
        Some(index) => println!("a is present at index : {}",index),
        None => println!("not present")
    }
}