use super::{MJHead, MJHeadChild, NAME};
use crate::print_children;

impl MJHeadChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJAttributes(elt) => elt,
            Self::MJBreakpoint(elt) => elt,
            Self::MJFont(elt) => elt,
            Self::MJPreview(elt) => elt,
            Self::MJRaw(elt) => elt,
            Self::MJStyle(elt) => elt,
            Self::MJTitle(elt) => elt,
        }
    }
}

impl Print for MJHeadChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

print_children!(MJHead, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_head::MJHead::default();
        assert_eq!("<mj-head></mj-head>", item.dense_print());
    }

    #[test]
    fn with_all() {
        let origin = r#"<mjml>
  <mj-head>
    <mj-attributes>
      <mj-all font-size="12px" />
    </mj-attributes>
    <mj-breakpoint width="12px" />
    <mj-font href="https://jolimail.io" name="Comic" />
    <mj-preview>Hello World with all!</mj-preview>
    <mj-title>Hello World!</mj-title>
  </mj-head>
</mjml>
"#;
        let root = crate::mjml::MJML::parse(origin).unwrap();
        assert_eq!(origin, root.pretty_print());
        let head = root.head().unwrap();
        assert_eq!(head.breakpoint().unwrap().value(), "12px");
        assert_eq!(head.preview().unwrap().content(), "Hello World with all!");
        assert_eq!(head.title().unwrap().content(), "Hello World!");
        assert_eq!(head.children().len(), 5);
    }

    #[test]
    fn with_title() {
        let mut item = crate::mj_head::MJHead::default();
        item.children.push(crate::mj_head::MJHeadChild::MJTitle(
            crate::mj_title::MJTitle::from("Hello World!"),
        ));
        assert_eq!(
            "<mj-head><mj-title>Hello World!</mj-title></mj-head>",
            item.dense_print()
        );
    }
}
