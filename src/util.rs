use lopdf::content::Operation;

pub fn operand_to_f32(op: &Operation) -> Result<Vec<f32>, ()> {
    let mut res = Vec::<f32>::new();
    for operand in &op.operands {
        let f = operand.as_float().or(operand.as_i64().map(|v| v as f32));
        if let Ok(r) = f {
            res.push(r)
        } else {
            return Err(());
        }
    }
    Ok(res)
}

pub struct CoordMatrix(f32, f32, f32, f32, f32, f32);
impl CoordMatrix {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self(a, b, c, d, e, f)
    }
    pub fn offset(x: f32, y: f32) -> Self {
        Self(1., 0., 0., 1., x, y)
    }
}
impl From<CoordMatrix> for na::Matrix3<f32> {
    fn from(m: CoordMatrix) -> Self {
        Self::new(m.0, m.1, 0., m.2, m.3, 0., m.4, m.5, 1.)
    }
}
