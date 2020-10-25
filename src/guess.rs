use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() -> i32 {
  let stdin = io::stdin;
  let secret = rand::thread_rng().gen_range(1, 1001);

  loop {
    println!("Please choose a number.");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failure to read.");

    let input: i32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    match input.cmp(&secret) {
      Ordering::Less => println!("Too small."),
      Ordering::Greater => println!("Too big."),
      Ordering::Equal => {
        return input;
      }
    }
  }
}
