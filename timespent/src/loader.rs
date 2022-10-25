use crate::activity::Activities;
use crate::parser::parse_activity;
use eyre::Result;
use std::fs::{read_dir, File};
use std::io::{prelude::*, BufReader};

pub fn load_from_filepath(path: &str) -> Result<Activities> {
    let mut activities = Vec::new();
    let files = read_dir(path)?;

    for f in files {
        let f = f?;
        let path = f.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let date = filename.split(".txt").next().unwrap();
        let f = File::open(f.path())?;
        let reader = BufReader::new(f);

        for line_f in reader.lines() {
            if let Ok(line) = line_f {
                match parse_activity(date, line.clone().as_str()) {
                    Ok((_, activity)) => activities.push(activity),
                    Err(err) => println!("can not parse line: {} {}", line, err),
                }
            } else {
                println!("file {}: can not read line: {:?}", &filename, line_f);
            }
        }
    }

    Ok(activities.into())
}
