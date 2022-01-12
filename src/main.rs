use clap::{App, Arg};

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use enigo::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Simple Mouse Clicker")
        .version("0.1.0")
        .author("poly000 <pedajilao@vip.qq.com>")
        .about("automatic mouse clicker tool.")
        .arg(
            Arg::with_name("times")
                .help("times to click")
                .short("t")
                .long("times")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("offset")
                .help("offset between clicks (ms), not including offset between press and release")
                .short("o")
                .long("offset")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("right_key")
                .help("click right key instead")
                .short("r")
                .long("right-key")
        )
        .get_matches();

    let times = matches
        .value_of("times")
        .ok_or("illegal format of 'times', you should provide a integer.")?
        .parse()?;
    let offset = Duration::from_millis(
        matches
            .value_of("offset")
            .ok_or("illegal format of 'offset', you should provide a integer.")?
            .parse()?,
    );
    let right_key: bool = matches.is_present("right_key");

    let mut enigo = Enigo::new();

    for _ in 0_u64..times {
        enigo.mouse_click(if right_key {MouseButton::Right} else {MouseButton::Left});
        if !offset.is_zero() {
            sleep(offset);
        }
    }
    Ok(())
}