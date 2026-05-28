use std::io;

fn main() {
    loop {
    println!("Please enter a tempature in fahrenheit: ");
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //(32°F − 32) × 5/9 = 0°C
        let celsius = ((temp -32.0) * (5.0/9.0)).round();
    println!("The tempature in celsius is: {celsius}");
    println!("Do you want to enter another temature?(Y/N)");
        let mut test = String::new();

        io::stdin()
            .read_line(&mut test)
            .expect("Failed to read line");
        let test: String = test.trim().to_lowercase();

    if test == "n" {
        break;
        }   
    }
}
