use super::MJML;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MJMLRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJML,
}

impl<'e, 'h> Render for MJMLRender<'e, 'h> {
    fn render(&self, buf: &mut String) -> Result<(), Error> {
        let mut body_content = String::default();
        if let Some(body) = self.element.body() {
            body.renderer(Rc::clone(&self.header))
                .render(&mut body_content)?;
        }
        buf.push_str("<!doctype html>");
        buf.push_str("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        // TODO
        buf.push_str("</html>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJML {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render + 'r> {
        Box::new(MJMLRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

impl MJML {
    pub fn render(&self) -> Result<String, Error> {
        let header = Rc::new(RefCell::new(Header::new(&self.head)));
        let mut buffer = String::default();
        self.renderer(header).render(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let elt = MJML::default();
        assert_eq!("<!doctype html><html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\"></html>", elt.render().unwrap());
    }
}
