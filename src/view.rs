use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List, ListItem},
};

use crate::{Index, Object};

pub fn render_index(index: Index, area: Rect, frame: &mut Frame) {
    let block = Block::bordered().title("Index");

    let items = index
        .entries
        .iter()
        .map(|entry| ListItem::new(format!("- {} {:?}", entry.file_name, entry.blob_hash.0)))
        .collect::<Vec<ListItem>>();

    let list = List::new(items).block(block);

    frame.render_widget(list, area);
}

pub struct ObjectView {
    pub objects: Vec<Object>,
}

impl ObjectView {
    pub fn new(objects: Vec<Object>) -> Self {
        Self { objects }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered().title("Objects");

        let items = self
            .objects
            .iter()
            .map(object_item)
            .collect::<Vec<ListItem>>();

        let list = List::new(items).block(block);

        frame.render_widget(list, area);
    }
}

pub fn object_item(object: &Object) -> ListItem {
    match object {
        Object::Blob { hash, content } => ListItem::new(format!("- blob {:?} {}", hash, content)),
        Object::Tree { hash, contents } => {
            ListItem::new(format!("- tree {:?} {:?}", hash, contents))
        }
        Object::Commit { hash, .. } => ListItem::new(format!("- commit {:?}", hash)),
    }
}
