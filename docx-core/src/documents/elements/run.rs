use super::{Break, DeleteText, Drawing, RunProperty, Tab, Text};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::BreakType;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub run_property: RunProperty,
    pub children: Vec<RunChild>,
}

impl Default for Run {
    fn default() -> Self {
        let run_property = RunProperty::new();
        Self {
            run_property,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunChild {
    Text(Text),
    DeleteText(DeleteText),
    Tab(Tab),
    Break(Break),
    Drawing(Drawing),
}

impl Serialize for RunChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RunChild::Text(ref s) => {
                let mut t = serializer.serialize_struct("Text", 2)?;
                t.serialize_field("type", "text")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::DeleteText(ref s) => {
                let mut t = serializer.serialize_struct("DeleteText", 2)?;
                t.serialize_field("type", "deleteText")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Tab(_) => {
                let mut t = serializer.serialize_struct("Tab", 1)?;
                t.serialize_field("type", "tab")?;
                t.end()
            }
            RunChild::Break(ref s) => {
                let mut t = serializer.serialize_struct("Break", 2)?;
                t.serialize_field("type", "break")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Drawing(ref s) => {
                let mut t = serializer.serialize_struct("Drawing", 2)?;
                t.serialize_field("type", "drawing")?;
                t.serialize_field("data", s)?;
                t.end()
            }
        }
    }
}

impl Run {
    pub fn new() -> Run {
        Run {
            ..Default::default()
        }
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Run {
        self.children.push(RunChild::Text(Text::new(text)));
        self
    }

    pub fn add_delete_text(mut self, text: impl Into<String>) -> Run {
        self.children
            .push(RunChild::DeleteText(DeleteText::new(text)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.children.push(RunChild::Tab(Tab::new()));
        self
    }

    pub fn add_drawing(mut self, d: Drawing) -> Run {
        self.children.push(RunChild::Drawing(d));
        self
    }

    pub fn add_break(mut self, break_type: BreakType) -> Run {
        self.children.push(RunChild::Break(Break::new(break_type)));
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Run {
        self.run_property = self.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Run {
        self.run_property = self.run_property.italic();
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> Run {
        self.run_property = self.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Run {
        self.run_property = self.run_property.vanish();
        self
    }
}

impl BuildXML for Run {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_run().add_child(&self.run_property);
        for c in &self.children {
            match c {
                RunChild::Text(t) => b = b.add_child(t),
                RunChild::DeleteText(t) => b = b.add_child(t),
                RunChild::Tab(t) => b = b.add_child(t),
                RunChild::Break(t) => b = b.add_child(t),
                RunChild::Drawing(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let b = Run::new().add_text("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_underline() {
        let b = Run::new().add_text("Hello").underline("single").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr><w:u w:val="single" /></w:rPr><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_child_json() {
        let c = RunChild::Text(Text::new("Hello"));
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}"#
        );
    }

    #[test]
    fn test_run_json() {
        let run = Run {
            children: vec![
                RunChild::Tab(Tab::new()),
                RunChild::Text(Text::new("Hello")),
                RunChild::Break(Break::new(BreakType::Page)),
                RunChild::DeleteText(DeleteText::new("deleted")),
            ],
            run_property: RunProperty {
                sz: Some(Sz::new(30)),
                sz_cs: Some(SzCs::new(30)),
                color: Some(Color::new("C9211E")),
                highlight: Some(Highlight::new("yellow")),
                underline: Some(Underline::new("single")),
                bold: Some(Bold::new()),
                bold_cs: Some(BoldCs::new()),
                italic: Some(Italic::new()),
                italic_cs: Some(ItalicCs::new()),
                vanish: Some(Vanish::new()),
            },
        };
        assert_eq!(
            serde_json::to_string(&run).unwrap(),
            r#"{"runProperty":{"sz":30,"szCs":30,"color":"C9211E","highlight":"yellow","underline":"single","bold":true,"boldCs":true,"italic":true,"italicCs":true,"vanish":true},"children":[{"type":"tab"},{"type":"text","data":{"preserveSpace":true,"text":"Hello"}},{"type":"break","data":{"breakType":"page"}},{"type":"deleteText","data":{"text":"deleted","preserveSpace":true}}]}"#
        );
    }
}
