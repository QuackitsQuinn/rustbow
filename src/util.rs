
use ansi_term::Color::RGB;
use prisma::Rgb;
use rand::{distributions::Alphanumeric, Rng,thread_rng, rngs::ThreadRng};
use angular_units::Deg;
use prisma::{FromColor, Hsv};

pub fn paint(input: &str, color: Rgb<f64>) -> String {
    let r: u8 = (color.red() * 255 as f64).floor() as u8;
    let g: u8 = (color.green() * 255 as f64).floor() as u8;
    let b: u8 = (color.blue() * 255 as f64).floor() as u8;
    let painted = RGB(r, g, b).paint(input).to_string();
    painted
}
pub fn get_rand_char() -> String {
    let rng = rand::thread_rng();
    let character = rng
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
    character
}
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
