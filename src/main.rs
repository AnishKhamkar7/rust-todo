use  std::io::{stdout,stdin,Write};

fn main() {
    
    let mut  s = String::new();
    println!("Enter Some text");

    let _ = stdout().flush(); 
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    println!("Yout typed {}",s);
}  
