fn main() {
    let x = 5;
    println!("the value of x is: {x}");

    // if expression
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let mut cnt = 0;
    'counting_up: loop {                    // loop labels must start with a single quote
        println!("count = {cnt}");
        let mut remaining = 10;

        loop { 
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;         // breaks the outer loop with the label
            }
            remaining -= 1;
        }
        
        cnt += 1;
    }

    println!("end count = {cnt}");

}
