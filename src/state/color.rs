use crate::*;
use lopdf::content::Operation;

#[derive(Debug, Clone)]
enum ColorSpace {
    CMYK,
    RGB,
    Gray,
}

#[derive(Debug, Clone)]
pub struct Color {
    space: ColorSpace,
    data: (f32, f32, f32, f32),
}

impl Color {
    pub fn new() -> Self {
        Self {
            space: ColorSpace::Gray,
            data: (0., 0., 0., 0.),
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        match operation.operator.as_ref() {
            "g" | "G" => {
                if let Ok([g]) = operand_to_f32(&operation).as_deref() {
                    self.space = ColorSpace::Gray;
                    self.data = (*g, 0., 0., 0.);
                }
            }
            "rg" | "RG" => {
                if let Ok([r, g, b]) = operand_to_f32(&operation).as_deref() {
                    self.space = ColorSpace::RGB;
                    self.data = (*r, *g, *b, 0.);
                }
            }
            "k" | "K" => {
                if let Ok([c, m, y, k]) = operand_to_f32(&operation).as_deref() {
                    self.space = ColorSpace::CMYK;
                    self.data = (*c, *m, *y, *k);
                }
            }
            "cs" | "CS" => {
                if let Some(name) = operation.operands.get(0).and_then(|o| o.as_name().ok()) {
                    match name {
                        b"DeviceGray" => {
                            self.space = ColorSpace::Gray;
                            self.data = (0., 0., 0., 0.);
                        }
                        b"DeviceRGB" => {
                            self.space = ColorSpace::RGB;
                            self.data = (0., 0., 0., 0.);
                        }
                        b"DeviceCMYK" => {
                            self.space = ColorSpace::CMYK;
                            self.data = (0., 0., 0., 0.);
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
        match self.space {
            ColorSpace::Gray => eq(self.data.0, 1.),
            ColorSpace::RGB => eq(self.data.0, 1.) && eq(self.data.1, 1.) && eq(self.data.2, 1.),
            ColorSpace::CMYK => {
                eq(self.data.0, 0.)
                    && eq(self.data.1, 0.)
                    && eq(self.data.2, 0.)
                    && eq(self.data.3, 0.)
            }
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
