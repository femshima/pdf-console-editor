use std::fmt::Display;

use crate::graphics::color::Color;

#[derive(Debug, Clone)]
pub struct ArgRange(f32, f32);

impl Default for ArgRange {
    fn default() -> Self {
        Self(20., 400.)
    }
}

impl Display for ArgRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

impl ArgRange {
    pub fn parser(s: &str) -> Result<Self, String> {
        match s.split_once("..").map(|(l, u)| (l.parse(), u.parse())) {
            Some((Ok(l), Ok(u))) => {
                if 0. < l && l <= u {
                    Ok(Self(l, u))
                } else {
                    Err("Range must satisfy 0<[Lower Bound]<=[Upper Bound]".to_string())
                }
            }
            _ => Err("Range satisfy the format <number>..<number>".to_string()),
        }
    }
    pub fn to_f32_f32(&self) -> (f32, f32) {
        (self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct ArgColor(pub Color);

impl Display for ArgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ArgColor {
    pub fn parser(s: &str) -> Result<Self, String> {
        let parse_regex =
            regex::Regex::new(r"(?P<space>gray|rgb|cmyk)\((?P<params>([0-9.]+,? *)*)\)").unwrap();
        if let Some(matches) = parse_regex.captures(s) {
            let space = matches.name("space").unwrap().as_str();
            let params: Vec<f32> = matches
                .name("params")
                .unwrap()
                .as_str()
                .split(",")
                .filter_map(|m| m.trim().parse().ok())
                .collect();
            match space {
                "gray" => {
                    if let [g] = params[..] {
                        return Ok(Self(Color::Gray(g)));
                    }
                }
                "rgb" => {
                    if let [r, g, b] = params[..] {
                        return Ok(Self(Color::RGB(r, g, b)));
                    }
                }
                "cmyk" => {
                    if let [c, m, y, k] = params[..] {
                        return Ok(Self(Color::CMYK(c, m, y, k)));
                    }
                }
                _ => panic!(),
            }
            Err(r"The number of parameters is wrong".to_string())
        } else {
            Err(r"Cannot parse the color. Make sure it looks like rgb(0.1,0.7,0.8)".to_string())
        }
    }
}
