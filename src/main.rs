use lopdf::content::Operation;
use lopdf::Object;
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

fn main() {
    let args = Cli::parse();
    transform_pdf(
        &args.input,
        &args.output,
        |operation, state| match operation.operator.as_ref() {
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
        },
    )
}
