use std::io;

fn function(x: i32, y: i32){
    println!("You added {} times", x);
    println!("You removed {} times", y);
}

fn main() {
    //Rust is weird, but i kind of like it 
    println!("Please enter the name of the person using this list");
    
    println!("Your first name");
    //except for this, i don't like this input 
    let mut _first_name = String::new();
    io::stdin().read_line(&mut _first_name)
    .expect("Not a word");
    
    println!("Your last name");
    let mut _last_name = String::new();
    io::stdin().read_line(&mut _last_name)
    .expect("Not a word");
    
    let full_name = format!("{} {}, lets make your list", _first_name.trim(), _last_name.trim());
    println!("Hello {}", full_name);
    let mut todo_list : Vec<String> = Vec::new();
    let mut count = 0;
    let mut seperate_count = 0;
    loop{
        println!("Please select an option\n1.Add\n2.Remove\n3.Display\n4.End Program");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)
        .expect("Not a valid choice");
        let answer = answer.trim();
    //println!("You chose {}", answer.trim_end());
        match answer{
            "1" => {
                println!("Adding to your lits\n");
                println!("How many items do you want for your list?");
                let mut num_for_list = String::new();
                io::stdin().read_line(&mut num_for_list)
                    .expect("Not a valid choice");
                let num_items: usize = num_for_list.trim().parse().expect("Not a number");
                
                for i in 0..num_items{
                    println!("Enter your item {} ", i+1);
                    let mut item = String::new();
                    io::stdin().read_line(&mut item)
                    .expect("Failed to read line");
                    todo_list.push(item.trim().to_string());
                    count += 1;
                    //seperate_count += 1;
                    //println!("Count is {}", count);
                    //println!("{:?}", todo_list);
                }
            }
            "2" => {
                println!("Removing from your list");
                if count == 0{
                    println!("There's nothing in your list to remove");
                }else{
                println!("What item would you like to remove");
                //println!("{}",todo_list.len());
                while count == todo_list.len() {
                    let mut remove_answer = String::new();
                    io::stdin().read_line(&mut remove_answer).expect("Something");
                    
                    if todo_list.iter().any(|i| i == remove_answer.trim()){
                        todo_list.retain(|x| *x != remove_answer.trim());
                        //always remember to update your counter
                        count -= 1;
                        seperate_count += 1;
                    }else{
                        println!("Not in the list");
                    }
                    break;
                    }
                    
                }
            }
            "3" => {
                println!("Displaying your list");
                if count == 0{
                    println!("There's nothing to display in your list");
                }
                println!("{:?}", todo_list);
            }
            "4" => {
                println!("Thank you for using my program");
                function(count.try_into().unwrap(), seperate_count);
                break;
            }
            _ => println!("Not a valid choice"),
        }
    
    }
  
}