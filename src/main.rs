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

use core::{error, panic};
use std::{env, io::Error, process::exit};

struct Note {
    title: String,
    body: String,
    time: String,
}

fn main() {
    println!("RustNote App Version: 0.0.1");
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //if args.len() != 3 {
    //   
    //}
    match args.len()  { 3 => (),_=>panic!("You did not specify the appropriate amount of args\n") }
    if args[1].len() <= 0 || args[2].len() <= 0  {
        panic!("Unexpected NULL Argument");

    }

    let command = args[2].to_string();
    let note_number: u32= args[1].parse().unwrap();

    println!("{command},{note_number}"); 

        

}

fn create_folder() {

}

fn check_if_folder_exists() {

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






