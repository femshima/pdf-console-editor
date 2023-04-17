use kurbo::{BezPath, Point, Shape};
use lopdf::content::Operation;
use lopdf::Object;
use pdf_console_editor::*;

use clap::Parser;

mod argparse;
use argparse::*;
use pdf_console_editor::graphics::color::Color;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    rectangle: bool,
    #[arg(short, long, value_parser=ArgRange::parser, default_value_t=ArgRange::default())]
    edge_length: ArgRange,

    #[arg(short, long, value_parser=ArgColor::parser)]
    colored_text: Vec<ArgColor>,

    #[arg(short, long)]
    background_color: bool,

    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut modifier = PdfModifier::new(&args.input).unwrap();
    for page_id in modifier.pages() {
        let mut paths: Vec<(usize, BezPath, Color)> = Vec::new();
        let mut objects: Vec<(usize, Point)> = Vec::new();
        if args.background_color {
            modifier.for_each(page_id, &mut |operation, state| {
                match operation.operator.as_ref() {
                    "f" | "F" | "f*" => paths.extend(state.path.subpaths().iter().map(|path| {
                        (
                            state.id,
                            (*path).to_owned(),
                            state.graphics.color.non_stroke.to_owned(),
                        )
                    })),
                    _ => objects.push((
                        state.id,
                        state.graphics.text.line_matrix.translation().to_point(),
                    )),
                };
            });
        }
        modifier.apply(
            page_id,
            &mut |operation, state| match operation.operator.as_ref() {
                "f" | "F" | "f*" => {
                    let need_removal = if args.background_color {
                        objects.iter().any(|(i, point)| {
                            *i < state.id
                                && state
                                    .path
                                    .subpaths()
                                    .iter()
                                    .any(|path| path.contains(*point))
                        })
                    } else {
                        args.rectangle && state.path.is_rect(args.edge_length.to_f32_f32())
                    };
                    if need_removal {
                        vec![Operation::new("n", vec![])]
                    } else {
                        vec![operation]
                    }
                }
                "TJ" | "Tj" => {
                    let need_replace = if args.background_color {
                        if let Some(background) = paths.iter().rfind(|(i, path, _color)| {
                            let t = state.graphics.text.line_matrix.translation();
                            *i < state.id && path.contains(t.to_point())
                        }) {
                            background.2.equals_to(&state.graphics.color.non_stroke)
                        } else {
                            false
                        }
                    } else {
                        args.colored_text
                            .iter()
                            .any(|c| c.0.equals_to(&state.graphics.color.non_stroke))
                    };
                    if need_replace {
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
        );
    }
    modifier.save(&args.output);
}
