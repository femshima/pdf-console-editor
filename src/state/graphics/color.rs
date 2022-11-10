use crate::*;
use lopdf::content::Operation;

#[derive(Debug, Clone)]
pub enum Color {
    CMYK(f32, f32, f32, f32),
    RGB(f32, f32, f32),
    Gray(f32),
}

impl Color {
    pub fn new() -> Self {
        Self::Gray(0.)
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        match operation.operator.as_ref() {
            "g" | "G" => {
                if let Ok([g]) = operand_to_f32(&operation).as_deref() {
                    *self = Self::Gray(*g);
                }
            }
            "rg" | "RG" => {
                if let Ok([r, g, b]) = operand_to_f32(&operation).as_deref() {
                    *self = Self::RGB(*r, *g, *b);
                }
            }
            "k" | "K" => {
                if let Ok([c, m, y, k]) = operand_to_f32(&operation).as_deref() {
                    *self = Self::CMYK(*c, *m, *y, *k);
                }
            }
            "cs" | "CS" => {
                if let Some(name) = operation.operands.get(0).and_then(|o| o.as_name_str().ok()) {
                    match name {
                        "DeviceGray" => {
                            *self = Self::Gray(0.);
                        }
                        "DeviceRGB" => {
                            *self = Self::RGB(0., 0., 0.);
                        }
                        "DeviceCMYK" => {
                            *self = Self::CMYK(0., 0., 0., 1.);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    pub fn equals_to(&self, rhs: &Self) -> bool {
        let l = Self::to_rgb(&self);
        let r = Self::to_rgb(&rhs);
        (l.0 - r.0).abs() <= f32::EPSILON
            && (l.1 - r.1).abs() <= f32::EPSILON
            && (l.2 - r.2).abs() <= f32::EPSILON
    }
    fn to_rgb(color: &Self) -> (f32, f32, f32) {
        match color {
            Self::Gray(g) => (*g, *g, *g),
            Self::RGB(r, g, b) => (*r, *g, *b),
            Self::CMYK(c, m, y, k) => (
                255. * (1. - c) * (1. - k),
                255. * (1. - m) * (1. - k),
                255. * (1. - y) * (1. - k),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorState {
    pub stroke: Color,
    pub non_stroke: Color,
}

impl ColorState {
    pub fn new() -> Self {
        Self {
            stroke: Color::new(),
            non_stroke: Color::new(),
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        match operation.operator.as_ref() {
            "G" | "RG" | "K" | "CS" => self.stroke.handle_operation(operation),
            "g" | "rg" | "k" | "cs" => self.non_stroke.handle_operation(operation),
            _ => (),
        }
    }
}
