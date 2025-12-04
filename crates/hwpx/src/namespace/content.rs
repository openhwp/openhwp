//! namespace: http://www.idpf.org/2007/opf/
//! filename: Contents/content.hpf

use crate::{
    any_element::{AnyElement, ElementName},
    core::MediaType,
    error::Error,
};

#[derive(Debug)]
pub struct Package {
    pub metadata: Option<Metadata>,
    pub manifest: Option<Manifest>,
    pub spine: Option<Spine>,
}

impl TryFrom<AnyElement> for Package {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__PACKAGE)?;

        let (metadata, manifest, spine) = children!(element;
            opt OPENDOCUMENT__OPF__METADATA, Metadata;
            opt OPENDOCUMENT__OPF__MANIFEST, Manifest;
            opt OPENDOCUMENT__OPF__SPINE, Spine
        );

        Ok(Self {
            metadata,
            manifest,
            spine,
        })
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub title: Option<String>,
    pub language: Option<String>,
    pub meta: Vec<Meta>,
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
                ElementName::OPENDOCUMENT__OPF__META => meta.push(Meta::try_from(child)?),
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

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub content: String,
    pub text: Option<String>,
}

impl TryFrom<AnyElement> for Meta {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__META)?;

        let (name, content) = attributes!(element, "meta";
            "name" as name => one (string),
            "content" as content => one (string),
        );

        Ok(Self {
            name,
            content,
            text: element.text,
        })
    }
}

#[derive(Debug)]
pub struct Manifest {
    pub items: Vec<Item>,
}

impl TryFrom<AnyElement> for Manifest {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__MANIFEST)?;

        let items = children!(element;
            many OPENDOCUMENT__OPF__ITEM, Item
        );

        Ok(Self { items })
    }
}

#[derive(Debug)]
pub struct Spine {
    pub item_refs: Vec<ItemRef>,
}

impl TryFrom<AnyElement> for Spine {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__SPINE)?;

        let item_refs = children!(element;
            many OPENDOCUMENT__OPF__ITEM_REFERENCE, ItemRef
        );

        Ok(Self { item_refs })
    }
}

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub href: String,
    pub media_type: MediaType,
    pub embedded: bool,
}

impl TryFrom<AnyElement> for Item {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__ITEM)?;

        let (id, href, media_type, embedded) = attributes!(element, "item";
            "id" as id => one (string),
            "href" as href => one (string),
            "media-type" as media_type => one MediaType,
            "isEmbeded" as embedded => default false; boolean,
        );

        Ok(Self {
            id,
            href,
            media_type,
            embedded,
        })
    }
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

impl TryFrom<AnyElement> for ItemRef {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__OPF__ITEM_REFERENCE)?;

        let (id_ref, linear) = attributes!(element, "itemref";
            "idref" as id_ref => one (string),
            "linear" as linear => one (boolean),
        );

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
