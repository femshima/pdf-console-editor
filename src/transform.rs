
use std::path::PathBuf;

use lopdf::content::{Content, Operation};
use lopdf::{Document};

use crate::State;

pub fn process_content<F>(doc: &Document, page_id: (u32, u16), f: F) -> Content
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

pub fn transform_pdf<F>(input_path: &PathBuf, output_path: &PathBuf, converter: F)
where
    F: Fn(Operation, &State) -> Vec<Operation>,
{
    let mut doc = Document::load(input_path).unwrap();
    let pages: Vec<(u32, u16)> = doc.page_iter().collect();
    // let page_id = doc.page_iter().nth(11).unwrap();
    for page_id in pages {
        let modified_content = process_content(&doc, page_id, &converter).encode().unwrap();
        doc.change_page_content(page_id, modified_content).unwrap();
    }
    doc.compress();
    doc.save(&output_path).unwrap();
}