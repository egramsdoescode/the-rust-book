fn main() {
    let mut temp = farenheit_to_celsius(54.0);

    println!("54 degrees farenheit to celsius: {temp}");

    temp = celsius_to_farenheit(temp);

    println!("Celsius temp back to farenheit: {temp}");
}

fn farenheit_to_celsius(temp: f64) -> f64 {
   (temp - 32.0) * (5.0/9.0) 
}

fn celsius_to_farenheit(temp: f64) -> f64 {
    (9.0/5.0) * temp + 32.0
}
