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

        let mut target_application = None;
        let mut major = None;
        let mut minor = None;
        let mut micro = None;
        let mut build = None;
        let mut os = None;
        let mut xml_version = None;
        let mut application = None;
        let mut app_version = None;

        for (key, value) in element.attributes {
            match key.as_str() {
                "targetApplication" => target_application = Some(value),
                "major" => major = Some(value.parse()?),
                "minor" => minor = Some(value.parse()?),
                "micro" => micro = Some(value.parse()?),
                "buildNumber" => build = Some(value.parse()?),
                "os" => os = Some(value.parse()?),
                "xmlVersion" => xml_version = Some(value),
                "application" => application = Some(value),
                "appVersion" => app_version = Some(value),
                _ => {}
            }
        }

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
