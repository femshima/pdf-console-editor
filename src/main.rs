use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use pdf_console_editor::*;

use clap::Parser;

#[derive(Debug)]
#[derive(Parser)]
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

fn process_content(content: Content, args: &Cli) -> Content {
    let mut operations: Vec<Operation> = Vec::new();

    let mut path = Path::new();
    let mut color = ColorState::new();
    for operation in content.operations {
        path.handle_operation(&operation);
        color.handle_operation(&operation);

        // println!("{},{:?}", operation.operator, operation.operands);
        match operation.operator.as_ref() {
            "f" | "F" | "f*" => {
                if args.rectangle && path.is_rect((args.lower_bound_edge, args.upper_bound_edge)) {
                    operations.push(Operation::new("n", vec![]))
                } else {
                    operations.push(operation);
                }
            }
            "TJ" => {
                if args.colored_text && color.non_stroke.is_white() {
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
        let content_data = doc.get_page_content(page_id).unwrap();
        let content = Content::decode(&content_data).unwrap();

        let modified_content = process_content(content, &args).encode().unwrap();
        doc.change_page_content(page_id, modified_content).unwrap();
    }
    doc.compress();
    doc.save(&args.output).unwrap();
}
