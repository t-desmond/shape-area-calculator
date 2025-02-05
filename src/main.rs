mod shapes;
use std::io::{self, Write};

use shapes::{Area, Cicle, Rectangle};
fn main() {
    loop{ 
        print!("select shape to calculate: [1. circle | 2. rectangle | 3. stop]:  ");
        io::stdout().flush().unwrap();
        let mut shape_choice = String::new();
        io::stdin().read_line(&mut shape_choice).expect("failed to read choice");

        match shape_choice.trim() {
            "1" => {
                  let mut radius = String::new();
                    print!("Enter the raduis of the cicle: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut radius).expect("failed to read");
                    let radius: f64 = match radius.trim().parse(){
                        Ok(r) => r,
                        Err(_) => {
                            println!("enter valied raduis");
                            return;
                        }
                    };
                    println!("The area of the circle is: {}", Cicle::new(radius).area());
            }
            "2" => {
                    let mut width = String::new();
                    print!("Enter the width of the rectangle: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut width).expect("failed to read");
                    let width: f64 = match width.trim().parse(){
                        Ok(r) => r,
                        Err(_) => {
                            println!("enter valied raduis");
                            return;
                        }
                    };
                    let mut height = String::new();
                    print!("Enter the height of the rectangle: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut height).expect("failed to read");
                    let height: f64 = match height.trim().parse(){
                        Ok(r) => r,
                        Err(_) => {
                            println!("enter valied raduis");
                            return;
                        }
                    };
                    println!("the area of the rectangle is {}", Rectangle::new(width, height).area());
                }
            "3" => break,
            _ => {
                println!("invalied choice");
                continue;
            }
        }
      
    }
}
