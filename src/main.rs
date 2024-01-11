use std::io::Write;
use std::fs::{File, OpenOptions};

mod file_read;

/*
* Current ToDo:
* Make it so that you can read from the txt file. 
* 
* */

fn see_items(items: &mut Vec<String>){
    if items.len() == 0 {
        println!("You should try inputting some items before viewing them.");
        return;
    }
    for (idx, val) in items.iter().enumerate() {
        println!("{}. {}", idx, val);
    }
}

fn remove_item(items: &mut Vec<String>){
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
fn add_item(items: &mut Vec<String>) {
    println!("What is the item that you would like to add?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    items.push(input);
}

fn save_items (items: &mut Vec<String>) -> std::io::Result<()> {
    println!("Please input the name of the file you would like to save, followed by .txt. For example: file.txt");
    
    let mut filename = String::new();

    std::io::stdin().read_line(&mut filename).expect("Failed to read line");

    let filepath = format!("../storage/{}", &filename.trim());

    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&filepath);

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

fn get_user_fn(items: &mut Vec<String>) -> bool {
    println!("What would you like to do: e = exit, c = check an item, a = add an item, s = save your current list, v = see current items");

    let mut check = true;
    while check {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");
           
        check = false;
        
        let t_input = input.trim();

        match t_input{
            "c" => remove_item(items),
            "a" => add_item(items),
            "s" => save_items(items).expect("Reason"),
            "v" => see_items(items),
            "e" => return false,
            _ => {
                check = true;
                println!("Please enter a valid option");
            }
        };
    }
    
    return true;
}

fn todo_list() {
    let mut items:Vec<String> = Vec::new();
    let mut cont:bool = true;
    file_read::load_file(&mut items);
    while cont {
        cont = get_user_fn(&mut items);
    }
}

fn main(){
    todo_list();
}
