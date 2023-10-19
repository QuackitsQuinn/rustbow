#[cfg(test)]
mod tests {
    use crate::util::nextcol;
    use angular_units::Deg;
    use approx::assert_relative_eq;
    use prisma::{FromColor, Hsv, Rgb};

    #[test]
    fn test_serede_serial() {
        let config = crate::conf::Config {
            block_mode: true,
            change_rate: 0.001,
            saturation: 1.0,
            value: 1.0,
            random: true,
            chars: None,
        };
        let serialized = toml::to_string(&config).unwrap();
        println!("{}", serialized);
    }
    #[test]
    fn test_serede_deserial() {
        let serialized = r#"
        block_mode = true
        change_rate = 0.001
        saturation = 1.0
        value = 1.0
        random = true
        "#;
        let deserialized: crate::conf::Config = toml::from_str(serialized).unwrap();
        println!("{:?}", deserialized);
    }
    #[test]
    fn test_next_col() {
        let new_hue = 1.0;
        let color = Rgb::from_color(&Hsv::new(Deg(0 as f64), 1_f64, 1_f64));
        let new_color = nextcol(color, 1.0);
        let hue_scalar: Deg<f64> = Hsv::from_color(&new_color).hue();
        let actual_hue = hue_scalar.0.ceil();
        assert_relative_eq!(actual_hue, new_hue);
    }
}
