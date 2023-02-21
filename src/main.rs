extern crate crossterm;
extern crate prisma;

mod util;
mod conf;
mod test;

use angular_units::Deg;
use conf::Config;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{size, Clear, ClearType},
};
use prisma::{FromColor, Hsv, Rgb};
use std::{
    io::stdout, thread,
};

// TODO: Make all of these configurable with args + default config file

/// Main function 
/// For now, it contains the main loop. It will be moved to a separate file later with better customizability.
fn main() -> crossterm::Result<()> {
    let config = conf::get_config();
    rainbow(config)
}

fn rainbow(config:Config) -> crossterm::Result<()> {
    let mut stdout = stdout();
    let res = execute!(stdout, Hide);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
    // Create the inital color. The sat and val are never changed so they only need to be set once.
    let mut color = Rgb::from_color(&Hsv::new(Deg(0 as f64), config.saturation, config.value));
    let size = size();
    let sizeur = size.unwrap();
    let mx = sizeur.1;
    let my = sizeur.0;
    let thread = thread::spawn(move || {
        util::wait_for_keypress();
    });
    loop {
        color = util::nextcol(color, config.change_rate);
        util::put_rand(color, &stdout, mx, my,config.block_mode);
        if thread.is_finished() {
            break; // Exit when a key is pressed
        }
    }
    let _ = execute!(stdout, Show);
    let _ = execute!(stdout, Clear(ClearType::All));
    Ok(())
}
