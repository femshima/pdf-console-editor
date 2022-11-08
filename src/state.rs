use std::collections::HashMap;

use lopdf::{Document, content::Operation};

use self::graphics::GraphicsState;

pub mod graphics;
pub mod path;

pub struct State {
    pub graphics: GraphicsState,
    pub path: path::Path,
    graphics_dict: HashMap<Vec<u8>, GraphicsState>,
    graphics_stack: Vec<GraphicsState>
}

impl State {
    pub fn new(doc: &Document, page_id: (u32, u16)) -> Self {
        Self {
            graphics: GraphicsState::new(),
            path: path::Path::new(),
            graphics_dict: Self::extgstate_map(doc, page_id),
            graphics_stack:Vec::new()
        }
    }
    fn extgstate_map(doc: &Document, page_id: (u32, u16)) -> HashMap<Vec<u8>, GraphicsState> {
        let mut result = HashMap::new();
        if let Some(resource_dict) = doc
            .get_page_resources(page_id)
            .0
            .and_then(|resource| resource.get(b"ExtGState").ok())
            .and_then(|d| d.as_dict().ok())
        {
            for (k, v) in resource_dict {
                let extgstate_dict = v
                    .as_reference()
                    .and_then(|object_id| doc.get_object(object_id))
                    .and_then(|dict| dict.as_dict());
                if let Some(extgstate) = extgstate_dict
                    .ok()
                    .and_then(|dict| GraphicsState::from_dict(dict).ok())
                {
                    result.insert(k.to_vec(), extgstate);
                }
            }
        }
        result
    }
    pub fn handle_operation(&mut self, operation: &Operation) {
        self.path.handle_operation(&operation);
        self.graphics.handle_operation(&operation);
        match operation.operator.as_ref() {
            "q" => {
                self.graphics_stack.push(self.graphics.clone());
            }
            "Q" => {
                if let Some(state) = self.graphics_stack.pop() {
                    self.graphics = state;
                }
            }
            "gs" => {
                if let Some(state) = operation
                    .operands
                    .get(0)
                    .and_then(|o| o.as_name().ok())
                    .and_then(|name| self.graphics_dict.get(&name.to_vec()))
                {
                    self.graphics = state.clone();
                }
            }
            _ => (),
        }
    }
}
