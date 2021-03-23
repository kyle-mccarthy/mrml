use super::MJBody;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;

struct MJBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJBody,
}

impl<'e, 'h> MJBodyRender<'e, 'h> {
    fn attribute(&self, name: &str) -> Option<&String> {
        self.element.attributes.get(name)
    }

    fn push_body_style(&self, buf: &mut String) {
        if let Some(bg_color) = self.attribute("background-color") {
            buf.push_str(" style=\"background-color:");
            buf.push_str(bg_color);
            buf.push_str("\"");
        }
    }

    fn render_preview(&self, buf: &mut String) {
        if let Some(value) = self
            .header
            .borrow()
            .head()
            .as_ref()
            .and_then(|h| h.preview())
            .map(|p| p.content())
        {
            buf.push_str(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">"#);
            buf.push_str(value);
            buf.push_str("</div>");
        }
    }

    fn render_content(&self, buf: &mut String) -> Result<(), Error> {
        buf.push_str("<div");
        if let Some(class) = self.attribute("css-class") {
            buf.push_str(" class=\"");
            buf.push_str(class);
            buf.push_str("\"");
        }
        self.push_body_style(buf);
        buf.push_str("\">");
        for child in self.element.children.iter() {
            child.renderer(Rc::clone(&self.header)).render(buf)?;
        }
        buf.push_str("</div>");
        Ok(())
    }
}

impl<'e, 'h> Render for MJBodyRender<'e, 'h> {
    fn render(&self, buf: &mut String) -> Result<(), Error> {
        buf.push_str("<body");
        self.push_body_style(buf);
        buf.push_str(">");
        self.render_preview(buf);
        self.render_content(buf)?;
        buf.push_str("</body>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJBody {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render + 'r> {
        Box::new(MJBodyRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
