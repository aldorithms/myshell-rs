use input_macro::input;
use std::{error::Error, process::{exit, Command}, collections::HashMap, fs::File, io::{BufReader, BufRead, BufWriter, Write}};

/// The main function of the MyShell program.
///
/// # Purpose
/// This is the entry point of the MyShell program, responsible for handling user input, executing commands,
/// and managing aliases and settings.
///
/// # Returns
/// This function returns a `Result<(), Box<dyn Error + 'static>>`. It returns `Ok(())` if the program runs successfully
/// and returns an `Err` containing an error message if any errors occur during execution.
//
/// ```
fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut shellname = format!("My Shell"); // Default shell name
    let mut terminator = format!(">"); // Default terminator
    let mut aliases: HashMap<String, String> = HashMap::new(); // Initialize alias HashMap
    let max_aliases = 10;

    loop {
        let input: String = {
            let input = input!("{}{} ", shellname, terminator); // Take input
            input.to_string()
        }; // Workaround to ensure input lives long enough.

        let inputs: Vec<_> = input
            .split_whitespace() // Split inputs by the empty spaces between them.
            .collect(); // Collects split elements into Vector.

            if let Err(e) = match_inputs(&inputs, &mut shellname, &mut terminator, &mut aliases, max_aliases) {
                eprintln!("Error: {}", e);
            }
    } // End of Shell's loop
}

/// Matches and handles user input commands.
///
/// # Purpose
/// This function is responsible for matching and handling user input commands. It performs actions
/// based on the provided input, such as setting the shell name, changing the terminator, managing aliases,
/// and executing commands.
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words.
/// - `shellname`: A mutable reference to a `String` containing the name of the shell.
/// - `terminator`: A mutable reference to a `String` containing the current terminator.
/// - `aliases`: A mutable reference to a `HashMap<String, String>` containing user-defined aliases.
/// - `max_aliases`: An `usize` indicating the maximum number of aliases allowed.
///
/// # Returns
/// This function returns a `Result<(), Box<dyn Error>>`. It returns `Ok(())` if the command is executed
/// successfully and returns an `Err` containing an error message if any errors occur during execution.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::match_inputs;
///
/// let mut shellname = "My Shell".to_string();
/// let mut terminator = ">".to_string();
/// let mut aliases = HashMap::new();
///
/// let inputs = vec!["SETSHELLNAME", "Custom Shell Name"];
///
/// if let Err(e) = match_inputs(&inputs, &mut shellname, &mut terminator, &mut aliases, 10) {
///     eprintln!("Error: {}", e);
/// }
/// ```
fn match_inputs(inputs: &[&str],shellname: &mut String,terminator: &mut String,aliases: &mut HashMap<String, String>,max_aliases: usize) -> Result<(), Box<dyn Error>> {
    match inputs.get(0) {
        Some(&"STOP") 
            => exit(0),
        Some(&"SETSHELLNAME") 
            => set_shell_name(inputs, shellname),
        Some(&"SETTERMINATOR") 
            =>  set_terminator(inputs, terminator),
        Some(&"NEWNAME") 
            => set_new_name(inputs, aliases),
        Some(&"READNEWNAMES") 
            => read_new_names(inputs, aliases, max_aliases),
        Some(&"LISTNEWNAMES") 
            => list_new_names(aliases),
        Some(&"SAVENEWNAMES") 
            => save_new_names(inputs, aliases),
        Some(command) => {
            if let Some(alias_command) = aliases.get(command.to_string().as_str()) {
                // Execute the alias command if it exists
                let alias_args: Vec<&str> = alias_command.split_whitespace().collect();
                if let Err(e) = execute_command(alias_args[0], &alias_args[1..]) {
                    eprintln!("Error executing alias command: {}", e);
                }
            } else {
                if let Err(e) = execute_command(command, &inputs[1..]) {
                    eprintln!("Error executing command: {}", e);
                }
            }
        }
        None => {}
    }

    Ok(())
}

/// Sets the name of the shell.
///
/// # Purpose
/// This function sets the name of the shell to a new value based on user input. It collects and joins
/// the input words into a single string and updates the `shellname` reference accordingly.
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words, where the
///   first word is the command (`SETSHELLNAME`) and the rest are the words for the new shell name.
/// - `shellname`: A mutable reference to a `String` containing the current name of the shell.
///
/// # Returns
/// This function does not return any value. It updates the `shellname` reference in-place.
///
/// # Examples
/// ```
/// use myshell::set_shell_name;
///
/// let mut shellname = "My Shell".to_string();
/// let inputs = vec!["SETSHELLNAME", "Custom Shell Name"];
///
/// set_shell_name(&inputs, &mut shellname);
///
/// assert_eq!(shellname, "Custom Shell Name");
/// ```
fn set_shell_name(inputs: &[&str], shellname: &mut String) {
    let new_name = inputs
        .iter()
        .skip(1)
        .map(|&s| s)
        .collect::<Vec<_>>()
        .join(" ")
        .to_string();
        *shellname = new_name;
    println!("Shell name set to: {}", shellname);
}

