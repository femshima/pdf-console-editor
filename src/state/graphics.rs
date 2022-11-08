use lopdf::{content::Operation, Dictionary};

use crate::{operand_to_f32, CoordMatrix};

use self::{color::ColorState, line::Line, text::Text};

pub mod color;
pub mod line;
pub mod text;

#[derive(Debug, Clone)]
pub struct GraphicsState {
    //CTM
    pub ctm: na::Matrix3<f32>,

    //clipping path

    //color space
    //color
    pub color: ColorState,

    //text state
    pub text: Text,

    //line width
    //line cap
    //line join
    //miter limit
    //dash pattern
    pub line: Line,

    //rendering intent

    //stroke adjustment
    pub stroke_adjustment: bool,

    //blend mode
    //soft mask

    //alpha constant
    pub alpha_constant_stroke: f32,
    pub alpha_constant_non_stroke: f32,
    //alpha source
    pub alpha_source: bool,
}

impl GraphicsState {
    pub fn new() -> Self {
        Self {
            ctm: na::Matrix3::identity(),
            color: ColorState::new(),
            text: Text::new(),
            line: Line::new(),
            stroke_adjustment: false,
            alpha_constant_stroke: 1.,
            alpha_constant_non_stroke: 1.,
            alpha_source: false,
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        self.color.handle_operation(&operation);
        self.text.handle_operation(&operation);
        self.line.handle_operation(&operation);
        match operation.operator.as_ref() {
            "cm" => {
                if let Ok([a, b, c, d, e, f]) = operand_to_f32(operation).as_deref() {
                    let m: na::Matrix3<f32> = CoordMatrix::new(*a, *b, *c, *d, *e, *f).into();
                    self.ctm = m * self.ctm;
                }
            }
            _ => (),
        }
    }
    pub fn from_dict(dict: &Dictionary) -> Result<Self, ()> {
        if !dict
            .get(b"Type")
            .and_then(|t| t.as_name_str())
            .map_or(false, |v| v == "ExtGState")
        {
            return Err(());
        }

        let mut result = Self::new();
        result.text = Text::from_dict(&dict);
        result.line = Line::from_dict(&dict);
        if let Ok(alpha_constant) = dict
            .get(b"CA")
            .and_then(|o| o.as_float().or(o.as_i64().map(|v| v as f32)))
        {
            result.alpha_constant_stroke = alpha_constant;
        }
        if let Ok(alpha_constant) = dict
            .get(b"ca")
            .and_then(|o| o.as_float().or(o.as_i64().map(|v| v as f32)))
        {
            result.alpha_constant_non_stroke = alpha_constant;
        }
        if let Ok(alpha_source) = dict.get(b"TK").and_then(|o| o.as_bool()) {
            result.alpha_source = alpha_source;
        }
        Ok(result)
    }
}
