use std::io::Write;
use std::fs::OpenOptions;


pub fn see_items(items: &mut Vec<String>){
    if items.len() == 0 {
        println!("You should try inputting some items before viewing them.");
        return;
    }
    for (idx, val) in items.iter().enumerate() {
        println!("{}. {}", idx, val);
    }
}

pub fn remove_item(items: &mut Vec<String>){
    if items.len() == 0 {
        println!("You should try inputting some items before removing them.");
        return;
    }
    println!("Type the line number that you would like to remove");
    see_items(items);
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<i32>(){
            Ok(num) => {
                if num >= items.len().try_into().unwrap() {
                    println!("Please input a number in the list.");
                }
                else {
                    items.remove(num.try_into().unwrap());
                    println!("Way to go!");
                    break
                }
            },
            Err(_) => {
                println!("Please input a valid number.");
            }
        };
    }
}

//FINISHED
pub fn add_item(items: &mut Vec<String>) {
    println!("What is the item that you would like to add?");
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    items.push(input);
}

pub fn save_items (items: &mut Vec<String>) -> std::io::Result<()> {
    println!("Please input the name of the file you would like to save, followed by .txt. For example: file.txt");
    
    let mut filename = String::new();

    std::io::stdin().read_line(&mut filename).expect("Failed to read line");

    let filepath = format!("../storage/{}", &filename.trim());

    let file = OpenOptions::new().write(true).create(true).truncate(true).open(&filepath);

    match file{
        Ok(mut file) => {
            for s in items {
                file.write_all(s.as_bytes())?;
            }
            println!("Your file has been saved in the storage folder of this project. You may now safely exit.");
        }
        Err(e) => {
            println!("Error opening the file: {}", e);
        }
    }

    Ok(())
}


