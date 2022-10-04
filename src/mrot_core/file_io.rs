use std::{
    fs::{self, File},
    io::{BufReader, Stderr},
    path::Path,
};

use serde::de;

fn deserialize_orchestration_file<T: de::DeserializeOwned>(
    file_buffer: BufReader<File>,
) -> Result<T, serde_yaml::Error> {
    let d = serde_yaml::from_reader(file_buffer).expect("failed to deserialise file");

    Ok(d)
}

/// read_orchestration_file
pub fn read_orchestration_file<T: de::DeserializeOwned>(file_path: &Path) -> Result<T, Stderr> {
    let file_handler = fs::File::open(file_path).expect("failed to open file");

    let buffer = BufReader::new(file_handler);

    Ok(deserialize_orchestration_file::<T>(buffer).unwrap())
}

pub fn write_orchestration_file<T>(_file_path: &Path) -> T {
    unimplemented!()
}
