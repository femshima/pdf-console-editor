use lopdf::{content::Operation, Dictionary};

use crate::operand_to_f32;

#[derive(Debug, Clone)]
pub enum RenderingMode {
    Fill,
    Stroke,
    FillAndStroke,
    Invisible,
    FillAndAddClippingPath,
    StrokeAndAddClippingPath,
    FillStrokeAddClippingPath,
    AddClippingPath,
}

impl RenderingMode {
    pub fn from_i64(render: i64) -> Self {
        match render {
            0 => RenderingMode::Fill,
            1 => RenderingMode::Stroke,
            2 => RenderingMode::FillAndStroke,
            3 => RenderingMode::Invisible,
            4 => RenderingMode::FillAndAddClippingPath,
            5 => RenderingMode::StrokeAndAddClippingPath,
            6 => RenderingMode::FillStrokeAddClippingPath,
            7 => RenderingMode::AddClippingPath,
            _ => RenderingMode::Fill,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub charactor_spacing: f32,
    pub word_spacing: f32,
    pub horizontal_scaling: f32,
    pub leading: f32,
    // font: Option<Vec<u8>>,
    pub font_size: Option<f32>,
    pub rendering_mode: RenderingMode,
    pub rise: f32,
    pub knockout: bool,

    // matrix: na::Matrix3<f32>,
    pub line_matrix: kurbo::Affine,
}

impl Text {
    pub fn new() -> Self {
        Self {
            charactor_spacing: 0.,
            word_spacing: 0.,
            horizontal_scaling: 100.,
            leading: 0.,
            // font: None,
            font_size: None,
            rendering_mode: RenderingMode::Fill,
            rise: 0.,
            knockout: true,
            // matrix: na::Matrix3::identity(),
            line_matrix: kurbo::Affine::IDENTITY,
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        match operation.operator.as_ref() {
            "Tc" => {
                if let Ok([spacing]) = operand_to_f32(operation).as_deref() {
                    self.charactor_spacing = *spacing;
                }
            }
            "Tw" => {
                if let Ok([spacing]) = operand_to_f32(operation).as_deref() {
                    self.word_spacing = *spacing;
                }
            }
            "Tz" => {
                if let Ok([scale]) = operand_to_f32(operation).as_deref() {
                    self.horizontal_scaling = *scale;
                }
            }
            "TL" => {
                if let Ok([leading]) = operand_to_f32(operation).as_deref() {
                    self.leading = *leading;
                }
            }
            "Tf" => {
                let font = operation.operands.get(0).and_then(|o| o.as_name_str().ok());
                let font_size = operation
                    .operands
                    .get(1)
                    .and_then(|o| o.as_float().or(o.as_i64().map(|v| v as f32)).ok());
                match (font, font_size) {
                    (Some(_f), Some(fs)) => {
                        // self.font = Some(f);
                        self.font_size = Some(fs);
                    }
                    (_, _) => (),
                }
            }
            "Tr" => {
                if let Some(Ok(render)) = operation.operands.get(0).map(|o| o.as_i64()) {
                    self.rendering_mode = RenderingMode::from_i64(render);
                }
            }
            "Ts" => {
                if let Ok([rise]) = operand_to_f32(operation).as_deref() {
                    self.rise = *rise;
                }
            }
            "BT" => {
                self.line_matrix = kurbo::Affine::IDENTITY;
                // self.matrix = na::Matrix3::identity();
            }
            "ET" => {
                self.line_matrix = kurbo::Affine::IDENTITY;
                // self.matrix = na::Matrix3::identity();
            }
            "Td" => {
                if let Ok([x, y]) = operand_to_f32(operation).as_deref() {
                    self.line_matrix *= kurbo::Affine::translate(((*x).into(), (*y).into()));
                    // self.matrix = self.line_matrix;
                }
            }
            "TD" => {
                if let Ok([x, y]) = operand_to_f32(operation).as_deref() {
                    self.leading = -y;
                    self.line_matrix *= kurbo::Affine::translate(((*x).into(), (*y).into()));
                    // self.matrix = self.line_matrix;
                }
            }
            "Tm" => {
                if let Ok([a, b, c, d, e, f]) = operand_to_f32(operation).as_deref() {
                    self.line_matrix = kurbo::Affine::new([*a, *b, *c, *d, *e, *f].map(f32::into));
                    // self.matrix = self.line_matrix;
                }
            }
            "T*" | "'" => {
                self.line_matrix *= kurbo::Affine::translate((0., (-self.leading).into()));
                // self.matrix = self.line_matrix;
            }
            "\"" => {
                if let [aw, ac] = operation.operands[0..2]
                    .iter()
                    .filter_map(|o| o.as_f32().or(o.as_i64().map(|v| v as f32)).ok())
                    .collect::<Vec<f32>>()[..]
                {
                    self.word_spacing = aw;
                    self.charactor_spacing = ac;
                }
            }
            _ => (),
        }
    }
    pub fn load_dict(&mut self, dict: &Dictionary) {
        if let Ok(knockout) = dict.get(b"TK").and_then(|o| o.as_bool()) {
            self.knockout = knockout
        }
        if let Ok(opts) = dict.get(b"Font").and_then(|o| o.as_array()) {
            let font = opts.get(0).and_then(|o| o.as_reference().ok());
            let font_size = opts.get(1)
                .and_then(|o| o.as_float().or(o.as_i64().map(|v| v as f32)).ok());
            match (font, font_size) {
                (Some(_f), Some(fs)) => {
                    // self.font = Some(f);
                    self.font_size = Some(fs);
                }
                (_, _) => (),
            }
        }
    }
}
