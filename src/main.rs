use std::io::Write;

mod file_read;
mod todo_list_actions;

fn get_user_fn(items: &mut Vec<String>) -> bool {
    println!("What would you like to do: \n e = exit \n c = check an item \n a = add an item \n s = save your current list \n v = see current items");

    let mut check = true;
    while check {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");
           
        check = false;
        
        let t_input = input.trim();

        match t_input{
            "c" => todo_list_actions::remove_item(items),
            "a" => todo_list_actions::add_item(items),
            "s" => todo_list_actions::save_items(items).expect("Reason"),
            "v" => todo_list_actions::see_items(items),
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
