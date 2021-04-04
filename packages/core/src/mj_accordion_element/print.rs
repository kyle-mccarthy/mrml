use super::MJAccordionElement;
use crate::prelude::print::{print_close, print_open, Print};
use std::fmt;

impl Print for MJAccordionElement {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(
            f,
            super::NAME,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        );
        self.title.as_ref().map(|child| {
            child.print(f, pretty, level + 1, indent_size);
        });
        self.text.as_ref().map(|child| {
            child.print(f, pretty, level + 1, indent_size);
        });
        print_close(f, super::NAME, pretty, level, indent_size);
    }
}

impl fmt::Display for MJAccordionElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_element::MJAccordionElement::default();
        assert_eq!(
            "<mj-accordion-element></mj-accordion-element>",
            item.dense_print()
        );
    }
}
