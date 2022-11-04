use crate::*;
use lopdf::content::Operation;

#[derive(Debug)]
pub struct Path {
    operations: Vec<Operation>,
    path_invalid: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            path_invalid: false,
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        if self.path_invalid {
            self.operations.clear();
            self.path_invalid = false;
        }
        match operation.operator.as_ref() {
            "re" | "m" | "l" | "h" => {
                self.operations.push(operation.clone());
            }
            "b" | "B" | "b*" | "B*" | "f" | "F" | "f*" | "s" | "S" | "n" => {
                self.operations.push(operation.clone());
                self.path_invalid = true;
            }
            _ => (),
        }
    }
    pub fn get_operations(&self) -> Vec<Operation> {
        self.operations.to_vec()
    }
    pub fn is_rect(&self, between: (f32, f32)) -> bool {
        let mut points: Vec<(f32, f32)> = Vec::new();
        // dbg!(&self.operations);
        for op in &self.operations[..] {
            match op.operator.as_ref() {
                "re" => {
                    if let Ok(operands) = operand_to_f32(&op) {
                        if let [x, y, width, height] = operands[..] {
                            points.push((x, y));
                            points.push((x, y + height));
                            points.push((x + width, y + height));
                            points.push((x + width, y));
                            points.push((x, y));
                            continue;
                        }
                    }
                }
                "m" | "l" => {
                    if let Ok(operands) = operand_to_f32(&op) {
                        if operands.len() == 2 {
                            points.push((operands[0], operands[1]));
                            continue;
                        }
                    }
                }
                "h" => {
                    if points.len() > 0 {
                        points.push(points[0]);
                        continue;
                    }
                }
                "b" | "b*" | "s" => {
                    if points.len() > 0 {
                        points.push(points[0]);
                        break;
                    }
                }
                "B" | "B*" | "f" | "F" | "f*" | "S" | "n" => {
                    break;
                }
                _ => {
                    return false;
                }
            }
        }
        if points.len() < 5 {
            return false;
        }
        points.push(points[1]);
        let edges: Vec<(f32, f32)> = points[..points.len() - 1]
            .iter()
            .zip(points[1..].iter())
            .map(|(p1, p2)| (p1.0 - p2.0, p1.1 - p2.1))
            .collect();
        let edge_len = edges
            .iter()
            .map(|(x, y)| (x.powi(2) + y.powi(2)).sqrt())
            .all(|l| between.0 < l && l < between.1);
        let vertex_count = edges[..edges.len() - 1]
            .iter()
            .zip(edges[1..].iter())
            .map(|(p1, p2)| p1.0 * p2.0 + p1.1 * p2.1)
            .all(|d| d.abs() < f32::EPSILON * 256.);
        vertex_count && edge_len
    }
}
