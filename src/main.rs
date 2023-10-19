extern crate crossterm;
extern crate prisma;

mod conf;
mod test;
mod util;

use angular_units::Deg;
use conf::Config;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{size, Clear, ClearType},
};
use prisma::{FromColor, Hsv, Rgb};
use std::{io::stdout, thread};

// TODO: Make all of these configurable with args

fn main() -> crossterm::Result<()> {
    let config = conf::get_config();
    let res = rainbow(config.0);
    if config.1 {
        let path = conf::get_config_path();
        let pathstr = path.to_str().unwrap();
        println!(); // newline to ensure the config file is at the start of the line
        println!("Since this is the first time rustbow has been ran, a config file has been created at {}!", pathstr)
    }
    res
}

fn rainbow(config: Config) -> crossterm::Result<()> {
    let mut stdout = stdout();
    let res = execute!(stdout, Hide);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
    // Create the initial color. The sat and val are never changed so they only need to be set once.
    let mut color = Rgb::from_color(&Hsv::new(Deg(0 as f64), config.saturation, config.value));
    let size = size();
    let sizeur = size.unwrap();
    let mx = sizeur.1;
    let my = sizeur.0;
    let thread = thread::spawn(move || {
        util::wait_for_keypress();
    });
    let chars = config
        .chars
        .unwrap_or_else(|| "█".to_owned())
        .chars()
        .collect::<Vec<char>>();
    loop {
        color = util::nextcol(color, config.change_rate);
        util::put_rand(color, &stdout, mx, my, config.random, &chars);
        if thread.is_finished() {
            break;
        }
    }
    let _ = execute!(stdout, Show);
    let _ = execute!(stdout, Clear(ClearType::All));
    Ok(())
}
