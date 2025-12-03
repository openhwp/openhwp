//! namespace: http://www.idpf.org/2007/opf/
//! filename: Contents/content.hpf

use crate::{
    any_element::{AnyElement, ElementName},
    core::MediaType,
    error::Error,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Package {
    pub metadata: Option<Metadata>,
    pub manifest: Option<Manifest>,
    pub spine: Option<Spine>,
}

#[derive(Debug)]
pub struct Metadata {
    pub title: Option<String>,
    pub language: Option<String>,
    pub meta: Vec<Meta>,
}

#[derive(Debug)]
pub struct Manifest {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Spine {
    pub item_refs: Vec<ItemRef>,
}

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub content: String,
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub href: String,
    pub media_type: MediaType,
    pub embedded: bool,
}

#[derive(Debug)]
pub struct Setting {
    pub id: String,
    pub href: String,
    pub media_type: MediaType,
}

#[derive(Debug)]
pub struct ItemRef {
    pub id_ref: String,
    pub linear: bool,
}

impl TryFrom<AnyElement> for Package {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__PACKAGE)?;

        let mut metadata = None;
        let mut manifest = None;
        let mut spine = None;

        for child in element.children {
            match child.name {
                ElementName::OPENDOCUMENT__OPF__METADATA => metadata = Some(child.try_into()?),
                ElementName::OPENDOCUMENT__OPF__MANIFEST => manifest = Some(child.try_into()?),
                ElementName::OPENDOCUMENT__OPF__SPINE => spine = Some(child.try_into()?),
                _ => unknown!("Unknown package element: {:?}", child.name),
            }
        }

        Ok(Self {
            metadata,
            manifest,
            spine,
        })
    }
}

impl TryFrom<AnyElement> for Metadata {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__METADATA)?;

        let mut title = None;
        let mut language = None;
        let mut meta = vec![];

        for child in element.children {
            match child.name {
                ElementName::OPENDOCUMENT__OPF__TITLE => title = child.text,
                ElementName::OPENDOCUMENT__OPF__LANGUAGE => language = child.text,
                ElementName::OPENDOCUMENT__OPF__META => {
                    let Some((_, name)) = child.attributes.iter().find(|(key, _)| key == "name")
                    else {
                        continue;
                    };
                    let Some((_, content)) =
                        child.attributes.iter().find(|(key, _)| key == "content")
                    else {
                        continue;
                    };
                    if let Some("") = child.text.as_deref() {
                        continue;
                    }
                    meta.push(Meta {
                        name: name.to_owned(),
                        content: content.to_owned(),
                        text: child.text,
                    });
                }
                _ => unknown!("Unknown metadata element: {:?}", child.name),
            }
        }

        Ok(Self {
            title,
            language,
            meta,
        })
    }
}

impl TryFrom<AnyElement> for Manifest {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__MANIFEST)?;

        let mut items = vec![];

        for child in element.children {
            items.push(child.try_into()?);
        }

        Ok(Self { items })
    }
}

impl TryFrom<AnyElement> for Spine {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__SPINE)?;

        let mut item_refs = Vec::new();

        for child in element.children {
            item_refs.push(child.try_into()?);
        }

        Ok(Self { item_refs })
    }
}

impl TryFrom<AnyElement> for Item {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__ITEM)?;

        let mut id = None;
        let mut href = None;
        let mut media_type = None;
        let mut embedded = false;

        for (key, value) in element.attributes {
            match key.as_str() {
                "id" => id = Some(value),
                "href" => href = Some(value),
                "media-type" => media_type = Some(MediaType::from_str(&value)?),
                "isEmbeded" => match value.as_str() {
                    "1" => embedded = true,
                    "0" => embedded = false,
                    _ => {}
                },
                _ => {}
            }
        }

        let (id, href, media_type) = match (id, href, media_type) {
            (Some(id), Some(href), Some(media_type)) => (id, href, media_type),
            (None, _, _) => missing_attribute!("id"),
            (_, None, _) => missing_attribute!("href"),
            (_, _, None) => missing_attribute!("media-type"),
        };

        Ok(Self {
            id,
            href,
            media_type,
            embedded,
        })
    }
}

