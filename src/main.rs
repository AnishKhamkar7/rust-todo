use  std::io::{stdout,stdin,Write};

static mut todos:Vec<String> = Vec::new(); 

fn addTodos(){

}

fn deleteTodo(){

}

fn doneTodo(){
    
}



fn main() {
    
    let mut  s = String::new();
    let _ = stdout().flush(); 
    stdin().read_line( &mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    println!("Yout typed {}",s);
}  
