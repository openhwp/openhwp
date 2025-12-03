//! namespace: http://www.hancom.co.kr/hwpml/2011/app
//! filename: settings.xml

use crate::{
    any_element::{AnyElement, ElementName},
    core::IdRef,
    error::Error,
};

#[derive(Debug)]
pub struct Setting {
    pub caret_position: Option<CaretPosition>,
    pub print: Option<Print>,
}

#[derive(Debug)]
pub struct CaretPosition {
    pub list_id_ref: IdRef,
    pub paragraph_id_ref: IdRef,
    pub position: u32,
}

#[derive(Debug)]
pub struct Print {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

impl TryFrom<AnyElement> for Setting {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__APP__HWP_APPLICATION_SETTING)?;

        let (caret_position, print) = children! {element;
            opt HANCOM__APP__CARET_POSITION, CaretPosition;
            opt OPENDOCUMENT__CONFIG__CONFIG_ITEM_SET, Print;
        };

        Ok(Self {
            caret_position,
            print,
        })
    }
}

impl TryFrom<AnyElement> for CaretPosition {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__APP__CARET_POSITION)?;

        let (list_id_ref, paragraph_id_ref, position) = attributes!(element, "CaretPosition";
            "listIDRef" as list_id_ref => one IdRef,
            "paraIDRef" as paragraph_id_ref => one IdRef,
            "pos" as position => one u32,
        );

        Ok(Self {
            list_id_ref,
            paragraph_id_ref,
            position,
        })
    }
}

impl TryFrom<AnyElement> for Print {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::OPENDOCUMENT__CONFIG__CONFIG_ITEM_SET)?;

        if element
            .attributes
            .iter()
            .find(|(key, value)| key == "name" && value == "PrintInfo")
            .is_none()
        {
            missing_attribute!("<config-item-set name=\"PrintInfo\">");
        }

        let mut items = vec![];

        for child in element.children {
            child.expect(ElementName::OPENDOCUMENT__CONFIG__CONFIG_ITEM)?;

            let (name, r#type) = attributes!(child, "config-item";
                "name" as name => one (string),
                "type" as ty => one (string),
            );

            let value = match child.text {
                Some(value) => value,
                None => continue,
            };

            items.push(Item {
                name,
                r#type,
                value,
            });
        }

        Ok(Self { items })
    }
}

impl Print {
    pub fn auto_foot_note(&self) -> Option<bool> {
        self.find("PrintAutoFootNote")
    }

    pub fn auto_head_note(&self) -> Option<bool> {
        self.find("PrintAutoHeadNote")
    }

    pub fn method(&self) -> Option<u32> {
        self.find("PrintMethod")
    }

    pub fn overlap_size(&self) -> Option<u32> {
        self.find("OverlapSize")
    }

    pub fn crop_mark(&self) -> Option<u32> {
        self.find("PrintCropMark")
    }

    pub fn binder_hole_type(&self) -> Option<u32> {
        self.find("BinderHoleType")
    }

    pub fn zoom_x(&self) -> Option<u32> {
        self.find("ZoomX")
    }

    pub fn zoom_y(&self) -> Option<u32> {
        self.find("ZoomY")
    }

    fn find<T: std::str::FromStr>(&self, name: &str) -> Option<T> {
        self.items
            .iter()
            .find(|item| item.name == name)
            .and_then(|item| item.value.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ha:HWPApplicationSetting xmlns:ha="http://www.hancom.co.kr/hwpml/2011/app"
  xmlns:config="urn:oasis:names:tc:opendocument:xmlns:config:1.0">
  <ha:CaretPosition listIDRef="0" paraIDRef="6" pos="18" />
  <config:config-item-set name="PrintInfo">
    <config:config-item name="PrintAutoFootNote" type="boolean">false</config:config-item>
    <config:config-item name="PrintAutoHeadNote" type="boolean">false</config:config-item>
    <config:config-item name="PrintMethod" type="short">0</config:config-item>
    <config:config-item name="OverlapSize" type="short">0</config:config-item>
    <config:config-item name="PrintCropMark" type="short">0</config:config-item>
    <config:config-item name="BinderHoleType" type="short">0</config:config-item>
    <config:config-item name="ZoomX" type="short">100</config:config-item>
    <config:config-item name="ZoomY" type="short">100</config:config-item>
  </config:config-item-set>
</ha:HWPApplicationSetting>
"#;
        let element = AnyElement::from_bytes(XML)?;
        let setting = Setting::try_from(element)?;

        insta::assert_debug_snapshot!(setting);

        if let Some(print) = setting.print {
            assert_eq!(print.auto_foot_note(), Some(false));
            assert_eq!(print.auto_head_note(), Some(false));
            assert_eq!(print.method(), Some(0));
            assert_eq!(print.overlap_size(), Some(0));
            assert_eq!(print.crop_mark(), Some(0));
            assert_eq!(print.binder_hole_type(), Some(0));
            assert_eq!(print.zoom_x(), Some(100));
            assert_eq!(print.zoom_y(), Some(100));
        }

        Ok(())
    }
}
