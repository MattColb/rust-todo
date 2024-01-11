use std::io::{Write, BufRead}; 
use std::fs::File;
use std::fs;

fn get_file() -> File {
    println!("Please input the filename that you would like to load");
    loop{
        let mut file = String::new();
        std::io::stdout().flush().unwrap();
        print!("> ");

        std::io::stdin().read_line(&mut file).expect("Failed to read line");

        let filepath = format!("../storage/{}", file.trim());
        
        //Eventually add something that will collect the txt.
        if !fs::metadata(filepath.clone()).is_ok(){
            println!("Please enter a valid file."); 
        } else{
            println!("You entered a valid file. Now reading the contents... ");
            let file = match File::open(filepath) {
                Ok(file) => file,
                Err(e) => {
                    panic!("Error creating the file: {}", e);
                }
            };
            return file;
        }
   }
}



fn user_wants() -> bool {
    println!("Would you like to load your todo list from a previous session?: ");
    loop{
        let mut uin = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut uin).expect("Should have been able to read this line");
        
        match uin.trim(){
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

pub fn load_file(items: &mut Vec<String>){
    if !user_wants() {
        return;
    }
    
    let f = get_file();

    let reader = std::io::BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(content) => items.push(content + "\n"),
            Err(e) => {
                panic!("Error handling the reading from a line: {e}");
            }
        }
    }

    return;
}
