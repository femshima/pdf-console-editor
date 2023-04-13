use std::path::PathBuf;

use lopdf::content::{Content, Operation};
use lopdf::Document;

use crate::State;

pub struct PdfModifier {
    doc: Document,
}

impl PdfModifier {
    pub fn new(input_path: &PathBuf) -> Result<Self, lopdf::Error> {
        Ok(Self {
            doc: Document::load(input_path)?,
        })
    }
    pub fn save(&mut self, output_path: &PathBuf) {
        self.doc.compress();
        self.doc.save(&output_path).unwrap();
    }

    pub fn apply<F>(&mut self, converter: F)
    where
        F: Fn(Operation, &State) -> Vec<Operation>,
    {
        let pages: Vec<(u32, u16)> = self.doc.page_iter().collect();
        for page_id in pages {
            let content_data = self.doc.get_page_content(page_id).unwrap();
            let content = Content::decode(&content_data).unwrap();

            let mut state = State::new(&self.doc, page_id);

            let mut operations: Vec<Operation> = Vec::new();
            for operation in content.operations {
                state.handle_operation(&operation);
                operations.extend(converter(operation, &state));
            }
            let modified_content = Content { operations }.encode().unwrap();
            self.doc
                .change_page_content(page_id, modified_content)
                .unwrap();
        }
    }

    pub fn forEach<F>(&mut self, converter: F)
    where
        F: Fn(Operation, &State),
    {
        for page_id in self.doc.page_iter() {
            let content_data = self.doc.get_page_content(page_id).unwrap();
            let content = Content::decode(&content_data).unwrap();

            let mut state = State::new(&self.doc, page_id);

            for operation in content.operations {
                state.handle_operation(&operation);
                converter(operation, &state);
            }
        }
    }
}
