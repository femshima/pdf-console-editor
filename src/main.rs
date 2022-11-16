use lopdf::content::Operation;
use lopdf::Object;
use pdf_console_editor::*;

use clap::Parser;

mod argparse;
use argparse::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    rectangle: bool,
    #[arg(short, long, value_parser=ArgRange::parser, default_value_t=ArgRange::default())]
    edge_length: ArgRange,

    #[arg(short, long, value_parser=ArgColor::parser)]
    colored_text: Vec<ArgColor>,

    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    transform_pdf(
        &args.input,
        &args.output,
        |operation, state| match operation.operator.as_ref() {
            "f" | "F" | "f*" => {
                if args.rectangle && state.path.is_rect(args.edge_length.to_f32_f32()) {
                    vec![Operation::new("n", vec![])]
                } else {
                    vec![operation]
                }
            }
            "TJ" | "Tj" => {
                if args
                    .colored_text
                    .iter()
                    .any(|c| c.0.equals_to(&state.graphics.color.non_stroke))
                {
                    vec![
                        Operation::new(
                            "rg",
                            vec![Object::from(0.), Object::from(0.), Object::from(1.)],
                        ),
                        operation,
                        state.graphics.color.operator_non_stroke(),
                    ]
                } else {
                    vec![operation]
                }
            }
            _ => {
                vec![operation]
            }
        },
    )
}
