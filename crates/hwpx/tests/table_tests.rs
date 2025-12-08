//! 표 파싱 테스트

use quick_xml::de::from_str;
use hwpx::paragraph::Table;

#[test]
fn parse_simple_table_attributes() {
    let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1854889781" zOrder="2" numberingType="TABLE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1" rowCnt="1" colCnt="3" cellSpacing="0" borderFillIDRef="2" noAdjust="1">
        <hp:sz width="47697" widthRelTo="ABSOLUTE" height="2994" heightRelTo="ABSOLUTE" protect="0"/>
        <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
        <hp:outMargin left="283" right="283" top="283" bottom="283"/>
        <hp:inMargin left="141" right="141" top="141" bottom="141"/>
    </hp:tbl>"#;

    let result: Result<Table, _> = from_str(xml);

    assert!(
        result.is_ok(),
        "간단한 테이블 파싱 실패: {:?}",
        result.err()
    );

    let table = result.unwrap();
    assert_eq!(table.id, Some(1854889781));
    assert_eq!(table.row_count, Some(1));
    assert_eq!(table.column_count, Some(3));
    assert_eq!(table.cell_spacing, 0);
}

#[test]
fn parse_table_with_rows() {
    let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1854889786" zOrder="1" numberingType="TABLE" textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1" rowCnt="1" colCnt="4" cellSpacing="0" borderFillIDRef="2" noAdjust="1">
        <hp:sz width="32981" widthRelTo="ABSOLUTE" height="2582" heightRelTo="ABSOLUTE" protect="0"/>
        <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
        <hp:outMargin left="283" right="283" top="283" bottom="283"/>
        <hp:inMargin left="141" right="141" top="141" bottom="141"/>
        <hp:tr>
            <hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="12">
                <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER" linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
                    <hp:p id="2147483648" paraPrIDRef="51" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0">
                        <hp:run charPrIDRef="37">
                            <hp:t>보도 시점</hp:t>
                        </hp:run>
                    </hp:p>
                </hp:subList>
                <hp:cellAddr colAddr="0" rowAddr="0"/>
                <hp:cellSpan colSpan="1" rowSpan="1"/>
                <hp:cellSz width="5670" height="1282"/>
                <hp:cellMargin left="510" right="510" top="141" bottom="141"/>
            </hp:tc>
        </hp:tr>
    </hp:tbl>"#;

    let result: Result<Table, _> = from_str(xml);

    assert!(
        result.is_ok(),
        "행이 있는 테이블 파싱 실패: {:?}",
        result.err()
    );

    let table = result.unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].cells.len(), 1);
    assert_eq!(table.rows[0].cells[0].name, Some("".to_string()));
}

#[test]
fn parse_table_with_multiple_cells() {
    let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1" rowCnt="1" colCnt="2">
        <hp:sz width="10000" widthRelTo="ABSOLUTE" height="1000" heightRelTo="ABSOLUTE" protect="0"/>
        <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
        <hp:outMargin left="0" right="0" top="0" bottom="0"/>
        <hp:inMargin left="0" right="0" top="0" bottom="0"/>
        <hp:tr>
            <hp:tc name="첫번째" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="1">
                <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP" linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
                    <hp:p id="0" pageBreak="0" columnBreak="0" merged="0"/>
                </hp:subList>
                <hp:cellAddr colAddr="0" rowAddr="0"/>
                <hp:cellSpan colSpan="1" rowSpan="1"/>
                <hp:cellSz width="5000" height="1000"/>
                <hp:cellMargin left="0" right="0" top="0" bottom="0"/>
            </hp:tc>
            <hp:tc name="두번째" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="1">
                <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP" linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
                    <hp:p id="1" pageBreak="0" columnBreak="0" merged="0"/>
                </hp:subList>
                <hp:cellAddr colAddr="1" rowAddr="0"/>
                <hp:cellSpan colSpan="1" rowSpan="1"/>
                <hp:cellSz width="5000" height="1000"/>
                <hp:cellMargin left="0" right="0" top="0" bottom="0"/>
            </hp:tc>
        </hp:tr>
    </hp:tbl>"#;

    let result: Result<Table, _> = from_str(xml);
    assert!(result.is_ok(), "여러 셀 테이블 파싱 실패: {:?}", result.err());

    let table = result.unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].cells.len(), 2);
    assert_eq!(table.rows[0].cells[0].name, Some("첫번째".to_string()));
    assert_eq!(table.rows[0].cells[1].name, Some("두번째".to_string()));
}

#[test]
fn parse_table_cell_with_paragraph() {
    let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1" rowCnt="1" colCnt="1">
        <hp:sz width="10000" widthRelTo="ABSOLUTE" height="1000" heightRelTo="ABSOLUTE" protect="0"/>
        <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
        <hp:outMargin left="0" right="0" top="0" bottom="0"/>
        <hp:inMargin left="0" right="0" top="0" bottom="0"/>
        <hp:tr>
            <hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="1">
                <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP" linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
                    <hp:p id="0" pageBreak="0" columnBreak="0" merged="0">
                        <hp:run charPrIDRef="1">
                            <hp:t>셀 내용</hp:t>
                        </hp:run>
                    </hp:p>
                </hp:subList>
                <hp:cellAddr colAddr="0" rowAddr="0"/>
                <hp:cellSpan colSpan="1" rowSpan="1"/>
                <hp:cellSz width="10000" height="1000"/>
                <hp:cellMargin left="0" right="0" top="0" bottom="0"/>
            </hp:tc>
        </hp:tr>
    </hp:tbl>"#;

    let result: Result<Table, _> = from_str(xml);
    assert!(result.is_ok());

    let table = result.unwrap();
    let cell = &table.rows[0].cells[0];
    assert_eq!(cell.paragraph_list.paragraphs.len(), 1);
}

#[test]
fn parse_table_repeat_header() {
    let xml = r#"<hp:tbl xmlns:hp="http://www.hancom.co.kr/hwpml/2011/paragraph" id="1" rowCnt="1" colCnt="1" repeatHeader="1">
        <hp:sz width="10000" widthRelTo="ABSOLUTE" height="1000" heightRelTo="ABSOLUTE" protect="0"/>
        <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
        <hp:outMargin left="0" right="0" top="0" bottom="0"/>
        <hp:inMargin left="0" right="0" top="0" bottom="0"/>
    </hp:tbl>"#;

    let result: Result<Table, _> = from_str(xml);
    assert!(result.is_ok());

    let table = result.unwrap();
    assert!(table.repeat_header);
}
