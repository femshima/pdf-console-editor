use std::fmt::Display;

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
