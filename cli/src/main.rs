use anyhow::{Context, Result};
use api::led::LED;
use api::Api;
use clap::ArgMatches;
use api::config::Config;

mod subcommands;

fn main() -> Result<()> {
    let matches: ArgMatches = clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(define_animation_subcommand!())
        .subcommand(define_brightness_subcommand!())
        .subcommand(define_speed_subcommand!())
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
                            LED::change_animation_effect(&mut config, interface, animation.hex)
                                .with_context(|| {
                                    format!("Failed to change animation effect to ID {}", id)
                                })?;
                        } else {
                            eprintln!("[ERROR] Animation effect not found for ID: {}", id);
                        }
                    } else if let Some(animation) = api::values::AnimationEffect::find_name(value) {
                        LED::change_animation_effect(&mut config, interface, animation.hex)
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
                        println!("0x{:02X}", brightness);
                        LED::change_brightness(&mut config, interface, brightness)
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
                        LED::change_speed(&mut config, interface, speed)
                            .with_context(|| {
                                format!("Failed to change speed {}", speed)
                            })?;
                    }
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