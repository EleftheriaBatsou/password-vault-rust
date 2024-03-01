mod pentry;

use crate::pentry::prompt; // print out stuff and accept values from the user
use crate::pentry::read_passwords_from_file; // print out everything that the file has
use crate::pentry::ServiceInfo;

fn clr() { // this function helps me clear everything out
    print!("{}[2J", 27 as char);
}
fn main() {
    clr();
    // you can also create your own askii -> https://www.ascii-art-generator.org/
    println!(
        "              _                                       _                                 "
    );
    println!("            /' `\\                      /'            ' )       )                   /'  /'");
    println!(
        "          /'     )                 --/'--             /      _/                  /'--/'--"
    );
    println!(
        "        /' (___,/'        ____     /'                /    _/~____              /'  /'   "
    );
    println!(
        "      /'   ;   /'    /  /'    )--/' /'    /         /  _/~ /'    )  /'    /  /'  /'     "
    );
    println!(
        "    /'    /' /'    /'  '---,   /' /'    /'         /_/~  /'    /' /'    /' /'  /'       "
    );
    println!(
        "(,/'     (_,(___,/(__(___,/   (__(___,/(__        /~    (___,/(__(___,/(__(__ (__       "
    );
    println!(
        "                                    /'                                                  "
    );
    println!(
        "                            /     /'                                                    "
    );
    println!(
        "                           (___,/'                                                      "
    );
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new(); // take the choice of the user
        std::io::stdin().read_line(&mut choice).unwrap(); // this will be avaliable to us with the mutable choice (string)
                                                               // then call the corresponding function
        match choice.trim() { // trim the choice, cause for example there might be spaces and match it with... 1, 2, 3, or 4 or invalid
            "1" => {
                clr();
                let entry = ServiceInfo::new( // ServiceInfo is a struct, you can find it in pentry.rs
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully.");
                entry.write_to_file(); // we write this function to ServiceInfo
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| { // services is going to be a vector
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
                        - Username : {} 
                        - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| { // this is the same as above, I need to get the services
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services {
                    if item.service.as_str() == search.as_str() { // ensure the search is a string and search...
                        println!(
                            "Service = {}
                            - Username : {} 
                            - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}