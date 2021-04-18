use std::io;
fn main() {
    
    loop{
        

    println!("How old are you?");
    
    let mut age = String::new();
    
    io::stdin().read_line(&mut age)
        .expect("Failed to read line");
    
    
    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if age > 18 {

        println!("You can vote! You have been eligible to vote for {} years!", age-18);

    }
    else{

        println!("You cannot vote! You will be eligible to vote in {} years!", 18-age);
    
        }

break;
}

}
