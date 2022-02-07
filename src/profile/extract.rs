use super::model::Profile;
use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::str;

pub fn extract_profile(id: u64, folder_path: &str) -> Result<Profile> {
    let mut profile = Profile::default();

    let path = format!("{}/{}.profile", folder_path, id);

    let mut reader = Reader::from_file(path)?;

    reader.trim_text(true);

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
                            b"game_version" => {
                                profile.game_version = attr_value;
                            }
                            b"username" => {
                                profile.username = attr_value;
                            }
                            b"sid" => {
                                profile.sid = attr_value;
                            }
                            b"rid" => {
                                profile.rid = attr_value;
                            }
                            b"squad_tag" => profile.squad_tag = attr_value,
                            b"color" => profile.color = attr_value,
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => {}
            Ok(Event::End(e)) => {
                println!("end e.name: {:?}", str::from_utf8(e.name())?);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {} : {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    Ok(profile)
}
