// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use std::fs;
use std::io;
use tracing::{info, error};

pub fn extract_profile_list() {}

pub fn get_user_profile_id(username: &str, profile_path: &str) -> Result<u64> {
    let entries = fs::read_dir(profile_path)?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".profile")
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    for path in entries.into_iter() {
        let file_name = path.file_name().unwrap();
        let file_name_str = String::from(file_name.to_str().unwrap());

        info!("reader read path: {:?}", path);

        let cloned_path = path.clone();

        let mut reader = Reader::from_file(path).map_err(|err| {
            anyhow!("Read file to str error in file: {:?}, {:?}", cloned_path, err)
        })?;

        let mut buf = Vec::new();


        let cloned_path = cloned_path.clone();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) => match e.name() {
                    b"profile" => {
                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            match attr_key {
                                b"username" => {
                                    if attr_value == username {
                                        let last_path_name: Vec<&str> =
                                            file_name_str.split(".").collect();
                                        info!("found last_path_name: {:?}", last_path_name);

                                        if let Some(id_str) = last_path_name.first() {
                                            let parse_res = id_str.parse::<u64>()?;
                                            info!("found username: {}, id: {}", username, parse_res);
                                            return Ok(parse_res);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => {
                    break;
                }
                Err(e) => {
                    error!("in error!");
                    return Err(anyhow!("Reader parse file error in file: {:?}: {:?}", cloned_path, e))
                },
                _ => {
                    //
                }
            }
        }
    }

    Err(anyhow!("not found user profile id"))
}
