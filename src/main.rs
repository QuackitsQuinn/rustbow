extern crate crossterm;
extern crate prisma;
mod util;

use angular_units::Deg;
use ansi_term::Color::RGB;
use crossterm::{
    cursor::{Hide, MoveTo},
    event::poll,
    execute,
    terminal::size,
};
use prisma::{FromColor, Hsv, Rgb};
use rand::{distributions::Alphanumeric, Rng};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};
// Change rate is how fast the color changes
// TODO: make it changeable by user
// TODO: make it a function of the size of the terminal
// TODO: make it so you dont have to ctrl+c to exit
// maybe 180/maxx*maxy?
const CHANGE_RATE: f64 = 0.001;
const BLOCK: bool = true;
fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    let res = execute!(stdout, Hide);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
    let mut color = Rgb::from_color(&Hsv::new(Deg(0 as f64), 1 as f64, 1 as f64));
    let size = size();
    let sizeur = size.unwrap();
    let mx = sizeur.1;
    let my = sizeur.0;
    loop {
        color = util::nextcol(color, CHANGE_RATE);
        stdout = put_rand(color, stdout, mx, my);
    }
    Ok(())
}
fn go_rand_pos(mut stdout: Stdout, mx: u16, my: u16) -> Stdout {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..mx);
    let y = rng.gen_range(0..my);
    // Not using result because speed is extremely important
    execute!(stdout, MoveTo(y as u16, x as u16));
    stdout
}
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

