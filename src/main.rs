use figlet_rs::FIGfont;
use std::{
    fs,
    io::{self},
    path::Path,
};

fn read_stdin(prompt: &str, buffer: &mut String) -> io::Result<()> {
    loop {
        println!("{}", prompt);
        buffer.clear();
        io::stdin().read_line(buffer)?;
        if let Err(err) = check_file_exist(&buffer) {
            return Err(err);
        } else {
            return Ok(());
        }
    }
}

fn check_file_exist(path: &str) -> io::Result<()> {
    let parsed_path = Path::new(path.trim_end());
    if parsed_path.exists() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} doesn't exist", parsed_path.display()),
        ))
    }
}

fn organize_files(src: &str, dst: &str) -> io::Result<()> {
    let paths = fs::read_dir(src)?;
    for path in paths {
        let file = path?;
        let file_path = file.path();
        let mime_type = mime_guess::from_path(file_path.clone());
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        assign_file_type(src, dst, file_name, mime_type.first_raw());
    }
    Ok(())
}

fn assign_file_type(src: &str, dst: &str, file_name: &str, file_type: Option<&str>) {
    if let Some(val) = file_type {
        let mime_type = val.split("/").nth(0);
        match mime_type {
            Some("text") | Some("image") | Some("audio") | Some("video") => {
                let val = mime_type.unwrap();
                create_dir(dst, val);
                move_file(dst, val, file_name, src).unwrap();
            }
            _ => {
                create_dir(dst, "other");
                move_file(dst, "other", file_name, src).unwrap();
            }
        };
    } else {
        create_dir(dst, "other");
        move_file(dst, "other", file_name, src).unwrap();
    }
}

fn create_dir(dst: &str, file_type: &str) {
    let path = Path::new(dst);
    let buff = path.join(file_type);
    let dir_result = fs::create_dir(buff);
    match dir_result {
        _ => {}
    }
}

fn move_file(dst: &str, file_type: &str, file_name: &str, src: &str) -> io::Result<()> {
    let to = Path::new(dst).join(file_type).join(file_name);
    let from = Path::new(src).join(file_name);
    if from.is_file() {
        fs::rename(from, to)
    } else {
        Ok(())
    }
}

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Void File Organizer");
    println!("{}", figure.unwrap());
    let mut src = String::new();
    let mut dst = String::new();
    read_stdin("Please enter the source folder (./)", &mut src).expect("something went wrong");
    read_stdin("Please enter the destination folder (./)", &mut dst).expect("something went wrong");
    organize_files(&src.trim_end(), &dst.trim_end()).expect("something went wrong");
}
