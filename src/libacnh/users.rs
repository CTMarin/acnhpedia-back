use std::{
    path::Path, 
    fs::{File, OpenOptions},
    io::{BufReader, BufRead, Write}
};

const fn user_file_name() -> &'static str {
    #![allow(unused_variables)]
    let file = "server-data/users.txt";
    
    #[cfg(test)]
    let file: &str = "server-data/users_test.txt";
    
    file
}

pub fn check_user(input_email: &str) -> bool {
    if exists_db_file() {
        let path = Path::new(user_file_name());
        let file = File::open(path).unwrap();
        let buffer_reader = BufReader::new(file);
        let mut lines_iter = buffer_reader.lines().map(|line| line.unwrap());

        while let Some(line) = lines_iter.next() {
            let mut split = line.split(":");
            let email = split.nth(0).unwrap();

            if email.eq(input_email) {
                return true
            }
        }
    }

    false
}

pub fn check_login(input_email: &str, input_password: &str) -> Result<i32, i32> {
    if exists_db_file() {
        let path = Path::new(user_file_name());
        let file = File::open(path).unwrap();
        let buffer_reader = BufReader::new(file);
        let mut lines_iter = buffer_reader.lines().map(|line| line.unwrap());
    
        while let Some(line) = lines_iter.next() {
            let mut split = line.split(":");
            let email = split.nth(0).unwrap();
            let password = split.nth(0).unwrap();
            if email.eq(input_email) {
                if password.eq(input_password) {
                    return Ok(200)
                } else {
                    return Err(401)
                }
            }
        }

        Err(404)
    } else {
        Err(500)
    }
}

pub fn register_user(email: &str, username: &str, password: &str, rep_password: &str) -> Result<i32, i32> {
    if password != rep_password {
        return Err(400)
    }

    if check_user(email) {
        return Err(409)
    } else {
        if let Err(_) = writeln!(open_users_file(), "{}:{}:{}", email, password, username) {
            return Err(500)
        }

        Ok(200)
    }
}

fn open_users_file() -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(user_file_name())
        .unwrap()
}

fn exists_db_file() -> bool {
    let path = Path::new(user_file_name());

    path.exists() || path.is_file()
}

