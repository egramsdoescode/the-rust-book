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

    counter = 0;

    // Returning values from the loops can be done using break as well
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}"); // Result is 20
    
    // Loop labels
    counter = 0;

    'counting_up: loop {
        println!("Count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {counter}");

    // While loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!!");

    // For loops

    // Loopling through a collection:

    let a = [10, 20, 30, 40 ,50];
    
    for element in a {
        println!("The value is: {element}");
    }

    // Counting a range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
