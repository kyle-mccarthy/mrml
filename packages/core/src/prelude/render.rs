use crate::mj_head::MJHead;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {}

pub struct Header<'h> {
    head: &'h Option<MJHead>,
    font_families: HashSet<String>,
    styles: HashSet<String>,
}

impl<'H> Header<'H> {
    pub fn new(head: &'H Option<MJHead>) -> Self {
        Self {
            head,
            font_families: HashSet::new(),
            styles: HashSet::new(),
        }
    }

    pub fn head(&self) -> &Option<MJHead> {
        &self.head
    }
}

pub trait Render {
    fn set_index(&mut self, _index: usize) {}
    fn set_siblings(&mut self, _count: usize) {}
    fn set_raw_siblings(&mut self, _count: usize) {}

    fn render(&self, buf: &mut String) -> Result<(), Error>;
}

pub trait Renderable<'r, 'e: 'r, 'h: 'r> {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render + 'r>;
}
