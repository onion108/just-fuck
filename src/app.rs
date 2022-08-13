use std::{path::Path, fs::{File, OpenOptions}, io::Read, io::Write};

pub fn user_friendly_read_from_file(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    if path.exists() {
        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!("cannot open the file {}: {}", filename, err));
            }
        };
        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(format!("cannot read from file {}: {}", filename, err))
        }
    } else {
        Err(format!("cannot read from file due to {} does not exist.", filename))
    }
}

pub fn user_friendly_write_to_file(filename: &str, content: &str) -> Result<(), String> {
    let path = Path::new(filename);
    let mut f = match OpenOptions::new().create(true).write(true).truncate(true).open(path) {
        Ok(file) => file,
        Err(err) => return Err(format!("cannot write to file {}: {}", filename, err)),
    };
    match write!(&mut f, "{}", content) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("cannot write to file {}: {}", filename, err))
    }
}
