//! namespace: http://www.hancom.co.kr/hwpml/2011/version
//! filename: version.xml

use crate::{
    any_element::{AnyElement, ElementName},
    error::Error,
};

#[derive(Debug)]
pub struct Version {
    pub target_application: Option<String>,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub micro: Option<u32>,
    pub build: Option<u32>,
    pub os: Option<u32>,
    pub xml_version: Option<String>,
    pub application: Option<String>,
    pub app_version: Option<String>,
}

impl TryFrom<AnyElement> for Version {
    type Error = Error;

    fn try_from(element: AnyElement) -> Result<Self, Self::Error> {
        element.expect(ElementName::HANCOM__VERSION__HCF_VERSION)?;

        let (
            target_application,
            major,
            minor,
            micro,
            build,
            os,
            xml_version,
            application,
            app_version,
        ) = attributes!(element, "HCFVersion";
            "targetApplication" as target_application => opt (string),
            "major" as major => opt u32,
            "minor" as minor => opt u32,
            "micro" as micro => opt u32,
            "buildNumber" as build => opt u32,
            "os" as os => opt u32,
            "xmlVersion" as xml_version => opt (string),
            "application" as application => opt (string),
            "appVersion" as app_version => opt (string),
        );

        Ok(Self {
            target_application,
            major,
            minor,
            micro,
            build,
            os,
            xml_version,
            application,
            app_version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() -> Result<(), Error> {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<hv:HCFVersion xmlns:hv="http://www.hancom.co.kr/hwpml/2011/version"
  tagetApplication="WORDPROCESSOR" major="5" minor="1" micro="0" buildNumber="1" os="1"
  xmlVersion="1.2" application="Hancom Office Hangul" appVersion="10, 0, 0, 9139 WIN32LEWindows_8" />"#;
        let element = AnyElement::from_bytes(XML)?;
        let version = Version::try_from(element)?;

        insta::assert_debug_snapshot!(version);

        Ok(())
    }
}
