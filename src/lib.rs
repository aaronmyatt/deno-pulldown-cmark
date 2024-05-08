mod utils;

use wasm_bindgen::prelude::*;
use pulldown_cmark::{self, Alignment};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExtractEvent {
    r#type: String,
    tag: String,
    content: String,
    level: u32,
    kind: String,
    fenced: bool,
    language: String,
    start_number: u64,
    label: String,
    alignments: Vec<String>,
    url: String,
    title: String,
    checked: bool
}

fn extract_tag(event: &mut ExtractEvent, tag: pulldown_cmark::Tag) {
    use pulldown_cmark::Tag::*;
    match tag {
        BlockQuote => {
            event.tag = "BLOCK_QUOTE".to_string();
        },
        CodeBlock(kind) => {
            event.tag = "CODE_BLOCK".to_string();
            use pulldown_cmark::CodeBlockKind::*;
            match kind {
                Indented => event.kind = "INDENTED".to_string(),
                Fenced(language) => {
                    event.kind = "FENCED".to_string();
                    event.language = language.into_string();
                }
            };
        },
        Emphasis => event.tag = "EMPHASIS".to_string(),
        FootnoteDefinition(label) => {
            event.tag = "FOOTNOTE_DEFINITION".to_string();
            event.label = label.into_string();
        },
        Heading(level) => {
            event.tag = "HEADING".to_string();
            event.level = level;
        },
        Image(kind, url, title) => {
            event.tag = "IMAGE".to_string();
            event.url = url.into_string();
            event.title = title.into_string();

            use pulldown_cmark::LinkType::*;
            match kind {
                Inline => event.kind = "INLINE".to_string(),
                Reference => event.kind = "REFERENCE".to_string(),
                Autolink => event.kind = "AUTOLINK".to_string(),
                Email => event.kind = "EMAIL".to_string(),
                Shortcut => event.kind = "SHORTCUT".to_string(),
                Collapsed => event.kind = "COLLAPSED".to_string(),
                CollapsedUnknown | ShortcutUnknown | ReferenceUnknown => event.kind = "UNKNOWN".to_string(),
            };
        },
        Item => event.tag = "ITEM".to_string(),
        List(start_number) => {
            event.tag = "LIST".to_string();
            match start_number {
                Some(num) => event.start_number = num,
                None => event.start_number = 0
            }
        },
        Link(kind, url, title) => {
            event.tag = "LINK".to_string();
            event.url = url.into_string();
            event.title = title.into_string();

            use pulldown_cmark::LinkType::*;
            match kind {
                Inline => event.kind = "INLINE".to_string(),
                Reference => event.kind = "REFERENCE".to_string(),
                Autolink => event.kind = "AUTOLINK".to_string(),
                Email => event.kind = "EMAIL".to_string(),
                Shortcut => event.kind = "SHORTCUT".to_string(),
                Collapsed => event.kind = "COLLAPSED".to_string(),
                CollapsedUnknown | ShortcutUnknown | ReferenceUnknown => event.kind = "UNKNOWN".to_string(),
            };
        },
        Paragraph => event.tag = "PARAGRAPH".to_string(),
        Strikethrough => event.tag = "STRIKETHROUGH".to_string(),
        Strong => event.tag = "STRONG".to_string(),
        Table(alignments) => {
            event.tag = "TABLE".to_string();
            event.alignments = alignments.iter().map(|align| {
                match align {
                    Alignment::None => "None".to_string(),
                    Alignment::Left => "Left".to_string(),
                    Alignment::Right => "Right".to_string(),
                    Alignment::Center => "Center".to_string(),
                }
            }).collect();
        },
        TableHead => event.tag = "TABLE_HEAD".to_string(),
        TableRow => event.tag = "TABLE_ROW".to_string(),
        TableCell => event.tag = "TABLE_CELL".to_string(),
        



    }
}

#[wasm_bindgen]
pub fn parse(contents: &str) -> JsValue {
    // parse markdown
    let parser = pulldown_cmark::Parser::new(&contents);

    let mut events: Vec<ExtractEvent> = Vec::new();

    for event in parser {
        let mut js_event = ExtractEvent {
            r#type: "".to_string(),
            tag: "".to_string(),
            content: "".to_string(),
            level: 0,
            kind: "".to_string(),
            fenced: false,
            language: "".to_string(),
            start_number: 0,
            label: "".to_string(),
            alignments: Vec::new(),
            url: "".to_string(),
            title: "".to_string(),
            checked: false
        };

        use pulldown_cmark::Event::*;
        match event {
            Start(tag) => {
                js_event.r#type = String::from("START");
                extract_tag(&mut js_event, tag);
            },
            End(tag) => {
                js_event.r#type = String::from("END");
                extract_tag(&mut js_event, tag);
            },
            Text(text) => {
                js_event.r#type = String::from("TEXT");
                js_event.content = text.into_string();
            },
            Code(text) => {
                js_event.r#type = String::from("CODE");
                js_event.content = text.into_string();
            },
            Html(text) => {
                js_event.r#type = String::from("HTML");
                js_event.content = text.into_string();
            },
            FootnoteReference(text) => {
                js_event.r#type = String::from("FOOTNOTE_REFERENCE");
                js_event.content = text.into_string();
            },
            SoftBreak => {
                js_event.r#type = String::from("SOFT_BREAK");
            },
            HardBreak => {
                js_event.r#type = String::from("HARD_BREAK");
            },
            Rule => {
                js_event.r#type = String::from("RULE");
            },
            TaskListMarker(checked) => {
                js_event.r#type = String::from("TASK_LIST_MARKER");
                js_event.checked = checked;
            },
        }
        events.push(js_event);
    }
    
    serde_wasm_bindgen::to_value(&events).unwrap()

    
}

#[wasm_bindgen]
pub fn html(contents: &str) -> String{
    let parser = pulldown_cmark::Parser::new(&contents);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
