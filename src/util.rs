
use std::{io::stdout, time::Duration, sync::mpsc};

use ansi_term::Color::RGB;
use crossterm::{execute, event::{poll, read}, cursor::Hide};
use prisma::Rgb;
use rand::{distributions::Alphanumeric, Rng,thread_rng, rngs::ThreadRng};
use angular_units::Deg;
use prisma::{FromColor, Hsv};

/// Paints the unput string with the given color
/// # Arguments
/// * `input` - The string to paint
/// * `color` - The color to paint the string with
/// # Returns
/// The painted string with ANSI escape codes
pub fn paint(input: &str, color: Rgb<f64>) -> String {
    let r: u8 = (color.red() * 255 as f64).floor() as u8;
    let g: u8 = (color.green() * 255 as f64).floor() as u8;
    let b: u8 = (color.blue() * 255 as f64).floor() as u8;
    let painted = RGB(r, g, b).paint(input).to_string();
    painted
}
/// Gets a random character
/// # Returns
/// A random character 
// TODO: make it return a char instead of a string
pub fn get_rand_char() -> String {
    let rng = rand::thread_rng();
    let character = rng
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
    character
}
/// Increments the hue of the given color by the given amount
/// # Arguments
/// * `color` - The color to increment
/// * `change_rate` - The amount to increment the hue by
/// # Returns
/// The new color
pub fn nextcol(color: Rgb<f64>, change_rate: f64) -> Rgb<f64> {
    let mut hsv = Hsv::from_color(&color);
    let mut thishue: Deg<f64> = hsv.hue();
    if thishue <= Deg(359 as f64) {
        thishue = thishue + Deg(change_rate);
    } else {
        thishue = Deg(0 as f64);
    }
    hsv.set_hue(thishue);
    Rgb::from_color(&hsv)
}
/// waits for a keypress in a separate thread
/// # Arguments
/// * `tx` - The sender to send the keypress to. sends a true bool
pub fn wait_for_keypress(tx: mpsc::Sender<bool>) {
    let mut stdout = stdout();
    
        if read().is_ok() { // wait until a key is pressed. dont care about info.
            tx.send(true).unwrap();
        }
}