/// Sets the terminator for the shell.
///
/// # Purpose
/// This function sets the terminator for the shell based on user input. If a new terminator is provided,
/// it updates the `terminator` reference with the new value. If no terminator is specified in the input,
/// it keeps the current terminator as is.
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words, where the
///   first word is the command (`SETTERMINATOR`) and the second word (if present) is the new terminator.
/// - `terminator`: A mutable reference to a `String` containing the current terminator for the shell.
///
/// # Returns
/// This function does not return any value. It updates the `terminator` reference in-place and prints
/// a message indicating the new terminator value.
///
/// # Examples
/// ```
/// use myshell::set_terminator;
///
/// let mut terminator = ">".to_string();
/// let inputs = vec!["SETTERMINATOR", "<"];
///
/// set_terminator(&inputs, &mut terminator);
///
/// assert_eq!(terminator, "<");
/// ```
fn set_terminator(inputs: &[&str], terminator: &mut String) {
    if let Some(new_terminator) = inputs.get(1) {
        *terminator = new_terminator.to_string();
        println!("Terminator set to: {}", terminator);
    } else {
        println!("No terminator specified. Using the default terminator: {}", terminator);
    }
}

/// Manages the alias list.
///
/// # Purpose
/// This function manages the alias list based on user input. It can perform three different operations:
///
/// 1. If no arguments are provided, it prints the current alias list.
/// 2. If one argument is provided, it deletes the alias with the given name if it exists.
/// 3. If two arguments are provided, it defines or updates an alias with the first argument as the new alias name
///    and the second argument as the command associated with the alias.
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words, where the
///   first word is the command (`NEWNAME`) and the rest are arguments.
/// - `aliases`: A mutable reference to a `HashMap<String, String>` containing user-defined aliases.
///
/// # Returns
/// This function does not return any value. It manages the `aliases` reference in-place and prints messages
/// to indicate the result of the operation.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::set_new_name;
///
/// let mut aliases = HashMap::new();
/// let inputs = vec!["NEWNAME", "myalias", "mycommand"];
///
/// set_new_name(&inputs, &mut aliases);
///
/// assert_eq!(aliases.get("myalias"), Some(&"mycommand".to_string()));
/// ```
fn set_new_name(inputs: &[&str], aliases: &mut HashMap<String, String>) {
    if inputs.len() == 1 {
        // No arguments provided, print the alias list
        println!("Aliases:");
        for (alias, command) in aliases {
            println!("{} - {}", alias, command);
        }
    } else if inputs.len() == 2 {
        // Delete the alias if it exists
        let alias_to_delete = inputs[1];
        if aliases.contains_key(alias_to_delete) {
            aliases.remove(alias_to_delete);
            println!("Alias '{}' deleted.", alias_to_delete);
        } else {
            println!("Alias '{}' does not exist.", alias_to_delete);
        }
    } else if inputs.len() == 3 {
        // Create or update an alias
        let new_alias = inputs[1];
        let old_command = inputs[2];
        aliases.insert(new_alias.to_string(), old_command.to_string());
        println!("Alias '{}' defined for '{}'.", new_alias, old_command);
    } else {
        println!("Invalid usage of NEWNAME command.");
    }
}

/// Reads aliases from a file and populates the alias list.
///
/// # Purpose
/// This function reads aliases from a specified file and populates the `aliases` map with the aliases
/// found in the file, up to the specified maximum number of aliases (`max_aliases`).
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words, where the
///   first word is the command (`READNEWNAMES`) and the second word is the name of the file to read
///   aliases from.
/// - `aliases`: A mutable reference to a `HashMap<String, String>` containing user-defined aliases.
/// - `max_aliases`: An `usize` indicating the maximum number of aliases allowed.
///
/// # Returns
/// This function does not return any value. It populates the `aliases` map with aliases read from the file
/// and handles any errors that may occur during file reading.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::{read_new_names, read_aliases_from_file};
///
/// let mut aliases = HashMap::new();
/// let inputs = vec!["READNEWNAMES", "aliases.txt"];
/// let max_aliases = 10;
///
/// read_new_names(&inputs, &mut aliases, max_aliases);
/// ```
fn read_new_names(inputs: &[&str], aliases: &mut HashMap<String, String>, max_aliases: usize) {
    if inputs.len() != 2 {
        println!("Usage: READNEWNAMES <file_name>");
        return;
    }

    let file_name = inputs[1];
    if let Err(e) = read_aliases_from_file(file_name, aliases, max_aliases) {
        eprintln!("Error reading aliases from file: {}", e);
    }
}

/// Lists all the aliases that have been defined.
///
/// # Purpose
/// This function lists all the aliases that have been defined and stored in the `aliases` map.
///
/// # Parameters
/// - `aliases`: A reference to a `HashMap<String, String>` containing user-defined aliases.
///
/// # Returns
/// This function does not return any value. It prints the list of aliases.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::list_new_names;
///
/// let mut aliases = HashMap::new();
/// aliases.insert("mycd".to_string(), "cd".to_string());
/// aliases.insert("mycopy".to_string(), "cp".to_string());
///
/// list_new_names(&aliases);
/// ```
fn list_new_names(aliases: &HashMap<String, String>) {
    println!("Aliases:");
    for (alias, command) in aliases {
        println!("{} - {}", alias, command);
    }
}

