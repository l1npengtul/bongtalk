use crate::error::BongTalkError;
use ::lending_iterator::prelude::*;
use quick_xml::events::Event;
use quick_xml::name::Prefix;
use quick_xml::Reader;
use rgb::{RGBA, RGBA8};
use rhai::{FuncArgs, ImmutableString, Variant};
use serde::__private::de;
use smartstring::{LazyCompact, SmartString};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::intrinsics::saturating_add;
use std::io::Read;
use std::mem::transmute;
use std::num::ParseFloatError;
use std::ops::Add;
use std::str::from_utf8;

struct TagName<'a> {
    pub name: &'a str,
    pub additional: Option<&'a str>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct CustomTag<'a> {
    pub tag: Cow<'a, str>,
    pub carries_attribute: bool,
}

impl<'a> PartialEq<&str> for CustomTag<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.tag == other
    }
}

impl<'a> Hash for CustomTag<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state)
    }
}

pub fn parse_text(text: &str, custom_tags: &[CustomTag]) -> Result<Text, BongTalkError> {
    let wrapped = format!("<p>{text}</p>");
    let mut parser = Reader::from_str(&wrapped);

    parser.trim_text(true);
    let mut section_style = Style {
        italicized: false,
        bold: false,
        size: None,
        underline: false,
        strikethrough: false,
        color: None,
        bg_color: None,
        alignment: TextAlignment::Center,
        font: None,
        shaky: false,
        deranged: false,
        wavy: false,
        custom_styles: Default::default(),
    };

    struct StyleStacks {
        pub italicized: u16,
        pub bold: u16,
        pub size: Vec<Option<f32>>,
        pub underline: u16,
        pub strikethrough: u16,
        pub color: Vec<Option<RGBA8>>,
        pub bg_color: Vec<Option<RGBA8>>,
        pub alignment: Vec<TextAlignment>,
        pub font: Vec<Option<ImmutableString>>,
        pub shaky: u16,
        pub deranged: u16,
        pub wavy: u16,
        pub style_stacks: BTreeMap<(ImmutableString, Option<ImmutableString>), u16>,
    }

    let mut stacks = StyleStacks {
        italicized: 0,
        bold: 0,
        size: Vec::with_capacity(5),
        underline: 0,
        strikethrough: 0,
        color: Vec::with_capacity(5),
        bg_color: Vec::with_capacity(5),
        alignment: Vec::with_capacity(5),
        font: Vec::with_capacity(5),
        shaky: 0,
        deranged: 0,
        wavy: 0,
        style_stacks: Default::default(),
    };

    let mut text = Text { sections: vec![] };
    let mut txt_set = false;

    loop {
        if let Ok(event) = parser.read_event() {
            match event {
                Event::Start(start) => {
                    let name = start.name();
                    let tag = {
                        match name.prefix() {
                            Some(prefix) => TagName {
                                name: from_utf8(prefix.as_ref())
                                    .map_err(|_| BongTalkError::XmlEncodingError)?,
                                additional: Some(
                                    from_utf8(name.local_name().as_ref())
                                        .map_err(|_| BongTalkError::XmlEncodingError)?,
                                ),
                            },
                            None => TagName {
                                name: from_utf8(name.local_name().as_ref())
                                    .map_err(|_| BongTalkError::XmlEncodingError)?,
                                additional: None,
                            },
                        }
                    };

                    if txt_set {
                        if let Some(sect) = text.sections.last_mut() {
                            sect.set_style(section_style.clone());
                            txt_set = false;
                        }
                    }

                    match tag.name {
                        "p" => continue,
                        "i" => {
                            section_style.italicized = true;
                            stacks.italicized += 1;
                        }
                        "b" => {
                            section_style.bold = true;
                            stacks.bold += 1;
                        }
                        "del" => {
                            section_style.strikethrough = true;
                            stacks.strikethrough += 1;
                        }
                        "u" => {
                            section_style.underline = true;
                            stacks.underline += 1;
                        }
                        "color" => {
                            let c = tag
                                .additional
                                .map(|ctag| RGBA8::from(csscolorparser::parse(ctag)?.to_rgba8()));
                            section_style.color = c;
                            stacks.color.push(c);
                        }
                        "bg" => {
                            let c = tag
                                .additional
                                .map(|ctag| RGBA8::from(csscolorparser::parse(ctag)?.to_rgba8()));
                            section_style.bg_color = c;
                            stacks.bg_color.push(c);
                        }
                        "size" => match tag.additional {
                            Some(additional) => match additional.parse::<f32>() {
                                Ok(size) => {
                                    section_style.size = Some(size);
                                    stacks.size.push(Some(size));
                                }
                                Err(why) => return Err(BongTalkError::XmlError(why.to_string())),
                            },
                            None => {
                                section_style.size = None;
                                stacks.size.push(None);
                            }
                        },
                        "align" => {
                            let alignment = match tag.additional.unwrap_or_default() {
                                "left" => TextAlignment::Left,
                                "center" => TextAlignment::Center,
                                "right" => TextAlignment::Right,
                                _ => {
                                    return Err(BongTalkError::XmlError(
                                        "Align not recognized".to_string(),
                                    ))
                                }
                            };
                            section_style.alignment = alignment;
                            stacks.alignment.push(alignment);
                        }
                        "font" => {
                            let f = tag.additional.map(|x| ImmutableString::from(x));
                            section_style.font = f.clone();
                            stacks.font.push(f);
                        }
                        "shaky" => {
                            section_style.shaky = true;
                            stacks.shaky += 1;
                        }
                        "deranged" => {
                            section_style.deranged = true;
                            stacks.deranged += 1;
                        }
                        "wavy" => {
                            section_style.wavy = true;
                            stacks.wavy += 1;
                        }
                        custom => {
                            for ct in custom_tags {
                                if ct == custom {
                                    if ct.carries_attribute {
                                        match tag.additional {
                                            Some(attr) => {
                                                let t = ImmutableString::from(custom);
                                                let at = Some(ImmutableString::from(attr));
                                                section_style
                                                    .custom_styles
                                                    .insert(t.clone(), at.clone());
                                                match stacks
                                                    .style_stacks
                                                    .get_mut(&(t.clone(), at.clone()))
                                                {
                                                    Some(c) => *c += 1,
                                                    None => stacks.style_stacks.insert((t, at), 1),
                                                }
                                            }
                                            None => {
                                                return Err(BongTalkError::XmlError(format!(
                                                "Custom tag {ct:?} has attribute not satisfied."
                                            )))
                                            }
                                        }
                                    } else {
                                        let t = ImmutableString::from(custom);
                                        section_style.custom_styles.insert(t.clone(), None);
                                        match stacks.style_stacks.get_mut(&(t.clone(), None)) {
                                            Some(c) => *c += 1,
                                            None => stacks.style_stacks.insert((t, None), 1),
                                        }
                                    }
                                }
                            }
                            return Err(BongTalkError::XmlError(format!("Bad Tag: {custom}")));
                        }
                    }
                }
                Event::End(end) => {
                    let name = end.name();
                    let tag = {
                        match name.prefix() {
                            Some(prefix) => TagName {
                                name: from_utf8(prefix.as_ref())
                                    .map_err(|_| BongTalkError::XmlEncodingError)?,
                                additional: Some(
                                    from_utf8(name.local_name().as_ref())
                                        .map_err(|_| BongTalkError::XmlEncodingError)?,
                                ),
                            },
                            None => TagName {
                                name: from_utf8(name.local_name().as_ref())
                                    .map_err(|_| BongTalkError::XmlEncodingError)?,
                                additional: None,
                            },
                        }
                    };

                    if txt_set {
                        if let Some(sect) = text.sections.last_mut() {
                            sect.set_style(section_style.clone());
                            txt_set = false;
                        }
                    }

                    match tag.name {
                        "p" => continue,
                        "i" => {
                            if stacks.italicized == 1 {
                                stacks.italicized = 0;
                                section_style.italicized = false;
                            } else {
                                stacks.italicized -= 1;
                            }
                        }
                        "b" => {
                            if stacks.bold == 1 {
                                stacks.bold = 0;
                                section_style.bold = false;
                            } else {
                                stacks.bold -= 1;
                            }
                        }
                        "del" => {
                            if stacks.strikethrough == 1 {
                                stacks.strikethrough = 0;
                                section_style.strikethrough = false;
                            } else {
                                stacks.strikethrough -= 1;
                            }
                        }
                        "u" => {
                            if stacks.underline == 1 {
                                stacks.underline = 0;
                                section_style.underline = false;
                            } else {
                                stacks.underline -= 1;
                            }
                        }
                        "color" => section_style.color = stacks.color.pop().flatten(),
                        "bg" => section_style.bg_color = stacks.bg_color.pop().flatten(),
                        "size" => section_style.size = stacks.size.pop().flatten(),
                        "align" => {
                            section_style.alignment = stacks.alignment.pop().unwrap_or_default()
                        }
                        "font" => section_style.font = stacks.font.pop().flatten(),
                        "shaky" => {
                            if stacks.shaky == 1 {
                                stacks.shaky = 0;
                                section_style.shaky = false;
                            } else {
                                stacks.shaky -= 1;
                            }
                        }
                        "deranged" => {
                            if stacks.deranged == 1 {
                                stacks.deranged = 0;
                                section_style.deranged = false;
                            } else {
                                stacks.deranged -= 1;
                            }
                        }
                        "wavy" => {
                            if stacks.wavy == 1 {
                                stacks.wavy = 0;
                                section_style.wavy = false;
                            } else {
                                stacks.wavy -= 1;
                            }
                        }
                        custom => {
                            for ct in custom_tags {
                                if ct == custom {
                                    if ct.carries_attribute {
                                        match tag.additional {
                                            Some(attr) => {
                                                let t = ImmutableString::from(custom);
                                                let at = Some(ImmutableString::from(attr));

                                                if let Some(st) = stacks
                                                    .style_stacks
                                                    .get_mut(&(t.clone(), at.clone()))
                                                {
                                                    if st == 1 {
                                                        *st = 0;
                                                        section_style
                                                            .custom_styles
                                                            .remove(&(t, at));
                                                    } else {
                                                        *st -= 1;
                                                    }
                                                }
                                            }
                                            None => {
                                                return Err(BongTalkError::XmlError(format!(
                                                    "Custom tag {ct:?} has attribute not satisfied."
                                                )))
                                            }
                                        }
                                    } else {
                                        let t = ImmutableString::from(custom);

                                        if let Some(st) =
                                            stacks.style_stacks.get_mut(&(t.clone(), None))
                                        {
                                            if st == 1 {
                                                *st = 0;
                                                section_style.custom_styles.remove(&(t, None));
                                            } else {
                                                *st -= 1;
                                            }
                                        }
                                    }
                                }
                            }
                            return Err(BongTalkError::XmlError(format!("Bad Tag: {custom}")));
                        }
                    }
                }
                Event::Text(t) => {
                    txt_set = true;
                    text.sections.push(Section {
                        text: t.unescape()?.into_owned(),
                        style: Default::default(),
                        id: Default::default(),
                    });
                }
                Event::Eof => break,
                _ => continue,
            }
        }
    }

    let mut shaky = 0;
    let mut deranged = 0;
    let mut wavy = 0;

    let mut sects = text.sections.windows_mut::<2>();
    while let Some(&mut [this, ref mut next]) = sects.next() {

        if this.style.wavy && this.style.wavy == next.style.wavy {
            match this.id.wavy {
                Some(id) => {
                    next.id.wavy = Some(id);
                }
                None => {
                    this.id.wavy = Some(wavy);
                    next.id.wavy = Some(wavy);
                }
            }
        } else {
            wavy += 1;
        }

        if this.style.deranged && this.style.deranged == next.style.deranged {
            match this.id.deranged {
                Some(id) => {
                    next.id.deranged = Some(id);
                }
                None => {
                    this.id.deranged = Some(deranged);
                    next.id.deranged = Some(deranged);
                }
            }
        } else {
            deranged += 1;
        }
        
        if this.style.shaky && this.style.shaky == next.style.shaky {
            match this.id.shaky {
                Some(id) => {
                    next.id.shaky = Some(id);
                }
                None => {
                    this.id.shaky = Some(shaky);
                    next.id.shaky = Some(shaky);
                }
            }
        } else {
            shaky += 1;
        }
    }

    Ok(text)
}

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Text {
    sections: Vec<Section>,
}

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Section {
    text: String,
    style: Style,
    id: SectionStyleId,
}

impl Section {
    pub fn new(text: String) -> Self {
        Section {
            text,
            style: Default::default(),
            id: Default::default(),
        }
    }

    pub fn new_with_style(text: String, style: Style) -> Self {
        Section {
            text,
            style,
            id: Default::default(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn mut_style(&mut self) -> &mut Style {
        &mut self.style
    }
}

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Style {
    pub italicized: bool,
    pub bold: bool,
    pub size: Option<f32>,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Option<RGBA8>,
    pub bg_color: Option<RGBA8>,
    pub alignment: TextAlignment,
    pub font: Option<ImmutableString>,
    pub shaky: bool,
    pub deranged: bool,
    pub wavy: bool,
    pub custom_styles: BTreeMap<ImmutableString, Option<ImmutableString>>,
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SectionStyleId {
    pub shaky: Option<u64>,
    pub deranged: Option<u64>,
    pub wavy: Option<u64>,
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum TextAlignment {
    Center,
    #[default]
    Left,
    Right,
}
