mod guess;

fn main() {
    let result: i32 = guess::guess();
    println!("Congratulations, winning pick was {}", result);
}
