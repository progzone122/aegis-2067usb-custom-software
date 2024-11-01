#[macro_export]
macro_rules! define_animation_subcommand {
    () => {
        clap::Command::new("animation").arg(clap::Arg::new("value").required(true).index(1))
    };
}
#[macro_export]
macro_rules! define_brightness_subcommand {
    () => {
        clap::Command::new("brightness").arg(clap::Arg::new("value").required(true).index(1))
    };
}

#[macro_export]
macro_rules! define_speed_subcommand {
    () => {
        clap::Command::new("speed").arg(clap::Arg::new("value").required(true).index(1))
    };
}
#[macro_export]
macro_rules! define_reset_subcommand {
    () => {
        clap::Command::new("reset")
    };
}