fn main() {
    let number = 7;
    
    // This condition MUST be a Boolean value, not an integer
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Loops 
    
    let mut counter = 0; 
    // loop keyword tells rust to loop indefinitely until stopped manually
    loop {
        println!("again");
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}
