



fn main() {
    println!("Hello, world!");
    let x: i32 = 1;
    println!("{}",x);  

    // character in rust

    let greeting: String = String::from("Watch your tone Boi");
    println!("{}",greeting);

    let char1: Option<char> = greeting.chars().nth(0);
    match char1 {
        Some(c) => println!("{}",c),
        None => println!("No character in given index"),
    }

    let my_string = String::from("Hello"); 
    let some_stirng2 = &my_string; //there can be multiple non mutable references
    let some_stirng3 = &my_string; // or non mutable owners
    let some_stirng4 = &my_string;
    println!("{},{},{}",some_stirng2,some_stirng3,some_stirng4);
    
    borrow_variable(&my_string); // passing as unmutalbe  reference


    let mut my_string2 = String::from("Hello"); //mutable varable
    
    let s2 = &mut my_string2;

    // let s3 = &my_string2;
    
    s2.push_str(" boi");
    println!("{}",s2);
    println!("{}",my_string2);
    
    mutborrow_variable(&mut my_string2); //passing mutable references
    // println!("{}",s2); //if i uncomment it gives error
    println!("{}",my_string2);

    mutborrow_variable2(&mut my_string2);
    // println!("{}",s2);
    println!("{}",my_string2);

}

fn borrow_variable(some_string: &String) { //borrow as reference
    println!("{}",some_string);
}
fn mutborrow_variable(mutsome_string: &mut String){ //borrow as mutable references
    mutsome_string.push_str(" whole");
}
fn mutborrow_variable2 (mutsome_string2: &mut String){ //borrow as mutable references
    mutsome_string2.push_str(" world");
}

