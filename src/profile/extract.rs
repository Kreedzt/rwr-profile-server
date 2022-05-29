// SPDX-License-Identifier: GPL-3.0-only
use super::model::Profile;
use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::str;
use tracing::info;

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
                b"stats" => {
                    for attr in e.attributes() {
                        let attr_unwrap_res = attr?;
                        let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                        let attr_key = attr_unwrap_res.key;

                        match attr_key {
                            b"kills" => {
                                profile.stats.kills = attr_value.parse()?;
                            }
                            b"deaths" => {
                                profile.stats.deaths = attr_value.parse()?;
                            }
                            b"time_played" => {
                                profile.stats.time_played = attr_value.parse()?;
                            }
                            b"player_kills" => {
                                profile.stats.player_kills = attr_value.parse()?;
                            }
                            b"teamkills" => {
                                profile.stats.team_kills = attr_value.parse()?;
                            }

                            b"longest_kill_streak" => {
                                profile.stats.longest_kill_streak = attr_value.parse()?;
                            }
                            b"targets_destroyed" => {
                                profile.stats.targets_destroyed = attr_value.parse()?;
                            }
                            b"vehicles_destroyed" => {
                                profile.stats.vehicles_destroyed = attr_value.parse()?;
                            }
                            b"soldiers_healed" => {
                                profile.stats.soldiers_healed = attr_value.parse()?;
                            }
                            b"times_got_healed" => {
                                profile.stats.times_got_healed = attr_value.parse()?;
                            }
                            b"distance_moved" => {
                                profile.stats.distance_moved = attr_value.parse()?;
                            }
                            b"shots_fired" => {
                                profile.stats.shots_fired = attr_value.parse()?;
                            }
                            b"throwables_thrown" => {
                                profile.stats.throwables_thrown = attr_value.parse()?;
                            }
                            b"rank_progression" => {
                                profile.stats.rank_progression = attr_value.parse()?;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => {}
            Ok(Event::End(e)) => {
                // DEBUG
                // println!("end e.name: {:?}", str::from_utf8(e.name())?);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {} : {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    Ok(profile)
}
