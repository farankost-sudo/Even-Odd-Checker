use std::io;

fn main() {
   loop{let mut number = String::new();

       io::stdin()
           .read_line(&mut number)
           .expect("Input error");

       let number: i32 = number.trim().parse().expect("Enter a number");
       println!("to end program type 0");
       if number == 0 {
           break;
       }
       else if number % 2 == 0 {
           println!("{number} is even");
       }
       else { println!("{number} is odd"); }}
}





















