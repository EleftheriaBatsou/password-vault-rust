use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {  
            service, 
            username, 
            password,
        }
    }
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> { // accepts json string and uses serde json
        serde_json::from_str(json_string) // this functon is to read from json
    }
    #[allow(dead_code)] 
    pub fn from_user_input() -> Self { // here is where we accept the values from the user
        println!("Enter Password Entry:");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service) //accept user's input as a service
            .expect("Failed to read line");              // if there is any error print it

        println!("Enter Username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        println!("Enter Password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    fn to_json(&self) -> String { // this functon is to read to json (this is similar to the function we wrote above)
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json") // the name of the file is passwords.json
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) { // here is where we're actually writing to the file
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);

    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }

    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt); // print the prompt we received, which is a string
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}