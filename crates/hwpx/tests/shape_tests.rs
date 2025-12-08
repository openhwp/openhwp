//! 그림 및 그리기 객체 파싱 테스트

use quick_xml::de::from_str;
use hwpx::paragraph::{Picture, Rectangle};

mod picture_tests {
    use super::*;

    #[test]
    fn parse_picture_attributes() {
        let xml = r#"<hp:pic xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1475691556" zOrder="3" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="401949733" reverse="0">
            <hp:offset x="4294937695" y="4294967295"/>
            <hp:orgSz width="59880" height="11400"/>
            <hp:curSz width="9749" height="2567"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="4874" centerY="1283" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="-29601" e4="0" e5="1" e6="-1"/>
                <hc:scaMatrix e1="0.162809" e2="0" e3="29601" e4="0" e5="0.225175" e6="1"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
        </hp:pic>"#;

        let result: Result<Picture, _> = from_str(xml);

        assert!(result.is_ok(), "그림 파싱 실패: {:?}", result.err());

        let picture = result.unwrap();
        assert_eq!(picture.id, Some(1475691556));
        assert_eq!(picture.z_order, 3);
        assert_eq!(picture.group_level, 0);
    }

    #[test]
    fn parse_full_picture_from_fixture() {
        let xml = r#"<hp:pic xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1475691556" zOrder="3" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="401949733" reverse="0">
            <hp:offset x="4294937695" y="4294967295"/>
            <hp:orgSz width="59880" height="11400"/>
            <hp:curSz width="9749" height="2567"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="4874" centerY="1283" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="-29601" e4="0" e5="1" e6="-1"/>
                <hc:scaMatrix e1="0.162809" e2="0" e3="29601" e4="0" e5="0.225175" e6="1"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
            <hp:imgRect>
                <hc:pt0 x="0" y="0"/>
                <hc:pt1 x="59880" y="0"/>
                <hc:pt2 x="59880" y="11400"/>
                <hc:pt3 x="0" y="11400"/>
            </hp:imgRect>
            <hp:imgClip left="524" right="15726" top="0" bottom="3960"/>
            <hp:inMargin left="0" right="0" top="0" bottom="0"/>
            <hp:imgDim dimwidth="16260" dimheight="3960"/>
            <hp:effects/>
            <hp:sz width="9749" widthRelTo="ABSOLUTE" height="2567" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="0" right="0" top="0" bottom="0"/>
            <hp:shapeComment>그림입니다.</hp:shapeComment>
        </hp:pic>"#;

        let result: Result<Picture, _> = from_str(xml);

        assert!(
            result.is_ok(),
            "전체 그림 파싱 실패: {:?}",
            result.err()
        );

        let picture = result.unwrap();
        assert!(picture.image_rectangle.is_some());
        assert!(picture.image_clip.is_some());
        assert!(picture.shape_comment.is_some());
        
        // 이미지 크기 확인 (Option 타입)
        assert_eq!(picture.original_size.width, Some(59880));
        assert_eq!(picture.original_size.height, Some(11400));
        assert_eq!(picture.current_size.width, Some(9749));
        assert_eq!(picture.current_size.height, Some(2567));
    }

    #[test]
    fn parse_picture_with_flip() {
        let xml = r#"<hp:pic xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1" zOrder="0">
            <hp:offset x="0" y="0"/>
            <hp:orgSz width="100" height="100"/>
            <hp:curSz width="100" height="100"/>
            <hp:flip horizontal="1" vertical="1"/>
            <hp:rotationInfo angle="0" centerX="50" centerY="50" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
                <hc:scaMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hc:img binaryItemIDRef="test" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
        </hp:pic>"#;

        let result: Result<Picture, _> = from_str(xml);
        assert!(result.is_ok());

        let picture = result.unwrap();
        assert!(picture.flip.horizontal);
        assert!(picture.flip.vertical);
    }
}

mod rectangle_tests {
    use super::*;

    #[test]
    fn parse_rectangle() {
        // r##"..."## 사용하여 # 문자를 포함한 문자열 처리
        let xml = r##"<hp:rect xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1712398788" zOrder="4" numberingType="PICTURE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="638656965" ratio="0">
            <hp:offset x="612" y="4294966253"/>
            <hp:orgSz width="48150" height="3825"/>
            <hp:curSz width="48899" height="4905"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="24449" centerY="2452" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="612" e4="0" e5="1" e6="-1043"/>
                <hc:scaMatrix e1="1.015556" e2="0" e3="-612" e4="0" e5="1.282353" e6="1043"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hp:lineShape color="#000000" width="33" style="NONE" endCap="FLAT" headStyle="NORMAL" tailStyle="NORMAL" headfill="1" tailfill="1" headSz="SMALL_SMALL" tailSz="SMALL_SMALL" outlineStyle="NORMAL" alpha="0"/>
            <hc:fillBrush>
                <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
            </hc:fillBrush>
            <hp:shadow type="NONE" color="#B2B2B2" offsetX="0" offsetY="0" alpha="0"/>
            <hc:pt0 x="0" y="0"/>
            <hc:pt1 x="48150" y="0"/>
            <hc:pt2 x="48150" y="3825"/>
            <hc:pt3 x="0" y="3825"/>
            <hp:sz width="48899" widthRelTo="ABSOLUTE" height="4905" heightRelTo="ABSOLUTE" protect="0"/>
            <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="0" allowOverlap="1" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
            <hp:outMargin left="0" right="0" top="0" bottom="0"/>
            <hp:shapeComment>사각형입니다.</hp:shapeComment>
        </hp:rect>"##;

        let result: Result<Rectangle, _> = from_str(xml);

        assert!(result.is_ok(), "사각형 파싱 실패: {:?}", result.err());

        let rect = result.unwrap();
        assert_eq!(rect.id, Some(1712398788));
        assert_eq!(rect.z_order, 4);
        assert_eq!(rect.ratio, Some(0));
    }

    #[test]
    fn parse_rectangle_minimal() {
        let xml = r##"<hp:rect xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" xmlns:hc="http://www.hancom.co.kr/hwpml/2011/core" id="1" zOrder="0" ratio="20">
            <hp:offset x="0" y="0"/>
            <hp:orgSz width="100" height="100"/>
            <hp:curSz width="100" height="100"/>
            <hp:flip horizontal="0" vertical="0"/>
            <hp:rotationInfo angle="0" centerX="50" centerY="50" rotateimage="0"/>
            <hp:renderingInfo>
                <hc:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
                <hc:scaMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
                <hc:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
            </hp:renderingInfo>
            <hp:lineShape color="#000000" width="1" style="SOLID" endCap="FLAT" headStyle="NORMAL" tailStyle="NORMAL" headfill="0" tailfill="0" headSz="SMALL_SMALL" tailSz="SMALL_SMALL" outlineStyle="NORMAL" alpha="0"/>
            <hc:pt0 x="0" y="0"/>
            <hc:pt1 x="100" y="0"/>
            <hc:pt2 x="100" y="100"/>
            <hc:pt3 x="0" y="100"/>
        </hp:rect>"##;

        let result: Result<Rectangle, _> = from_str(xml);
        assert!(result.is_ok());

        let rect = result.unwrap();
        assert_eq!(rect.ratio, Some(20));
    }
}
