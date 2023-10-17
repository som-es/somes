use std::{fs::OpenOptions, io::Write, path::PathBuf, str::FromStr};

use axum::Json;
use somes_common_lib::SaveEmailInfo;

pub fn save_email_to_file(email: &str) -> Result<(), String> {
    let path = PathBuf::from_str("./email_list.txt").unwrap();
    if !path.exists() {
        std::fs::File::create("./email_list.txt").unwrap();
    }
    let mut email_file = OpenOptions::new()
        .write(true)
        .open("./email_list.txt")
        .map_err(|_| "Could not open file".to_string())?;

    email_file
        .write_all(format!("{}\n", email).as_bytes())
        .map_err(|_| "Could not write email to file".to_string())?;
    email_file.flush().unwrap();
    Ok(())
}

pub async fn save_email(Json(save_email_info): Json<SaveEmailInfo>) -> Result<Json<()>, String> {
    println!("save_email_info: {save_email_info:?}");

    save_email_to_file(&save_email_info.email)?;

    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use super::save_email_to_file;

    #[test]
    fn save_mail() {
        let mail = "test_mail@gmail.com";
        save_email_to_file(mail).unwrap();
    }
}
