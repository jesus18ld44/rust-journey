fn main() {

    let mut s = String::from("jesus");

    s.push_str(" arevalo");
    println!("{}", s);

    // moves
    let x = 5;
    let y = x;      // make a copy of the value in x and bind it to y

    println!("x: {x}");

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1);
    // returns an error because s1 has been moved to s2
    
    let s3 = s2.clone();
    println!("{s3}");

    // passing a variable to a function will move or copy 
    // takes_ownership(s3);
    let mut s4 = takes_and_gives_ownership(s3);
    
    // println!("{}", s3);
    println!("{}", s4);

    change(&mut s4, &String::from(" world"));
    println!("{s4}");

} // the scope is now over, and s is no longer valid. It is dropped

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}
    
fn change(original_string: &mut String, to_add: &str) {
    original_string.push_str(to_add);

}
