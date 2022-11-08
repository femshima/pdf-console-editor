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

fn process_content<F>(doc: &Document, page_id: (u32, u16), f: F) -> Content
where
    F: Fn(Operation, &State) -> Vec<Operation>,
{
    let content_data = doc.get_page_content(page_id).unwrap();
    let content = Content::decode(&content_data).unwrap();

    let mut state = State::new(doc, page_id);

    let mut operations: Vec<Operation> = Vec::new();
    for operation in content.operations {
        state.handle_operation(&operation);
        operations.extend(f(operation, &state));
    }
    Content { operations }
}

fn main() {
    let args = Cli::parse();
    let mut doc = Document::load(&args.input).unwrap();
    let pages: Vec<(u32, u16)> = doc.page_iter().collect();
    // let page_id = doc.page_iter().nth(11).unwrap();
    for page_id in pages {
        let modified_content = process_content(&doc, page_id, |operation, state| {
            match operation.operator.as_ref() {
                "f" | "F" | "f*" => {
                    if args.rectangle
                        && state
                            .path
                            .is_rect((args.lower_bound_edge, args.upper_bound_edge))
                    {
                        vec![Operation::new("n", vec![])]
                    } else {
                        vec![operation]
                    }
                }
                "TJ" | "Tj" => {
                    if args.colored_text && state.graphics.color.non_stroke.is_white() {
                        vec![
                            Operation::new(
                                "rg",
                                vec![Object::from(0.), Object::from(0.), Object::from(1.)],
                            ),
                            operation,
                        ]
                    } else {
                        vec![operation]
                    }
                }
                _ => {
                    vec![operation]
                }
            }
        })
        .encode()
        .unwrap();
        doc.change_page_content(page_id, modified_content).unwrap();
    }
    doc.compress();
    doc.save(&args.output).unwrap();
}
