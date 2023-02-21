extern crate crossterm;
extern crate prisma;
mod util;
mod conf;
mod test;

use angular_units::Deg;
use crossterm::{
    cursor::Hide,
    execute,
    terminal::size,
};
use prisma::{FromColor, Hsv, Rgb};
use std::{
    io::stdout, thread,
};

// TODO: Make all of these configurable with args + default config file


// Hue change rate. changes per character
const CHANGE_RATE: f64 = 0.001;
// Saturation of the color
const SATURATION: f64 = 1 as f64;
// Value of the color
const VALUE: f64 = 1 as f64;
// Block mode. If true, it will use █ instead of random characters
const BLOCK: bool = true;

/// Main function 
/// For now, it contains the main loop. It will be moved to a separate file later with better customizability.
fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    let res = execute!(stdout, Hide);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
    // Create the inital color. The sat and val are never changed so they only need to be set once.
    let mut color = Rgb::from_color(&Hsv::new(Deg(0 as f64), SATURATION, VALUE));
    let size = size();
    let sizeur = size.unwrap();
    let mx = sizeur.1;
    let my = sizeur.0;
    let thread = thread::spawn(move || {
        util::wait_for_keypress();
    });
    loop {
        color = util::nextcol(color, CHANGE_RATE);
        stdout = util::put_rand(color, stdout, mx, my,BLOCK);
        if thread.is_finished() {
            break; // Exit when a key is pressed
        }
    }
    Ok(())
}
