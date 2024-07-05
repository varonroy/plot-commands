use rgb::RGBA;

fn parse_hex_digit(digit: char) -> u32 {
    match digit {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!("invalid character {digit}"),
    }
}

fn parse_hex(hex: &str) -> u32 {
    hex.chars()
        .map(parse_hex_digit)
        .rev()
        .enumerate()
        .map(|(i, x)| x * 10u32.pow(i as u32))
        .sum()
}

fn parse_hex_color(hex: &str) -> RGBA<f32> {
    let hex = if hex.starts_with("#") { &hex[1..] } else { hex };

    let r = parse_hex(&hex[0..2]);
    let g = parse_hex(&hex[2..4]);
    let b = parse_hex(&hex[4..6]);
    let a = hex.get(6..8).map(parse_hex).unwrap_or(255);

    RGBA {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

pub fn generate_palette() -> Vec<RGBA<f32>> {
    let colors = [
        "1f77b4", "ff7f0e", "2ca02c", "d62728", "9467bd", "8c564b", "e377c2", "7f7f7f", "bcbd22",
        "17becf",
    ];

    let colors = colors.into_iter().map(parse_hex_color).collect::<Vec<_>>();
    colors
}

#[derive(Debug, Clone)]
pub struct Palette {
    colors: Vec<RGBA<f32>>,
}

impl std::default::Default for Palette {
    fn default() -> Self {
        let colors = [
            "1f77b4", "ff7f0e", "2ca02c", "d62728", "9467bd", "8c564b", "e377c2", "7f7f7f",
            "bcbd22", "17becf",
        ];

        let colors = colors.into_iter().map(parse_hex_color).collect::<Vec<_>>();

        Self { colors }
    }
}

impl Palette {
    pub fn iter(&self) -> PaletteIter {
        self.clone().into()
    }
}

pub struct PaletteIter {
    palette: Palette,
    i: usize,
}

impl From<Palette> for PaletteIter {
    fn from(value: Palette) -> Self {
        Self {
            palette: value,
            i: 0,
        }
    }
}

impl Iterator for PaletteIter {
    type Item = RGBA<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.palette.colors.get(self.i).copied();
        if c.is_some() {
            self.i += 1;
        }
        c
    }
}
