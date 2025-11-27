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
use rusqlite::{Connection, Error, ErrorCode};
use std::{
    env::{self, consts::OS},
    fs::create_dir,
    io::ErrorKind,
    path::{Path, PathBuf},
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
        _ => panic!("You did not specify the appropriate amount of args\n"),
    }
    if args[1].len() <= 0 || args[2].len() <= 0 {
        panic!("Unexpected NULL Argument\n");
    }

    let command = args[2].to_string();
    let note_number: u32 = args[1].parse().unwrap();

    println!("DEBUG: Launch Parameters: {},{}", note_number, command);

    if note_number >= max_length {
        panic!("Your input exceeds the size of the database.\n")
    }

    let my_dir: PathBuf = match home_dir() {
        Some(my_dir) => my_dir,
        None => panic!(
            "Could not find users home directory. You will have to set up a config and point to where you would like to store data for the program\n"
        ),
    };

    println!("DEBUG: Home: {}", my_dir.to_string_lossy());
    let db_path = check_if_folder_and_db_exists_or_create(my_dir);
    create_db_table_if_not_exists(db_path);
}

fn check_if_folder_and_db_exists_or_create(my_dir: PathBuf) -> String {
    let mut my_dir = my_dir;

    my_dir.push("note");
    // let my_dir_concat = Path::new(&my_dir);
    match create_dir(&my_dir) {
        Ok(_file) => println!("Dir concact outcome: {:?}", my_dir.to_str()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("DEBUG:Folder already exists. It is ok to continue")
            }
            _ => println!(
                "Error creating directory. Access denied, or a parent folder in the path doesnt exist.: {:?}",
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
                // db_file.canonicalize().unwrap().to_string_lossy().into_owned()
            // match db_file.parent() {
            //     Some(parent_path) => parent_path.to_string_lossy().into_owned(),
            //     None => String::from(""),
            // }
        },
        Err(rusqlite) => format!("{}",rusqlite),
        // {
        //     match rusqlite.sqlite_error_code() {
        //         Some(ErrorCode::CannotOpen) => println!("DB Couldnt open"),
        //         _ => panic!("Could not open db for an unknown reason"),
        //     }
        // }, 
    }
    // println!("{}", db_file.to_string_lossy().into_owned());
}

fn create_db_table_if_not_exists(db_path: String) -> Result<(), Error> {
    let db_connect = Connection::open(db_path)?;
    let sql: &str = "CREATE TABLE IF NOT EXISTS notes(ID INT PRIMARY KEY NOT NULL, TIME INT NOT NULL, TITLE TEXT, NOTE TEXT)";
    db_connect.execute(sql, ())?;
    println!("DB Connect executed");
    Ok(())
}
fn check_if_note_already_exists() {}

fn get_case_switch_command_number() {}

fn create_note() {}

fn read_note() {}
fn delete_note() {}

fn modify_note() {}

fn modify_config() {}

fn restore_default_config() {}

fn read_config() {}

fn show_written_notes_num_and_title() {}