/// Handles the SAVENEWNAMES command.
///
/// # Purpose
/// This function handles the SAVENEWNAMES command, which saves the aliases stored in the `aliases` map to a file.
///
/// # Parameters
/// - `inputs`: A slice of `&str` representing the user input split into individual words, where the
///   first word is the command (`SAVENEWNAMES`) and the second word is the name of the file to save
///   aliases to.
/// - `aliases`: A reference to a `HashMap<String, String>` containing user-defined aliases.
///
/// # Returns
/// This function does not return any value. It prints a message to indicate the result of the operation
/// and handles any errors that may occur during file writing.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::save_new_names;
///
/// let mut aliases = HashMap::new();
/// aliases.insert("mycd".to_string(), "cd".to_string());
/// aliases.insert("mycopy".to_string(), "cp".to_string());
///
/// let inputs = vec!["SAVENEWNAMES", "aliases.txt"];
///
/// save_new_names(&inputs, &aliases);
/// ```
fn save_new_names(inputs: &[&str], aliases: &HashMap<String, String>) {
    if inputs.len() != 2 {
        println!("Usage: SAVENEWNAMES <file_name>");
        return;
    }

    let file_name = inputs[1];
    if let Err(e) = save_aliases_to_file(file_name, aliases) {
        eprintln!("Error saving aliases to file: {}", e);
    } else {
        println!("Aliases saved to file: {}", file_name);
    }
}

/// Executes a command with the specified arguments.
///
/// # Purpose
/// This function executes a command with the provided arguments using the `std::process::Command` struct.
///
/// # Parameters
/// - `command`: A `&str` representing the command to be executed.
/// - `args`: A slice of `&str` representing the arguments to be passed to the command.
///
/// # Returns
/// This function returns a `Result<(), Box<dyn Error>>`. It returns `Ok(())` if the command is executed
/// successfully and returns an `Err` containing an error message if the command returns a non-zero exit status.
///
/// # Examples
/// ```
/// use myshell::execute_command;
///
/// if let Err(e) = execute_command("ls", &["-l"]) {
///     eprintln!("Error: {}", e);
/// }
/// ```
fn execute_command(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new(command)
        .args(args)
        .status()?;
    
    if status.success() {
        Ok(()) // Program ended successfully.
    } else {
        Err(format!("Command '{}' returned a non-zero exit status", command).into()) // Return error that command ended with.
    }
}

/// Reads aliases from a file and populates a HashMap.
///
/// # Purpose
/// This function reads aliases from a specified file and populates a mutable `HashMap<String, String>`
/// with the alias-command pairs found in the file, up to a maximum specified by `max_aliases`.
///
/// # Parameters
/// - `file_name`: A `&str` representing the name of the file from which to read aliases.
/// - `aliases`: A mutable reference to a `HashMap<String, String>` that will store the aliases.
/// - `max_aliases`: An `usize` indicating the maximum number of aliases to read from the file.
///
/// # Errors
/// This function returns a `Result<(), Box<dyn Error>>`. It can return an error if there are issues
/// with file reading or parsing.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::read_aliases_from_file;
///
/// let mut aliases = HashMap::new();
///
/// if let Err(e) = read_aliases_from_file("aliases.txt", &mut aliases, 10) {
///     eprintln!("Error: {}", e);
/// }
/// ```
fn read_aliases_from_file(file_name: &str, aliases: &mut HashMap<String, String>, max_aliases: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, ' ').collect();

        if parts.len() == 2 {
            let alias = parts[0];
            let command = parts[1];
            aliases.insert(alias.to_string(), command.to_string());

            if aliases.len() >= max_aliases {
                break;
            }
        }
    }

    Ok(())
}
/// Saves the aliases to a file.
///
/// # Purpose
/// This function is responsible for saving a HashMap of aliases to a specified file.
///
/// # Parameters
/// - `file_name`: A `&str` representing the name of the file where the aliases will be saved.
/// - `aliases`: A reference to a `HashMap<String, String>` containing the aliases to be saved.
///
/// # Errors
/// This function returns a `Result<(), Box<dyn Error>>`. It can return an error if there are issues
/// with file creation or writing to the file.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use myshell::save_aliases_to_file;
///
/// let mut aliases = HashMap::new();
/// aliases.insert("myalias".to_string(), "ls -l".to_string());
///
/// if let Err(e) = save_aliases_to_file("aliases.txt", &aliases) {
///     eprintln!("Error: {}", e);
/// }
/// ```
fn save_aliases_to_file(file_name: &str, aliases: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut writer = BufWriter::new(file);
    for (alias, command) in aliases {
        match writeln!(writer, "{} {}", alias, command) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                break;
            }
        }

    }
    Ok(())
}