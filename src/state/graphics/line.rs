use lopdf::{content::Operation, Dictionary};

use crate::operand_to_f32;

#[derive(Debug, Clone)]
enum LineCap {
    ButtCap,
    RoundCap,
    ProjectingSquareCap,
}

impl LineCap {
    pub fn from_i64(i: i64) -> Self {
        match i {
            0 => LineCap::ButtCap,
            1 => LineCap::RoundCap,
            2 => LineCap::ProjectingSquareCap,
            _ => LineCap::ButtCap,
        }
    }
}

#[derive(Debug, Clone)]
enum LineJoin {
    MiterJoin,
    RoundJoin,
    BevelJoin,
}

impl LineJoin {
    pub fn from_i64(i: i64) -> Self {
        match i {
            0 => LineJoin::MiterJoin,
            1 => LineJoin::RoundJoin,
            2 => LineJoin::BevelJoin,
            _ => LineJoin::MiterJoin,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    width: f32,
    cap: LineCap,
    join: LineJoin,
    miter_limit: f32,
    dash_array: Vec<f32>,
    dash_phase: f32,
}

impl Line {
    pub fn new() -> Self {
        Self {
            width: 1.,
            cap: LineCap::ButtCap,
            join: LineJoin::MiterJoin,
            miter_limit: 10.,
            dash_array: vec![],
            dash_phase: 0.,
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        match operation.operator.as_ref() {
            "w" => self.op_set_width(&operation),
            "J" => self.op_set_cap(&operation),
            "j" => self.op_set_join(&operation),
            "M" => self.op_set_miterlimit(&operation),
            "d" => self.op_set_dash(&operation),
            _ => (),
        }
    }
    fn op_set_width(&mut self, operation: &Operation) {
        if let Ok([width]) = operand_to_f32(operation).as_deref() {
            self.width = *width;
        }
    }
    fn op_set_cap(&mut self, operation: &Operation) {
        if let Some(Ok(n)) = operation.operands.get(0).map(|o| o.as_i64()) {
            self.cap = LineCap::from_i64(n);
        }
    }
    fn op_set_join(&mut self, operation: &Operation) {
        if let Some(Ok(n)) = operation.operands.get(0).map(|o| o.as_i64()) {
            self.join = LineJoin::from_i64(n);
        }
    }
    fn op_set_miterlimit(&mut self, operation: &Operation) {
        if let Ok([limit]) = operand_to_f32(operation).as_deref() {
            self.miter_limit = *limit;
        }
    }
    fn op_set_dash(&mut self, operation: &Operation) {
        let array = operation.operands.get(0).and_then(|o| {
            o.as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|o| o.as_float().or(o.as_i64().map(|v| v as f32)).ok())
                        .collect::<Vec<f32>>()
                })
                .ok()
        });
        let phase = operation
            .operands
            .get(1)
            .and_then(|o| o.as_float().or(o.as_i64().map(|v| v as f32)).ok());
        match (array, phase) {
            (Some(a), Some(p)) => {
                self.dash_array = a;
                self.dash_phase = p;
            }
            (_, _) => (),
        }
    }
    pub fn load_dict(&mut self, dict: &Dictionary) {
        if let Ok(width) = dict.get(b"LW").and_then(|o| o.as_f32()) {
            self.width = width
        }
        if let Ok(n) = dict.get(b"LC").and_then(|o| o.as_i64()) {
            self.cap = LineCap::from_i64(n);
        }
        if let Ok(n) = dict.get(b"LJ").and_then(|o| o.as_i64()) {
            self.join = LineJoin::from_i64(n);
        }
        if let Ok(limit) = dict.get(b"ML").and_then(|o| o.as_f32()) {
            self.miter_limit = limit
        }
        if let Ok(dash) = dict.get(b"D").and_then(|o| o.as_array()) {
            self.op_set_dash(&Operation::new("d", dash.to_vec()));
        }
    }
}
