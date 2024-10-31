use clap::ArgMatches;
use api::Api;
use api::led::LED;
use anyhow::{Context, Result};

macro_rules! define_animation_subcommand {
    () => {
        clap::Command::new("animation")
            // animation name / animation id
            .arg(
                clap::Arg::new("value")
                    .required(true)
                    .index(1),
            )
    };
}

fn main() -> Result<()> {
    let matches: ArgMatches = clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(define_animation_subcommand!())
        .get_matches();

    let api: Api = Api::default();
    let device = api.connect_device(0);

    let mut led_settings: LED = LED::default();

    match device {
        Ok(interface) => {
            println!("[INFO] Successfully connected to device with VID {:04X} and PID {:04X}", api.vid, api.pid);
            match matches.subcommand() {
                Some(("animation", sub_m)) => {
                    let value: &str = sub_m.get_one::<String>("value")
                        .expect("value argument is required")
                        .as_str();

                    if let Ok(id) = value.parse::<u8>() {
                        if let Some(animation) = api::values::AnimationEffect::find_id(id as usize) {
                            led_settings.change_animation_effect(interface, animation.hex)
                                .with_context(|| format!("Failed to change animation effect to ID {}", id))?;
                        } else {
                            eprintln!("[ERROR] Animation effect not found for ID: {}", id);
                        }
                    } else if let Some(animation) = api::values::AnimationEffect::find_name(value) {
                        led_settings.change_animation_effect(interface, animation.hex)
                            .with_context(|| format!("Failed to change animation effect to name {}", value))?;
                    } else {
                        eprintln!("[ERROR] Could not find animation effect: {}", value);
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