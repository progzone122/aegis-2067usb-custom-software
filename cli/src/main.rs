use anyhow::{Context, Result};
use api::Api;
use api::led::led;
use clap::ArgMatches;
use api::config::Config;
use api::other::other;

mod subcommands;

fn main() -> Result<()> {
    let matches: ArgMatches = clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(define_animation_subcommand!())
        .subcommand(define_brightness_subcommand!())
        .subcommand(define_speed_subcommand!())
        .subcommand(define_reset_subcommand!())
        .get_matches();

    let api: Api = Api::default();
    let device = api.connect_device(0);

    let mut config: Config = Config::load();

    match device {
        Ok(interface) => {
            println!(
                "[INFO] Successfully connected to device with VID {:04X} and PID {:04X}",
                api.vid, api.pid
            );
            match matches.subcommand() {
                Some(("animation", sub_m)) => {
                    let value: &str = sub_m
                        .get_one::<String>("value")
                        .expect("value argument is required")
                        .as_str();

                    if let Ok(id) = value.parse::<u8>() {
                        if let Some(animation) = api::values::AnimationEffect::find_id(id as usize)
                        {
                            led::change_animation_effect(&mut config, interface, animation.hex)
                                .with_context(|| {
                                    format!("Failed to change animation effect to ID {}", id)
                                })?;
                        } else {
                            eprintln!("[ERROR] Animation effect not found for ID: {}", id);
                        }
                    } else if let Some(animation) = api::values::AnimationEffect::find_name(value) {
                        led::change_animation_effect(&mut config, interface, animation.hex)
                            .with_context(|| {
                                format!("Failed to change animation effect to name {}", value)
                            })?;
                    } else {
                        eprintln!("[ERROR] Could not find animation effect: {}", value);
                    }
                }
                Some(("brightness", sub_m)) => {
                    let value: &str = sub_m
                        .get_one::<String>("value")
                        .expect("value argument is required")
                        .as_str();

                    if let Ok(brightness) = value.parse::<u8>() {
                        led::change_brightness(&mut config, interface, brightness)
                            .with_context(|| {
                                format!("Failed to change brightness {}", brightness)
                            })?;
                    }
                }
                Some(("speed", sub_m)) => {
                    let value: &str = sub_m
                        .get_one::<String>("value")
                        .expect("value argument is required")
                        .as_str();

                    if let Ok(speed) = value.parse::<u8>() {
                        led::change_speed(&mut config, interface, speed)
                            .with_context(|| {
                                format!("Failed to change speed {}", speed)
                            })?;
                    }
                }
                Some(("reset", _)) => {
                    other::reset_settings(&mut config, interface)
                        .with_context(|| "Failed to reset settings")?;
                }
                None => {
                    eprintln!("[ERROR] No valid subcommand provided");
                }
                _ => {
                    eprintln!("[ERROR] No subcommand provided");
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] {:#?}", e);
            return Err(anyhow::anyhow!("Failed to connect to device"));
        }
    }

    Ok(())
}