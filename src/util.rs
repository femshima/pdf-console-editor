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

