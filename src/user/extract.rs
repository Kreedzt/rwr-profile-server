// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use std::fs;
use std::io;
use tracing::info;

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
        let reader_path = path.into_os_string().into_string().unwrap();

        let path_string = reader_path.clone();
        let path_list = path_string.split("\\").collect::<Vec<_>>();
        info!("path_list: {:?}", path_list);

        let last_path = path_list.last().unwrap();

        let mut reader = Reader::from_file(reader_path)?;

        let mut buf = Vec::new();

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
                                        info!("found username: {}, id: {}", username, last_path);
                                        let last_path_name: Vec<&str> =
                                            last_path.split(".").collect();
                                        if let Some(id_str) = last_path_name.first() {
                                            let parse_res = id_str.parse::<u64>()?;
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
                _ => {}
            }
        }
    }

    Err(anyhow!("not found user profile id"))
}
