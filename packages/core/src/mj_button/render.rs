use super::MJButton;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;

struct MJButtonRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJButton,
}

impl<'e, 'h> Render for MJButtonRender<'e, 'h> {
    fn render(&self, buf: &mut String) -> Result<(), Error> {
        // TODO
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJButton {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render + 'r> {
        Box::new(MJButtonRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