impl TryFrom<AnyElement> for ItemRef {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__ITEM_REFERENCE)?;

        let mut id_ref = None;
        let mut linear = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "idref" => id_ref = Some(value),
                "linear" => match value.as_str() {
                    "yes" => linear = Some(true),
                    "no" => linear = Some(false),
                    _ => {}
                },
                _ => {}
            }
        }

        let (id_ref, linear) = match (id_ref, linear) {
            (Some(id_ref), Some(linear)) => (id_ref, linear),
            (None, _) => missing_attribute!("idref"),
            (_, None) => missing_attribute!("linear"),
        };

        Ok(Self { id_ref, linear })
    }
}

impl Package {
    pub fn title(&self) -> Option<&str> {
        self.metadata.as_ref().and_then(|metadata| metadata.title())
    }

    pub fn language(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.language())
    }

    pub fn creator(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.creator())
    }

    pub fn subject(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.subject())
    }

    pub fn description(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.description())
    }

    pub fn created_date(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.created_date())
    }

    pub fn modified_date(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.modified_date())
    }

    pub fn date(&self) -> Option<&str> {
        self.metadata.as_ref().and_then(|metadata| metadata.date())
    }

    pub fn keyword(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.keyword())
    }

    pub fn header(&self) -> Option<&Item> {
        self.manifest
            .as_ref()
            .and_then(|manifest| manifest.header())
    }

    pub fn binaries(&self) -> impl Iterator<Item = &Item> {
        self.manifest
            .as_ref()
            .map(|manifest| manifest.binaries())
            .into_iter()
            .flatten()
    }

    pub fn images(&self) -> impl Iterator<Item = &Item> {
        self.manifest
            .as_ref()
            .map(|manifest| manifest.images())
            .into_iter()
            .flatten()
    }

    pub fn sections(&self) -> impl Iterator<Item = &Item> {
        self.manifest
            .as_ref()
            .map(|manifest| manifest.sections())
            .into_iter()
            .flatten()
    }

    pub fn setting(&self) -> Option<&Item> {
        self.manifest
            .as_ref()
            .and_then(|manifest| manifest.setting())
    }
}

impl Metadata {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref().or_else(|| self.find("title"))
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref().or_else(|| self.find("language"))
    }

    pub fn creator(&self) -> Option<&str> {
        self.find("creator")
    }

    pub fn subject(&self) -> Option<&str> {
        self.find("subject")
    }

    pub fn description(&self) -> Option<&str> {
        self.find("description")
    }

    pub fn created_date(&self) -> Option<&str> {
        self.find("CreatedDate")
    }

    pub fn modified_date(&self) -> Option<&str> {
        self.find("ModifiedDate")
    }

    pub fn date(&self) -> Option<&str> {
        self.find("date")
    }

    pub fn keyword(&self) -> Option<&str> {
        self.find("keyword")
    }

    #[inline]
    fn find(&self, name: &str) -> Option<&str> {
        self.meta
            .iter()
            .find(|meta| meta.name == name)
            .and_then(|meta| meta.text.as_deref())
    }
}

impl Manifest {
    pub fn header(&self) -> Option<&Item> {
        self.find_by_id("header")
    }

    pub fn binaries(&self) -> impl Iterator<Item = &Item> {
        self.items
            .iter()
            .filter(|item| item.href.starts_with("BinData/"))
    }

    pub fn images(&self) -> impl Iterator<Item = &Item> {
        self.items
            .iter()
            .filter(|item| item.href.starts_with("BinData/image"))
    }

    pub fn sections(&self) -> impl Iterator<Item = &Item> {
        self.items
            .iter()
            .filter(|item| item.href.starts_with("Contents/section"))
    }

    pub fn setting(&self) -> Option<&Item> {
        self.find_by_id("settings")
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn find_by_href(&self, href: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.href == href)
    }

    pub fn find_all_by_media_type(&self, media_type: MediaType) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.media_type == media_type)
            .collect()
    }
}

impl Spine {
    pub fn header(&self) -> Option<&ItemRef> {
        self.find("header")
    }

    pub fn sections(&self) -> impl Iterator<Item = &ItemRef> {
        self.item_refs
            .iter()
            .filter(|item_ref| item_ref.id_ref.starts_with("section"))
    }

