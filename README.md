## Password vault in Rust
_A CLI project_

#### main.rs:
**Module Declaration and Imports:**
- The main.rs file declares a module named pentry.
- It imports functions and types from the pentry module, including prompt, read_passwords_from_file, and ServiceInfo.
- Clear Screen Function (clr):

- The clr function is a helper function that clears the terminal screen using ANSI escape sequences.

**Main Function:**
- The main function serves as the entry point of the program.
- It clears the screen using clr.
- It prints an ASCII art representing the program's title or logo.
- Inside a loop, it presents a menu to the user, prompting for input and executing corresponding actions based on the user's choice.

**Menu Choices:**
For each menu choice:
- If the user chooses to add an entry ("1"), it prompts the user for service, username, and password, creates a ServiceInfo object, and writes it to the file.
- If the user chooses to list entries ("2"), it reads password entries from the file and prints them to the console.
- If the user chooses to search ("3"), it reads password entries from the file, prompts the user for a search term, and prints matching entries to the console.
- If the user chooses to quit ("4"), it prints a goodbye message and breaks out of the loop.

#### pentry.rs:

**ServiceInfo Struct:**
- The ServiceInfo struct represents a password entry with fields for service, username, and password.
- It implements methods to create a new entry, deserialize from JSON, serialize to JSON, and write to a file.

**File Operations:**
- The write_to_file method writes the ServiceInfo object to a file named passwords.json.
- The read_passwords_from_file function reads password entries from the passwords.json file into a vector of ServiceInfo objects.
- The prompt function prompts the user for input and returns the entered string.
- Overall, the program provides a basic command-line interface for managing a password vault, allowing users to add, list, search, and quit operations.

  #### Inspiration: [Akhil Sharma]([url](https://github.com/AkhilSharma90)https://github.com/AkhilSharma90)
