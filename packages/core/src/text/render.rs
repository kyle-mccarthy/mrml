use super::Text;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;

struct TextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Text,
}

impl<'e, 'h> Render for TextRender<'e, 'h> {
    fn render(&self, buf: &mut String) -> Result<(), Error> {
        buf.push_str(&self.element.0);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Text {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render + 'r> {
        Box::new(TextRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
