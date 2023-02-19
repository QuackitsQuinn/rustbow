extern crate crossterm;
extern crate prisma;
mod util;

use angular_units::Deg;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::size,
};
use prisma::{FromColor, Hsv, Rgb};
use rand::Rng;
use std::{
    io::{stdout, Stdout}, thread,
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
        stdout = put_rand(color, stdout, mx, my);
        if thread.is_finished() {
            break; // pure debug
        }
    }
    Ok(())
}
/// Moves the cursor to a random position on the screen
/// # Arguments
/// * `stdout` - The stdout to write to
/// * `mx` - The maximum x position
/// * `my` - The maximum y position
/// # Returns
/// The stdout to write to. This is likely to be removed in the future.
fn go_rand_pos(mut stdout: Stdout, mx: u16, my: u16) -> Stdout {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..mx);
    let y = rng.gen_range(0..my);
    // Not using result because speed is extremely important

    let _ = execute!(stdout, MoveTo(y as u16, x as u16));
    stdout
}
/// Prints a random character to the screen
/// # Arguments
/// * `color` - The color to paint the character
/// * `stdout` - The stdout to write to
/// * `mx` - The maximum x position
/// * `my` - The maximum y position
/// # Returns
///  The stdout to write to. This is likely to be removed in the future.
fn put_rand(color: Rgb<f64>, stdout: Stdout, mx: u16, my: u16) -> Stdout {
    let char: String;
    if !BLOCK {
        char = util::get_rand_char()
    } else {
        char = String::from("█")
    }
    let stdo = go_rand_pos(stdout, mx, my);
    let painted = util::paint(char.as_str(), color);
    print!("{}", painted);
    stdo
}

