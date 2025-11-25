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

use core::{panic};
use std::{env, fs::create_dir, path::PathBuf};
use dirs::{home_dir};


struct Note {
    title: String,
    body: String,
    time: String,
}

fn main() {
    let max_length:u32 = u32::MAX;

    println!("RustNote App Version: 0.0.1\n");
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //if args.len() != 3 {
    //   
    //}
    match args.len()  { 3 => (),_=>panic!("You did not specify the appropriate amount of args\n") }
    if args[1].len() <= 0 || args[2].len() <= 0  {
        panic!("Unexpected NULL Argument\n");

    }

    let command = args[2].to_string();
    let note_number: u32= args[1].parse().unwrap();

    println!("DEBUG: Launch Parameters: {},{}", note_number,command); 
    
    if note_number >= max_length {
        panic!("Your input exceeds the size of the database.\n")
    }

    let my_dir:PathBuf = match home_dir() {
        Some(my_dir) => my_dir,
        None => panic!("Could not find users home directory. You will have to set up a config and point to where you would like to store data for the program\n")
    };
    
    println!("DEBUG: Home: {}",my_dir.to_string_lossy());
    check_if_folder_exists_or_create_folder(my_dir);
}

fn check_if_folder_exists_or_create_folder(my_dir:PathBuf) -> std::io::Result<()> {
    create_dir(my_dir.to_string_lossy()+"/note")?;
    println!("DEBUG: Dir concact outcome: {}/note",my_dir.to_string_lossy());
    Ok(());
}


fn check_if_note_db_exists() {

}

fn create_db_table_if_not_exists() {

}
fn check_if_note_already_exists() {

}

fn get_case_switch_command_number() {

}

fn create_note() {

}

fn read_note() {

}
fn delete_note() {

}

fn modify_note() {

}

fn modify_config() {

}

fn restore_default_config() {

}

fn read_config() {

}

fn show_written_notes_num_and_title() {

}






