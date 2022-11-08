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
    pub fn is_white(&self) -> bool {
        fn eq(v: f32, c: f32) -> bool {
            (v - c).abs() < 1e-6
        }
        match self {
            Self::Gray(g) => eq(*g, 1.),
            Self::RGB(r, g, b) => eq(*r, 1.) && eq(*g, 1.) && eq(*b, 1.),
            Self::CMYK(c, m, y, k) => eq(*c, 0.) && eq(*m, 0.) && eq(*y, 0.) && eq(*k, 0.),
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
