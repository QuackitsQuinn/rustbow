

use std::io::Stdout;

use ansi_term::Color::RGB;
use crossterm::{event::read, execute, cursor::MoveTo};
use prisma::Rgb;
use rand::{distributions::Alphanumeric, Rng};
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
/// Waits for a keypress in a separate thread. Thread is killed when a key is pressed.
pub fn wait_for_keypress() {
        let _ = read();
}

/// Moves the cursor to a random position on the screen
/// # Arguments
/// * `stdout` - The stdout to write to 
/// * `mx` - The maximum x position
/// * `my` - The maximum y position
pub fn go_rand_pos(mut stdout: &Stdout, mx: u16, my: u16) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..mx);
    let y = rng.gen_range(0..my);

    let _ = execute!(stdout, MoveTo(y as u16, x as u16));
}
/// Prints a random character to the screen
/// # Arguments
/// * `color` - The color to paint the character
/// * `stdout` - The stdout to write to
/// * `mx` - The maximum x position
/// * `my` - The maximum y position
pub fn put_rand(color: Rgb<f64>, stdout: &Stdout, mx: u16, my: u16, block_mode:bool) {
    let char: String;
    if !block_mode {
        char = get_rand_char()
    } else {
        char = String::from("█")
    }
    go_rand_pos(&stdout, mx, my);
    let painted = paint(char.as_str(), color);
    print!("{}", painted);
}
