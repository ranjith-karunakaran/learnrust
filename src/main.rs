use std::{env, fs::{self, metadata}, io::Error, path::PathBuf};

fn get_temp_dir() -> Result<PathBuf, Error> {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("extracted_documents");    
    if let Ok(mdata) = metadata(&tmp_dir.as_path()) {
        if fs::exists(&tmp_dir).unwrap() {
            if mdata.is_dir() {
                fs::remove_dir_all(&tmp_dir);
            } else {
                fs::remove_file(&tmp_dir);
            }
        }
    }
    fs::create_dir(tmp_dir.as_path());
    Ok(tmp_dir)
 }

fn main () {
    match get_temp_dir() {
        Ok(dir) => {
            println!("{:?}", dir);
        },
        Err(_) => {
            println!("{}", "Unable to create folder")
        }
    };
}