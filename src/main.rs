use std::collections::HashMap;

use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use pdf_console_editor::*;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    rectangle: bool,
    #[arg(short, long, default_value_t = 20.)]
    lower_bound_edge: f32,
    #[arg(short, long, default_value_t = 200.)]
    upper_bound_edge: f32,

    #[arg(short, long)]
    colored_text: bool,

    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn extgstate_map(doc: &Document, page_id: (u32, u16)) -> HashMap<&Vec<u8>, GraphicsState> {
    let mut result = HashMap::new();
    if let Some(resource_dict) = doc
        .get_page_resources(page_id)
        .0
        .and_then(|resource| resource.get(b"ExtGState").ok())
        .and_then(|d| d.as_dict().ok())
    {
        for (k, v) in resource_dict {
            dbg!(k, v);
            let extgstate_dict = v
                .as_reference()
                .and_then(|object_id| doc.get_object(object_id))
                .and_then(|dict| dict.as_dict());
            if let Some(extgstate) = extgstate_dict
                .ok()
                .and_then(|dict| GraphicsState::from_dict(dict).ok())
            {
                result.insert(k, extgstate);
            }
        }
    }
    result
}

fn process_content(doc: &Document, page_id: (u32, u16), args: &Cli) -> Content {
    let content_data = doc.get_page_content(page_id).unwrap();
    let content = Content::decode(&content_data).unwrap();
    let graphics_state_dict = extgstate_map(doc, page_id);

    let mut operations: Vec<Operation> = Vec::new();

    let mut graphics_state_stack: Vec<GraphicsState> = Vec::new();
    let mut graphics_state = GraphicsState::new();
    let mut path = path::Path::new();
    for operation in content.operations {
        path.handle_operation(&operation);
        graphics_state.handle_operation(&operation);
        // dbg!(&operation);
        // dbg!(&graphics_state);

        // println!("{},{:?}", operation.operator, operation.operands);
        match operation.operator.as_ref() {
            "q" => {
                graphics_state_stack.push(graphics_state.clone());
                operations.push(operation);
            }
            "Q" => {
                if let Some(state) = graphics_state_stack.pop() {
                    graphics_state = state;
                }
                operations.push(operation);
            }
            "gs" => {
                if let Some(state) = operation
                    .operands
                    .get(0)
                    .and_then(|o| o.as_name().ok())
                    .and_then(|name| graphics_state_dict.get(&name.to_vec()))
                {
                    graphics_state = state.clone();
                }
                operations.push(operation);
            }
            "f" | "F" | "f*" => {
                if args.rectangle && path.is_rect((args.lower_bound_edge, args.upper_bound_edge)) {
                    operations.push(Operation::new("n", vec![]))
                } else {
                    operations.push(operation);
                }
            }
            "TJ" | "Tj" => {
                if args.colored_text && graphics_state.color.non_stroke.is_white() {
                    operations.push(Operation::new(
                        "rg",
                        vec![Object::from(0.), Object::from(0.), Object::from(1.)],
                    ));
                }
                operations.push(operation);
            }
            _ => {
                operations.push(operation);
            }
        }
    }
    Content { operations }
}

fn main() {
    let args = Cli::parse();
    let mut doc = Document::load(&args.input).unwrap();
    let pages: Vec<(u32, u16)> = doc.page_iter().collect();
    // let page_id = doc.page_iter().nth(11).unwrap();
    for page_id in pages {
        let modified_content = process_content(&doc, page_id, &args).encode().unwrap();
        doc.change_page_content(page_id, modified_content).unwrap();
    }
    doc.compress();
    doc.save(&args.output).unwrap();
}
