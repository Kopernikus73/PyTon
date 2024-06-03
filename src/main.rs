use std::io::{stdout, Write};
use std::process::Command;
use std::env;
use std::fs;

fn main() {
    println!(
"\x1b[4mWelcome to PyTon v0.1.5!\x1b[0m

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
    let mut stage:i8 = 1;
    while stage == 1{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var1.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var1.py") {
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
                    println!("Correct! Now move on to var2.py and follow the instructions\n");
                    stage = 2;
                }},
                Err(e) => eprintln!("Error: {}", e)
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var1.py")
        } else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("clear → clears the terminal output");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nString Variables in Python are declared like following\ny = 'Hello, World!'\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        }
    }
    
    while stage == 2{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var2.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var2.py") {
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


                if output == "Value : 5\n" && split_out.iter().any(|word| word.contains("Value")) && split_out.iter().any(|word| word.contains("=")) && split_out.iter().any(|word| word.contains("5")){
                    println!("Correct! Now move on to var3.py and follow the instructions\n");
                    stage = 3;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var2.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("clear → clears the terminal output");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nint(integer) Variables in Python are declared like following\ny = 42\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        }
    }

    while stage == 3{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var3.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var3.py") {
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

                if output == "My height in Meter is : 1.7\n" && split_out.iter().any(|word| word.contains("height")) && split_out.iter().any(|word| word.contains("=")) && split_out.iter().any(|word| word.contains("1.7")){
                    println!("Correct! Now move on to var4.py and follow the instructions\n");
                    stage = 4;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var3.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("clear → clears the terminal output");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nfloat(floating-point) Variables in Python are declared like following\ny = 4.2\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        }
    }

    while stage == 4{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var4.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var4.py") {
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

                if output == "Is True : True\n" && split_out.iter().any(|word| word.contains("status")) && split_out.iter().any(|word| word.contains("=")) && split_out.iter().any(|word| word.contains("True")){
                    println!("Correct! Now move on to var5.py and follow the instructions\n");
                    stage = 5;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var4.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("clear → clears the terminal output");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nbool(booleans) Variables in Python are declared like following\ny = True\nz = False\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        }
    }

    while stage == 5{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var5.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var5.py") {
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

                if output == "The 4 names are : ['Peter', 'Lisa', 'Bob', 'Marie']\nThe 2nd name is : Lisa\nThe first name and the last name are : ['Peter', 'Marie']\nThe 4 names are : ['Peter', 'Lisa', 'Paul', 'Marie']\n"{
                    println!("Correct! Now move on to var6.py and follow the instructions\n");
                    stage = 6;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var5.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("clear → clears the terminal output");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nLists in Python are declared like following\ny = ['hello',2,True,4.2]\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        } else if output == "help 2"{
            println!("\nTo get the value of a certain part of a list, use the following command\ngame_list = ['mineslaft','Fortbite']\nsecond_part_of_game_list = game_list[1]\n")
        } else if output == "help 3.1"{
            println!("\nLists in Python are declared like following\ny = ['hello',2,True,4.2]\n")
        } else if output == "help 3.2"{
            println!("\nYou can append python lists by using the following command\ngame_list = ['mineslaft','Fortbite']\ngame_list.append('Taster in Tasting Town')\n")
        } else if output == "help 4"{
            println!("\nTo change the value of a certain part of a list, use the following command\ngame_list = ['mineslaft','Fortbite']\ngame_list[1] = 'Taster in Tasting Town'\n")
        }

    }

    while stage == 6{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var6.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var6.py") {
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

                if output == "The 3 foods are : ('apple','orange','lemon')\nThe 2nd food is : orange\nThe first food and the last food are : ('apple','lemon')\nThe 3 foods are : ('apple','orange','banana')\n"{
                    println!("Correct! Now move on to var5.py and follow the instructions\n");
                    stage = 7;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var6.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nTuples in Python are declared like following\ny = ('hello',2,True,4.2)\n")
        } else if output == "help 1"{
            println!("\nVariables are implemented into Strings/print statements like following\nx = 'text :p'\nprint(f'Output: {}')\n","{x}")
        } else if output == "help 2"{
            println!("\nTo get the value of a certain part of a tuple, use the following command\ngame_tuple = ['mineslaft','Fortbite']\nsecond_part_of_game_tuple = game_list[1]\n")
        } else if output == "help 3"{
            println!("\nTuples with parts of different tuples/lists in Python are declared like following\ny = ('hello',2,True,4.2)\nz = (y[0],y[5])\n")
        } else if output == "help 4"{
            println!("\nTo change the value of a certain part of a tuple, use the following command\ngame_list = ('mineslaft','Fortbite')\ngame_list[1] = 'Taster in Tasting Town'\n")
        }
    }

    while stage == 7{
        let output = get_input();
        if output == "run"{
            match start_py_file("files/var7.py") {
                Ok(output) => {println!("{}", output);
                let string_var:String;
                match read_file("files/var7.py") {
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

                if output == "New York\nSan Diego\nEarth\n{'Country': 'USA', 'City': 'San Diego', 'Planet': 'Earth'}\n"{
                    println!("Correct!\n");
                    stage = 8;
                }},
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if output == "exit" {
            break
        } else if output == "clear" {
            clear_terminal();
            println!("\x1b[4mPyTon v0.1.5\x1b[0m\n");
            stdout().flush().unwrap();
        } else if output == "pwd" {
            println!("files/var7.py")
        }else if output == "help"{
            println!("\n\x1b[4mPyTon Commands\x1b[0m");
            println!("help → shows this help message");
            println!("help <part> → shows a help message for the chosen part of the exercise-file");
            println!("run → runs the exercise-file");
            println!("pwd → prints the working directory");
            println!("exit → exits PyTon\n")
        } else if output == "help 0"{
            println!("\nDictionaries in Python are declared like following\nheights = {}\n","{'Mark':'1.8m','Jenna':'1.7m'}");
        } else if output == "help 1"{
            println!("\nTo get a value that is assigned to another value in the dictionary, do the following\nheights = {}\nheight_of_Mark = heights['Mark']\n","{'Mark':'1.8m','Jenna':'1.7m'}");
        } else if output == "help 2"{
            println!("\nTo change a value that is assigned to another value in the dictionary, do the following\nheights = {}\nheights['Jenna'] = '1.6m'\n","{'Mark':'1.8m','Jenna':'1.7m'}");
        } else if output == "help 3"{
            println!("\nTo add a new value and a value that is assigned to it, to the dictionary, do the following\nheights = {}\nheights['Kyle'] = '1.9m'\n","{'Mark':'1.8m','Jenna':'1.7m'}")
        } else if output == "help 3"{
            println!("\nTo print a whole dictionary, just do the following\nheights = {}\nprint(heights)\n","{'Mark':'1.8m','Jenna':'1.7m'}")
        }
    }


    if stage == 8 {
        println!("That's it for now in version 0.1.5")
    }
}

fn clear_terminal() {
    let mut stdout = stdout();
    write!(stdout, "\x1B[2J").unwrap();
    write!(stdout, "\x1B[H").unwrap();
}

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