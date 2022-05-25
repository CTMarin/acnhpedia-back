use std::{
    fs::{File, OpenOptions}, 
    path::Path, 
    io::{BufReader, BufRead, Write}
};

fn user_file_name(user: &str) -> String {
    let file: String = format!("server-data/users/{}.txt", user);
    
    file
}

pub fn add_obtained_card(user: &str, card_type: &str, id: i32) -> Result<i32, i32> {
    if !check_obtained_card(user, card_type, id) {
        if let Err(_) = writeln!(open_user_file(user), "{}:{}", card_type, id) {
            return Err(500)
        }

        return Ok(200)

    } else {
        return Err(409)
    }
}

pub fn remove_obtained_card(user: &str, card_type: &str, id: i32) {

}

pub fn check_obtained_card(user: &str, input_card_type: &str, input_id: i32) -> bool {
    if exists_db_file(user) {
        let file_string = user_file_name(user);
        let path = Path::new(file_string.as_str());
        let file = File::open(path).unwrap();
        let buffer_reader = BufReader::new(file);
        let mut lines_iter = buffer_reader.lines().map(|line| line.unwrap());
    
        while let Some(line) = lines_iter.next() {
            let mut split = line.split(":");
            let card_type = split.nth(0).unwrap();
            let id = split.nth(0).unwrap().parse::<i32>().unwrap();
            if card_type.eq(input_card_type) && id == input_id {
                return true
            }
        }
    }
    false
}

fn open_user_file(user: &str) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(user_file_name(user))
        .unwrap()
}

fn exists_db_file(user: &str) -> bool {
    let path_string = user_file_name(user);
    let path = Path::new(path_string.as_str());

    path.exists() || path.is_file()
}