    fn find(&self, id_ref: &str) -> Option<&ItemRef> {
        self.item_refs
            .iter()
            .find(|item_ref| item_ref.id_ref == id_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content() -> Result<(), Error> {
        const XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<opf:package xmlns:ha="http://www.hancom.co.kr/hwpml/2011/app"
  xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph"
  xmlns:hp10="http://www.hancom.co.kr/hwpml/2016/paragraph"
  xmlns:hs="http://www.hancom.co.kr/hwpml/2011/section"
  xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core"
  xmlns:hh="http://www.hancom.co.kr/hwpml/2011/head"
  xmlns:hhs="http://www.hancom.co.kr/hwpml/2011/history"
  xmlns:hm="http://www.hancom.co.kr/hwpml/2011/master-page"
  xmlns:hpf="http://www.hancom.co.kr/schema/2011/hpf" xmlns:dc="http://purl.org/dc/elements/1.1/"
  xmlns:opf="http://www.idpf.org/2007/opf/"
  xmlns:ooxmlchart="http://www.hancom.co.kr/hwpml/2016/ooxmlchart"
  xmlns:epub="http://www.idpf.org/2007/ops"
  xmlns:config="urn:oasis:names:tc:opendocument:xmlns:config:1.0" version="" unique-identifier=""
  id="">
  <opf:metadata>
    <opf:title>1</opf:title>
    <opf:language>ko</opf:language>
    <opf:meta name="creator" content="text">user</opf:meta>
    <opf:meta name="subject" content="text" />
    <opf:meta name="description" content="text" />
    <opf:meta name="CreatedDate" content="text">2025-11-19T02:32:55Z</opf:meta>
    <opf:meta name="ModifiedDate" content="text">2025-11-23T07:46:18Z</opf:meta>
    <opf:meta name="date" content="text">2025년 11월 19일 수요일 오전 11:32:55</opf:meta>
    <opf:meta name="keyword" content="text" />
  </opf:metadata>
  <opf:manifest>
    <opf:item id="header" href="Contents/header.xml" media-type="application/xml" />
    <opf:item id="image1" href="BinData/image1.jpg" media-type="image/jpg" isEmbeded="1" />
    <opf:item id="image2" href="BinData/image2.png" media-type="image/png" isEmbeded="1" />
    <opf:item id="image3" href="BinData/image3.jpg" media-type="image/jpg" isEmbeded="1" />
    <opf:item id="image4" href="BinData/image4.jpg" media-type="image/jpg" isEmbeded="1" />
    <opf:item id="section0" href="Contents/section0.xml" media-type="application/xml" />
    <opf:item id="settings" href="settings.xml" media-type="application/xml" />
  </opf:manifest>
  <opf:spine>
    <opf:itemref idref="header" linear="no" />
    <opf:itemref idref="section0" linear="yes" />
  </opf:spine>
</opf:package>"#;
        let root = AnyElement::from_bytes(XML.as_bytes())?;
        let package = Package::try_from(root)?;

        insta::assert_debug_snapshot!(package);

        assert_eq!(package.title(), Some("1"));
        assert_eq!(package.language(), Some("ko"));
        assert_eq!(package.creator(), Some("user"));
        assert_eq!(package.subject(), None);
        assert_eq!(package.description(), None);
        assert_eq!(package.created_date(), Some("2025-11-19T02:32:55Z"));
        assert_eq!(package.modified_date(), Some("2025-11-23T07:46:18Z"));
        assert_eq!(
            package.date(),
            Some("2025년 11월 19일 수요일 오전 11:32:55")
        );
        assert_eq!(package.keyword(), None);

        assert_eq!(
            package.header().map(|item| item.id.as_str()),
            Some("header")
        );
        let binaries: Vec<_> = package.binaries().collect();
        assert_eq!(binaries.len(), 4);
        let images: Vec<_> = package.images().collect();
        assert_eq!(images.len(), 4);
        let sections: Vec<_> = package.sections().collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(
            package.setting().map(|item| item.id.as_str()),
            Some("settings")
        );

        Ok(())
    }
}
