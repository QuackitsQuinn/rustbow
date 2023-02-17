extern crate prisma;
extern crate crossterm;

use crossterm::{cursor::{Hide,MoveTo},execute,terminal::size,event::poll};
use rand::{distributions::Alphanumeric, Rng};
use ansi_term::Color::RGB;
use prisma::{Rgb, Hsv, FromColor};
use angular_units::Deg;
use std::{time::Duration,io::{stdout, Stdout}};
// Change rate is how fast the color changes
const CHANGE_RATE:f64 = 0.3;
const BLOCK:bool = true;
fn main() -> crossterm::Result<()>{
    let mut stdout = stdout();
    let res = execute!(stdout, Hide);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
    }
    let mut pgrmstate:bool = true;
    let mut color = Rgb::from_color(&Hsv::new(Deg(0 as f64), 1 as f64, 1 as f64));
    let size = size();
    let sizeur = size.unwrap();
    let mx = sizeur.1;
    let my = sizeur.0;
    while pgrmstate {
        color = nextcol(color);
        stdout = put_rand(color,stdout,mx,my);
        (pgrmstate,stdout) = end(stdout);
    }
    Ok(())
}
fn go_rand_pos(mut stdout:Stdout,mx:u16,my:u16) -> Stdout{
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..mx);
    let y = rng.gen_range(0..my);
    execute!(stdout,MoveTo(y as u16,x as u16));
    stdout
}
fn put_rand(color:Rgb<f64>,stdout:Stdout,mx:u16,my:u16) -> Stdout{
    let char:String;
    if !BLOCK {
        let rng = rand::thread_rng();
        char = rng.sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from).collect();
    } else {
        char = "█".to_string();
    }
    let stdo = go_rand_pos(stdout,mx,my);
    let painted = paint(char.as_str(),color);
    print!("{}", painted);
    stdo
}
fn end(stdout:Stdout) -> (bool,Stdout) {
    if poll(Duration::from_millis(0)).unwrap() {
        (false,stdout)
    } else {
        (true,stdout)
    }
}
fn nextcol(color:Rgb<f64>) -> Rgb<f64> {
    let mut hsv = Hsv::from_color(&color);
    let mut thishue:Deg<f64> = hsv.hue();
    if thishue <= Deg(359 as f64) {
        thishue = thishue + Deg(CHANGE_RATE);
    } else {
        thishue = Deg(0 as f64);
    }
    hsv.set_hue(thishue);
    Rgb::from_color(&hsv)
    
}

fn paint(input:&str,color:Rgb<f64>) -> String {
    let r:u8 = (color.red()*255 as f64).floor() as u8;
    let g:u8 = (color.green()*255 as f64).floor() as u8;
    let b:u8 = (color.blue()*255 as f64).floor() as u8;
    let painted = RGB(r,g,b).paint(input).to_string();
    painted
}