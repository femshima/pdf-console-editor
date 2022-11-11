use crate::*;
use kurbo::{BezPath, PathEl, Point, Rect, Shape, Size};
use lopdf::content::Operation;

#[derive(Debug, Clone)]
pub struct Path {
    paths: Vec<BezPath>,
    current_point: Option<Point>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            current_point: None,
        }
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        if self.current_point == None {
            self.paths.clear();
        }
        if self.paths.is_empty()
            || self.paths.last().unwrap().elements().last() == Some(&PathEl::ClosePath)
        {
            self.paths.push(BezPath::new());
        }
        let last = self.paths.last_mut().unwrap();
        // println!(
        //     "{},{:?},{:?}",
        //     &operation.operator, &operation.operands, &last
        // );

        let operands = operand_to_f32(operation)
            .map(|v| v.iter().map(|val| f64::from(*val)).collect::<Vec<_>>());

        match operation.operator.as_ref() {
            "m" => {
                if let Ok([x, y]) = operands.as_deref() {
                    let p = Point::new(*x, *y);
                    last.move_to(p);
                    self.current_point = Some(p);
                }
            }
            "l" => {
                if let Ok([x, y]) = operands.as_deref() {
                    let p = Point::new(*x, *y);
                    last.line_to(p);
                    self.current_point = Some(p);
                }
            }
            "c" => {
                if let Ok([x1, y1, x2, y2, x3, y3]) = operands.as_deref() {
                    let p1 = Point::new(*x1, *y1);
                    let p2 = Point::new(*x2, *y2);
                    let p3 = Point::new(*x3, *y3);
                    last.curve_to(p1, p2, p3);
                    self.current_point = Some(p3);
                }
            }
            "v" => {
                if let Ok([x2, y2, x3, y3]) = operands.as_deref() {
                    let p1 = self.current_point.unwrap();
                    let p2 = Point::new(*x2, *y2);
                    let p3 = Point::new(*x3, *y3);
                    last.curve_to(p1, p2, p3);
                    self.current_point = Some(p3);
                }
            }
            "y" => {
                if let Ok([x1, y1, x3, y3]) = operands.as_deref() {
                    let p1 = Point::new(*x1, *y1);
                    let p3 = Point::new(*x3, *y3);
                    last.curve_to(p1, p3, p3);
                    self.current_point = Some(p3);
                }
            }
            "re" => {
                if let Ok([x, y, width, height]) = operands.as_deref() {
                    let p = Point::new(*x, *y);
                    let s = Size::new(*width, *height);
                    let rect = Rect::from_origin_size(p, s);
                    self.paths.push(rect.into_path(0.01));
                    self.current_point = Some(p + s.to_vec2());
                }
            }
            "h" => {
                last.close_path();
            }
            "s" | "b" | "b*" => {
                last.close_path();
                self.current_point = None;
            }
            "S" | "f" | "F" | "f*" | "B" | "B*" | "n" => {
                self.current_point = None;
            }
            _ => (),
        }
    }
    pub fn subpaths(&self) -> Vec<&BezPath> {
        self.paths
            .iter()
            .filter(|path| !path.elements().is_empty() && path.bounding_box().area() > 0.)
            .collect::<Vec<_>>()
    }
    pub fn is_rect(&self, between: (f32, f32)) -> bool {
        let filtered = self.subpaths();
        if filtered.len() != 1 {
            return false;
        }
        let path = filtered.last().unwrap();
        let rect = path.bounding_box();
        let size = rect.size();
        let l: f64 = between.0.into();
        let u: f64 = between.1.into();
        l <= size.width && size.width < u && l <= size.height && size.height < u
    }
}
