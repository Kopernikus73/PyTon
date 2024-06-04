use std::io::{stdout, Write};
use std::process::Command;
use std::env;
use std::fs;

fn main() {
    println!(
"\x1b[4m\x1b[1mWelcome to PyTon v0.2.0!\x1b[0m

You need to have Python installed for this software
You will learn the basics of python programming and be able to create your first really own python program!

You will have to edit a specific file (depending on the exercise)
All the files you will have to edit are located in the /files folder
Let's just start with the var1.py file
Open the file in your text beloved editor and look at the instructions

You will be operating in the PyTon console (this console)
Here you can run commands, such as `help` to show a short help message\n"
); 


    stages();
}

fn stages(){
    let mut stage:i32 = 1;
    while stage == 1{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("String Variables in Python are declared like following\ny = 'Hello, World!'")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        }
    }
    
    while stage == 2{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("int(integer) Variables in Python are declared like following\ny = 42")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        }
    }

    while stage == 3{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("float(floating-point) Variables in Python are declared like following\ny = 4.2")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        }
    }

    while stage == 4{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("bool(booleans) Variables in Python are declared like following\ny = True\nz = False")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        }
    }

    while stage == 5{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("Lists in Python are declared like following\ny = ['hello',2,True,4.2]")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        } else if output == "help 2"{
            help_specific("To get the value of a certain part of a list, use the following command\ngame_list = ['mineslaft','Fortbite']\nsecond_part_of_game_list = game_list[1]")
        } else if output == "help 3.1"{
            help_specific("Lists in Python are declared like following\ny = ['hello',2,True,4.2]")
        } else if output == "help 3.2"{
            help_specific("You can append python lists by using the following command\ngame_list = ['mineslaft','Fortbite']\ngame_list.append('Taster in Tasting Town')")
        } else if output == "help 4"{
            help_specific("To change the value of a certain part of a list, use the following command\ngame_list = ['mineslaft','Fortbite']\ngame_list[1] = 'Taster in Tasting Town'")
        }

    }

    while stage == 6{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("Tuples in Python are declared like following\ny = ('hello',2,True,4.2)")
        } else if output == "help 1"{
            help_specific("Variables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {x}')")
        } else if output == "help 2"{
            help_specific("To get the value of a certain part of a tuple, use the following command\ngame_tuple = ['mineslaft','Fortbite']\nsecond_part_of_game_tuple = game_list[1]")
        } else if output == "help 3"{
            help_specific("Tuples with parts of different tuples/lists in Python are declared like following\ny = ('hello',2,True,4.2)\nz = (y[0],y[5])")
        } else if output == "help 4"{
            help_specific("To change the value of a certain part of a tuple, use the following command\ngame_list = ('mineslaft','Fortbite')\ngame_list[1] = 'Taster in Tasting Town'")
        }
    }

    while stage == 7{
        let output = get_input();
        if output == "run"{
            stage = run(stage);
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            stdout().flush().unwrap();
        } else if output == "pwd" {
            pwd(stage)
        } else if output == "help"{
            help()
        } else if output == "help 0"{
            help_specific("Dictionaries in Python are declared like following\nheights = {'Mark':'1.8m','Jenna':'1.7m'}");
        } else if output == "help 1"{
            help_specific("To get a value that is assigned to another value in the dictionary, do the following\nheights = {'Mark':'1.8m','Jenna':'1.7m'}\nheight_of_Mark = heights['Mark']");
        } else if output == "help 2"{
            help_specific("To change a value that is assigned to another value in the dictionary, do the following\nheights = {'Mark':'1.8m','Jenna':'1.7m'}\nheights['Jenna'] = '1.6m'");
        } else if output == "help 3"{
            help_specific("To add a new value and a value that is assigned to it, to the dictionary, do the following\nheights = {'Mark':'1.8m','Jenna':'1.7m'}\nheights['Kyle'] = '1.9m'")
        } else if output == "help 3"{
            help_specific("To print a whole dictionary, just do the following\nheights = {'Mark':'1.8m','Jenna':'1.7m'}\nprint(heights)")
        }
    }

    if stage == 8 {
        println!("That's it for now in version 0.2.0")
    }
}


//* PyTon commands

fn clear_terminal() {
    let mut stdout = stdout();
    write!(stdout, "\x1B[2J").unwrap();
    write!(stdout, "\x1B[H").unwrap();
    write!(stdout, "\x1b[4m\x1b[1mPyTon v0.2.0\x1b[0m\n\n").unwrap()
}

fn help(){
    println!("\n\x1b[4mPyTon Commands\x1b[0m");
    println!("help → shows this help message");
    println!("help <part> → shows a help message for the chosen part of the exercise-file");
    println!("run → runs the exercise-file");
    println!("clear → clears the terminal output");
    println!("pwd → prints the working directory");
    println!("exit → exits PyTon\n")
}

fn help_specific(text:&str){
    println!("{}\n",text);
}

fn run(stage:i32) -> i32{
    let full_file_path = format!("files/var{}.py",&stage);

    match start_py_file(&full_file_path) {
        Ok(output) => {println!("{}", output);
        let string_var:String;
        match read_file(&full_file_path) {
            Ok(content) => {
                string_var = content;
            }
            Err(_) => {
                println!("Error reading file");
                string_var = format!("err_reading_file");
            }
        }
    
        let mut split_out: Vec<&str> = string_var.split('\n').collect();
        split_out.retain(|s|!s.contains('§'));


        if output == "Hello, Keith!\n" && split_out.iter().any(|word| word.contains("name")) && split_out.iter().any(|word| word.contains(" = ")) && split_out.iter().any(|word| word.contains("Keith"))
            {
            println!("Correct! Now move on to var{} and follow the instructions\n",stage+1);
            stage+1
        }else{
            stage 
        }},
        Err(e) => {
            eprintln!("Error: {}", e);
            stage
        }
    }
}

fn pwd(stage:i32){
    println!("files/var{}.py\n",stage)
}


//* General Functions

fn read_file(file_name: &str) -> Result<String,String> {
    let exe_path = env::current_exe().map_err(|e| format!("Failed to get the path of the current executable: {}", e))?;

    let parent_dir_of_exe = exe_path.parent().ok_or("The executable is not in a directory with a parent.")?;
    
   let file_path = parent_dir_of_exe.join(file_name);
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");



    Ok(contents)
}

fn get_input() -> String{
    let mut input = String::new();
    print!("> ");
    stdout().flush().unwrap();
    // Lesen der Benutzereingabe und Speichern im String-Objekt
    std::io::stdin().read_line(&mut input)
        .expect("Fehler beim Lesen der Eingabe");

    // Entfernen des Zeilenumbruchs am Ende der Eingabe
    let input = input.trim();
    input.to_string()
}

fn start_py_file(file_name: &str) -> Result<String, String> {
    // Erhalte den Pfad zum aktuellen Executable
    let exe_path = env::current_exe().map_err(|e| format!("Failed to get the path of the current executable: {}", e))?;

    // Erhalte das Elternverzeichnis des Executable
    let parent_dir_of_exe = exe_path.parent().ok_or("The executable is not in a directory with a parent.")?;

    // Erstelle den vollständigen Pfad zur Python-Datei
    let file_path = parent_dir_of_exe.join(file_name);

    // Führe das Python-Skript aus und fange die Ausgabe ab
    let output = Command::new("python3")
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Standardausgabe als String
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Fehlerausgabe als String
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        return Err(format!("Error from Python script:\n{}", stderr));
    }

    // Rückgabe der Standardausgabe
    Ok(stdout)
}