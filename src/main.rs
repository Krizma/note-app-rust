/*Things Needed
Message Struct

create_folder()
check_if_folder()
check_note_db()
callback()
create_db_table_if_not_exists()
check_if_note_already_exists()
get_case_switch_command_number() --Improve this probs
create_note()
read_note()
trim()
delete_note()
modify_note()
create_config() --Not in C version
read_config() --Not in C version
*/

use core::panic;
use dirs::home_dir;
use rusqlite::{Connection, Result};
use std::{
    env::{self}, fs::create_dir, io::ErrorKind, path::PathBuf, string
};

struct Note {
    id: u32,
    time: i64,
    title: String,
    note: String,
}

fn main() {
    let max_length: u32 = u32::MAX;

    println!("RustNote App Version: 0.0.1\n");
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //if args.len() != 3 {
    //
    //}
    match args.len() {
        3 => (),
        _ => panic!("DEBUG:You did not specify the appropriate amount of args\n"),
    }
    if args[1].len() <= 0 || args[2].len() <= 0 {
        panic!("DEBUG:Unexpected NULL Argument\n");
    }

    let note_command = args[2].to_string();
    let note_number: u32 = args[1].parse().unwrap();

    println!("DEBUG: Launch Parameters: {},{}", note_number, note_command);

    if note_number >= max_length {
        panic!("DEBUG: Your input exceeds the size of the database.\n")
    }

    let my_dir: PathBuf = match home_dir() {
        Some(my_dir) => my_dir,
        None => panic!(
            "DEBUG: Could not find users home directory. You will have to set up a config and point to where you would like to store data for the program\n"
        ),
    };

    println!("DEBUG: Home: {}", my_dir.to_string_lossy());
    let db_path: String = check_if_folder_and_db_exists_or_create(my_dir);
    create_db_table_if_not_exists(db_path.clone());
    check_if_note_already_exists(db_path.clone(),note_number.clone());
    get_user_command(db_path.clone(),note_command.clone());
}

fn check_if_folder_and_db_exists_or_create(my_dir: PathBuf) -> String {
    let mut my_dir = my_dir;

    my_dir.push("note");
    // let my_dir_concat = Path::new(&my_dir);
    match create_dir(&my_dir) {
        Ok(_file) => println!("Dir concact outcome: {:?}", my_dir.to_str()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("DEBUG: Folder already exists. It is ok to continue")
            }
            _ => println!(
                "\nDEBUG: Error creating directory. Access denied, or a parent folder in the path doesnt exist.: {:?}",
                error
            ),
        },
    }
    let mut db_file = my_dir.clone();
    print!("{:?}", db_file);
    db_file.push("note");
    db_file.set_file_name("note");
    print!("{:?}", db_file);
    db_file.set_extension("db");
    print!("{:?}", db_file);
    let db: Result<Connection, rusqlite::Error> = Connection::open(&db_file);
    match db {
        Ok(_) => {
            db_file.to_string_lossy().into_owned()

        },
        Err(rusqlite) => format!("{}",rusqlite),
    }
}
fn create_db_table_if_not_exists(db_path: String) -> Result<()> {
    let sql: &str = "CREATE TABLE IF NOT EXISTS notes(ID INT PRIMARY KEY NOT NULL, TIME INT NOT NULL, TITLE TEXT, NOTE TEXT)";
    let db_connect = Connection::open(db_path)?;
    let table_exists = db_connect.table_exists(None, "notes");
      match table_exists.as_ref() {
        Ok(true)=>{
            println!("\nDEBUG: table_exists");
        }
        Ok(false)=>{
          if let Err(e) = db_connect.execute(sql, ()){
            println!("\nDEBUG: cant create: {}",e);
          }
        }
        Err(e)=>
        {
            println!("\nDEBUG: HEHE HAHA ERROR {}",e);
        }
      }
    Ok(())
}
fn check_if_note_already_exists(db_path:String,note_number:u32) -> Result<()>{
    let db_connect = Connection::open(db_path)?;
    let sql:String = format!("SELECT EXISTS(SELECT 1 FROM notes WHERE ID = {}",note_number);
    let sql: &str = &sql;
    db_connect.execute(&sql, ())?;
    println!("DB Connect and Executed in: Check if note already exists");
    Ok(())

}

fn get_user_command(db_path:String,note_command:String) {
    let create_strings: [String; 2] = ["create","new"];
    match note_command {
         "create" | "new" => create_note(),
        "read"  => read_note(),
        "delete" | "remove" => delete_note(),
        "update" | "edit" | "modify" => modify_note(),
    }
}

fn create_note() {
    unimplemented!("Create note")
}

fn read_note() {
    unimplemented!("read note")
}
fn delete_note() {
    unimplemented!("delete note")
}

fn modify_note() {
    unimplemented!("modify note")
}

fn modify_config() {
    unimplemented!("modify config")
}

fn restore_default_config() {
    unimplemented!("restor default config")
}

fn read_config() {
    unimplemented!("read config")
}

fn show_written_notes_num_and_title() {
    unimplemented!("show written notes")
}
