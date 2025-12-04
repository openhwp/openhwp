# 9 Header XML 스키마

> **출처**: KS X 6101:2024, 페이지 26-41

## 9.1 네임스페이스

Header XML은 기본적으로 `http://www.owpml.org/owpml/2024/head`를 기본 네임스페이스로 사용한다.
기본 네임스페이스의 접두어(prefix)는 기본적으로 "hh"를 사용한다. 잘못된 사용을 줄이기 위해서 "hh"를 기본 네임스페이스(`http://www.owpml.org/owpml/2024/head`) 이외의 네임스페이스에 사용하지 않는 것을 권고한다.

## 9.2 헤더 XML 구조

### 9.2.1 헤더 구조

`<head>` 요소는 header.xml 파일에서 최상위 요소로서, 문서 내용에 관련된 모든 설정들을 하위 요소로 가지고 있다. `<head>` 요소는 네 개의 하위 요소를 가지고 있다. 각 하위 요소에 대한 설명은 다음에 오는 항들에서 자세하게 설명한다.

#### 그림 5 — `<head>`의 구조

```
head (Root Element)
└── Type: HWPMLHeadType
    └── Attributes
        └── beginNum (시작 번호)
    └── refList (Type: MappingTableType)
    └── forbiddenWordList (Type: ForbiddenWordListType)
    └── compatibleDocument (Type: CompatibleDocumentType)
    └── trackchangeConfig
```

#### 표 11 — head version

| 속성 이름 | 설명                                                           |
| --------- | -------------------------------------------------------------- |
| version   | OWPML Header XML의 버전. 이 문서 기준으로 현재 버전은 1.0이다. |

#### 표 12 — head 속성

| 하위 요소 이름     | 설명                                                            |
| ------------------ | --------------------------------------------------------------- |
| beginNum           | 문서 내에서 각종 객체들의 시작 번호 정보를 가지고 있는 요소     |
| refList            | 본문에서 사용될 각종 데이터에 대한 맵핑 정보를 가지고 있는 요소 |
| forbiddenWordList  | 금칙 문자 목록을 가지고 있는 요소                               |
| compatibleDocument | 문서 호환성 설정                                                |
| trackchangeConfig  | 변경 추적 정보와 암호 정보를 가지고 있는 요소                   |
| docOption          | 연결 문서 정보와 저작권 관련 정보를 가지고 있는 요소            |
| metaTag            | 메타태그 정보를 가지고 있는 요소                                |

### 9.2.2 beginNum 요소

`<beginNum>` 요소는 문서 내에서 사용되는 각종 객체들의 번호의 시작 숫자를 설정하기 위한 요소이다. 기본적으로 시작 번호는 1에서 시작되며, 사용자 설정에 의해서 1 이외의 번호에서 시작할 수 있게 된다. 시작 번호를 지정할 수 있는 객체에는 [페이지, 각주, 미주, 그림, 표, 수식] 등이 있다.

#### 그림 6 — `<beginNum>`의 구조

```
beginNum (시작 번호)
└── Attributes
    ├── page (Type: xs:positiveInteger) — 페이지 시작 번호
    ├── footnote (Type: xs:positiveInteger) — 각주 시작 번호
    ├── endnote (Type: xs:positiveInteger) — 미주 시작 번호
    ├── pic (Type: xs:positiveInteger) — 그림 시작 번호
    ├── tbl (Type: xs:positiveInteger) — 표 시작 번호
    └── equation (Type: xs:positiveInteger) — 수식 시작 번호
```

#### 표 13 — beginNum 속성

| 속성 이름 | 설명             |
| --------- | ---------------- |
| page      | 페이지 시작 번호 |
| footnote  | 각주 시작 번호   |
| endnote   | 미주 시작 번호   |
| pic       | 그림 시작 번호   |
| tbl       | 표 시작 번호     |
| equation  | 수식 시작 번호   |

#### 샘플 5 — beginNum 예

```xml
<hh:beginNum page="1" footnote="1" endnote="1" pic="1" tbl="1" equation="1"/>
```

### 9.2.3 refList 요소

`<refList>` 요소는 본문에서 사용되는 각종 설정 데이터를 가지고 있는 요소이다. `<refList>` 요소는 header XML에서 대부분의 설정 정보를 가지고 있다.

하위 요소에 대한 자세한 설명은 **9.3**에서 서술한다.

#### 그림 7 — `<refList>`의 구조

```
refList (Type: MappingTableType)
├── fontfaces — 글꼴 정보목록
├── borderFills — 테두리/배경/채우기 정보목록
├── charProperties — 글자 모양 정보
├── tabProperties — 탭 정보
├── numberings — 번호 붙임 모양
├── bullets — 글머리표 문단 모양
├── paraProperties — 문단 모양
├── styles — 스타일목록
├── memoProperties — 메모 모양
├── trackChanges — 변경 추적
└── trackChangeAuthors — 변경추적 작성자
```

#### 표 14 — refList 요소

| 하위 요소 이름     | 설명                         |
| ------------------ | ---------------------------- |
| fontfaces          | 글꼴 정보 목록               |
| borderFills        | 테두리/배경/채우기 정보 목록 |
| charProperties     | 글자 모양 목록               |
| tabProperties      | 탭 정의 목록                 |
| numberings         | 번호 문단 모양 목록          |
| bullets            | 글머리표 문단 모양 목록      |
| paraProperties     | 문단 모양 목록               |
| styles             | 스타일 목록                  |
| memoProperties     | 메모 모양 목록               |
| trackChanges       | 변경 추적 정보 목록          |
| trackChangeAuthors | 변경 추적 검토자 목록        |

#### 샘플 6 — refList 예

```xml
<hh:refList>
  <hh:fontfaces itemCnt="7">
    <hh:fontface lang="HANGUL" fontCnt="2">
      <hh:font id="0" face="함초롬돋움" type="TTF" isEmbedded="0">
        <hh:typeInfo familyType="FCAT_GOTHIC" weight="6" proportion="4" contrast="0" strokeVariation="1"
          armStyle="1" letterform="1" midline="1" xHeight="1"/>
      </hh:font>
    </hh:fontface>
  </hh:fontfaces>
  ...
</hh:refList>
```

### 9.2.4 forbiddenWordList 요소

`<forbiddenWordList>` 요소는 금칙 문자의 목록이다.

#### 그림 8 — `<forbiddenWordList>`의 구조

```
forbiddenWordList (Type: ForbiddenWordListType)
├── Attributes
│   └── itemCnt — 금칙 문자의 개수
└── forbiddenWord (1..∞, Type: xs:string)
```

#### 표 15 — forbiddenWordList 속성

| 속성 이름 | 설명             |
| --------- | ---------------- |
| itemCnt   | 금칙 문자의 개수 |

| 하위 요소 이름 | 설명                                                                     |
| -------------- | ------------------------------------------------------------------------ |
| forbiddenWord  | 금칙 문자. 요소의 값으로 문자열을 가짐. 해당 요소의 추가적인 설명은 생략 |

`<forbiddenWord>` 자식 요소는 요소 값으로 금칙 문자열을 가지는 단순 형식의 요소이다. 다음은 `<forbiddenWordList>` 요소의 예시이다.

#### 샘플 7 — forbiddenWordList 예

```xml
<forbiddenWordList itemCnt="2">
  <forbiddenWord>d</forbiddenWord>
  <forbiddenWord>f</forbiddenWord>
</forbiddenWordList>
```

### 9.2.5 compatibleDocument 요소

#### 9.2.5.1 compatibleDocument

`<compatibleDocument>` 요소는 이 표준에서 정의하고 있는 문서 형식을 다른 형태의 문서 형식으로 저장할 때 사용되는 정보를 가지고 있는 요소이다.

#### 그림 9 — `<compatibleDocument>`의 구조

```
compatibleDocument (Type: CompatibleDocumentType)
├── Attributes
│   └── targetProgram — 대상 프로그램
└── layoutCompatibility — 문서 포맷간 정보
```

#### 표 16 — compatibleDocument 속성

| 속성 이름     | 설명          |
| ------------- | ------------- |
| targetProgram | 대상 프로그램 |

| 하위 요소 이름      | 설명                 |
| ------------------- | -------------------- |
| layoutCompatibility | 레이아웃 호환성 설정 |

#### 샘플 8 — compatibleDocument 예

```xml
<hh:compatibleDocument targetProgram="HWP201X">
  <hh:layoutCompatibility/>
</hh:compatibleDocument>
```

#### 9.2.5.2 layoutCompatibility 요소

`<layoutCompatibility>` 요소는 HWP 문서를 다른 형식의 문서로 변환시킬 때 필요한 설정 정보이다. 즉, HWP 문서를 OOXML 워드 문서 또는 ODF 워드 문서로 변환시킬 경우, HWP 문서에서는 지원되지만 OOXML 워드 문서 또는 ODF 워드 문서에서 지원되지 않는 레이아웃 설정 등을 어떤 방식으로 변환시킬 것인지에 대한 설정이다. 하위 요소가 나타나는 경우에는 그 값이 사용되는 경우이고 나타나지 않는 경우는 사용되지 않는 경우이다.

#### 그림 10 — `<layoutCompatibility>`의 구조

(다수의 하위 요소 - 표 17 참조)

#### 표 17 — layoutCompatibility 요소

| 하위 요소 이름                     | 설명                                                                                  |
| ---------------------------------- | ------------------------------------------------------------------------------------- |
| applyFontWeightToBold              | 진하게 글자에 글꼴의 너비를 적용함.                                                   |
| useInnerUnderline                  | 밑줄 위치를 줄 영역의 안쪽으로 그음.                                                  |
| fixedUnderlineWidth                | 밑줄, 취소선 두께에 글자 크기를 반영하지 않음.                                        |
| doNotApplyStrikeoutWithUnderline   | 밑줄과 함께 취소선 취소선을 적용하지 않음.                                            |
| useLowercaseStrikeout              | 취소선을 영문 소문자 기준으로 그음.                                                   |
| extendLineheightToOffset           | 글자 위치의 강조점에 의한 영역까지 줄 높이를 확장함.                                  |
| applyFontspaceToLatin              | 라틴어 사이의 빈칸에 글꼴에 어울리는 빈칸을 적용함.                                   |
| treatQuotationAsLatin              | 인용 부호를 글꼴에 어울리는 빈칸에서 라틴어로 취급함.                                 |
| doNotApplyDiacSymMarkOfNoneAndSix  | 강조점의 [없음]과 6개 의의 목록을 적용하지 않음.                                      |
| doNotAlignWhitespaceOnRight        | 줄의 가장 오른쪽 빈칸을 다음 줄로 넘기지 않음.                                        |
| doNotAdjustWordInJustify           | 양쪽 정렬에서 단어의 문자간 간격을 보정하지 않음.                                     |
| baseCharUnitOnEAsian               | 글자 단위를 바탕글 스타일의 한글 크기를 기준으로 적용함.                              |
| baseCharUnitOfIndentOnFirstChar    | 들여쓰기/내어쓰기의 글자 단위를 문단 첫 글자의 크기를 기준으로 적용함.                |
| adjustLineheightToFont             | 기본 줄 높이를 글꼴에 맞춰서 조정함.                                                  |
| adjustBaselineInFixedLinespacing   | 줄 간격의 [고정값]에서 기준선을 새로 정렬에 따라 조정함.                              |
| applyPrevspacingBeneathObject      | 개체 이래 문단의 위 간격을 개체 기준으로 적용함.                                      |
| applyNextspacingOfLastPara         | 마지막 문단의 아래 간격을 영역에 포함하여 확장함.                                     |
| applyAtLeastToPercent100Pct        | 줄 간격의 [최소]를 [글자에 따라]에서 100 %로 적용함.                                  |
| doNotApplyAutoSpaceEAsianEng       | 한글과 영어 간격에 자동 조절을 적용하지 않음.                                         |
| doNotApplyAutoSpaceEAsianNum       | 한글과 숫자 간격에 자동 조절을 적용하지 않음.                                         |
| adjustParaBorderfillToSpacing      | 문단 테두리/배경의 영역을 문단 어백과 위, 아래 줄 간격을 제외하고 줄 간격에만 적용함. |
| connectParaBorderfillOfEqualBorder | 문단 테두리가 같은 문단의 문단 테두리/배경을 연결함.                                  |
| adjustParaBorderOffsetWithBorder   | 문단 테두리/배경의 간격을 테두리 설정 시에 적용함.                                    |
| extendLineheightToParaBorderOffset | 문단 테두리의 굵기와 간격의 영역까지 줄 높이를 확장.                                  |
| applyParaBorderToOutside           | 문단 테두리를 지정된 영역의 바깥쪽으로 적용함.                                        |
| applyMinColumnWidthTo1mm           | 단 영역의 최소 폭을 1 mm로 지정함.                                                    |
| applyTabPosBasedOnSegment          | 탭 위치를 개체에 의해 배치된 영역을 기준으로 적용함.                                  |
| breakTabOverLine                   | 줄 영역을 넘어선 탭을 다음 줄로 넘김.                                                 |
| adjustVertPosOfLine                | 줄 간격에 따라 줄의 위치를 조정함.                                                    |
| doNotApplyWhiteSpaceHeight         | white space 문자의 글자 크기를 줄 높이에 반영하지 않음.                               |
| doNotAlignLastPeriod               | 줄의 마지막 마침표를 다음 줄로 넘기지 않음.                                           |
| doNotAlignLastForbidden            | 줄의 마지막 금칙 문자를 다음 줄로 넘기지 않음.                                        |
| baseLineSpacingOnLineGrid          | 줄 격자의 간격을 줄 간격의 기준으로 적용함.                                           |
| applyCharSpacingToCharGrid         | 글자 격자의 간격을 글자에 따른 자간으로 적용함.                                       |
| doNotApplyGridInHeaderFooter       | 머리말, 꼬리말에 줄/글자 격자를 적용하지 않음.                                        |
| applyExtendHeaderFooterEachSection | 본문 영역으로 확장되는 구역 단위 머리말, 꼬리말을 적용함.                             |
| doNotApplyHeaderFooterAtNoSpace    | 머리말, 꼬리말 영역이 없을 때에는 머리말, 꼬리말을 적용하지 않음.                     |
| doNotApplyColSeparatorAtNoGap      | 단 사이의 간격이 없을 때에는 단 구분선을 적용하지 않음.                               |
| doNotApplyLinegridAtNoLinespacing  | 줄 간격이 없으면 줄 격자의 간격을 적용하지 않음.                                      |
| doNotApplyImageEffect              | 그림 효과를 적용하지 않음.                                                            |
| doNotApplyShapeComment             | 개체 설명문 적용하지 않음.                                                            |
| doNotAdjustEmptyAnchorLine         | 조판 부호만 있는 빈 줄에 개체 배치를 조정하지 않음.                                   |
| overlapBothAllowOverlap            | 개체 두 개가 서로 겹침 허용인 경우에만 서로 겹침.                                     |
| doNotApplyVertOffsetOfForward      | 조판 부호 다음 쪽으로 넘겨진 개체에 세로 위치를 적용하지 않음.                        |
| extendVertLimitToPageMargins       | 문단 기준 개체의 세로 위치를 종이 영역까지 확장함.                                    |
| doNotHoldAnchorOfTable             | 문단 기준 표의 조판 부호는 쪽 넘김을 방지하지 않음.                                   |
| doNotFormattingAtBeneathAnchor     | 문단과 조판 부호 다음 쪽으로 넘겨진 개체 사이 영역에 본단을 배치하지 않음.            |
| adjustBaselineOfObjectToBottom     | 글자처럼 취급한 개체의 기준선을 개체 아래쪽으로 조정함                                |
| doNotApplyExtensionCharcharPr      | 글자 겹치기의 확장 기능을 적용하지 않음.                                              |

### 9.2.6 trackChangeConfig 요소

#### 9.2.6.1 trackChangeConfig

`<trackChangeConfig>`는 변경 추적에 대한 상태 정보와 암호 정보를 가지고 있다.

#### 그림 11 — `<trackChangeConfig>`의 구조

```
trackchangeConfig
├── Attributes
│   └── flags (Type: xs:nonNegativeInteger) — 변경 추적 상태 정보
└── config-item-set — 변경 추적 암호 정보
```

#### 표 18 — trackChangeConfig 요소

| 속성 이름 | 설명                |
| --------- | ------------------- |
| flags     | 변경 추적 상태 정보 |

| 하위 요소 이름  | 설명                |
| --------------- | ------------------- |
| config-item-set | 변경 추적 암호 정보 |

`<trackChangeConfig>`의 하위 속성인 flag 값은 변경 추적 문서의 상태 및 표시 정보 값을 가지고 있다.

#### 표 19 — flag 값

| flag 값    | 설명                     |
| ---------- | ------------------------ |
| 0x00000001 | 변경 추적 상태           |
| 0x00000002 | 변경 추적 원본           |
| 0x00000004 | 변경 내용 안보기         |
| 0x00000008 | 변경 추적 문장 안 표시   |
| 0x00000010 | 변경 추적 서식 표시      |
| 0x00000020 | 변경 추적 삽입/삭제 표시 |

#### 9.2.6.2 config-item-set 요소

`<config-item-set>` 요소는 변경 추적 암호 정보를 갖고 있는 요소로 **13.2.2**의 속성을 따른다.

#### 그림 12 — `<config-item-set>`의 구조

```
config-item-set
├── Attributes
│   └── name (Type: xs:string)
└── config-item
```

#### 샘플 9 — config-item-set 예

```xml
<config:config-item-set name="TrackChangePasswordInfo">
  <config:config-item name="algorithm-name" type="string">PBKDF2</config:config-item>
  <config:config-item name="salt" type="base64Binary">nsJ...</config:config-item>
  </config:config-item>
  <config:config-item name="iteration-count" type="int">1024</config:config-item>
  </config:config-item>
  <config:config-item name="hash" type="base64Binary">j2E...</config:config-item>
  </config:config-item>
</config:config-item-set>
```

### 9.2.7 docOption 요소

`<docOption>`은 연결 문서 정보와 저작권 관련 정보를 가지고 있는 요소이다.

#### 그림 13 — docOption 요소

```
docOption (Type: DocOptionType)
├── linkinfo — Attributes
└── licensemark — Attributes
```

#### 표 20 — docOption 요소

| 하위 요소 이름 | 설명             |
| -------------- | ---------------- |
| linkinfo       | 연결 문서 정보   |
| licensemark    | 저작권 관련 정보 |

#### 9.2.7.1 linkinfo 요소

`<linkinfo>`는 연결 문서 정보를 가지고 있는 요소이다.

#### 표 21 — linkinfo 요소

| 속성 이름       | 설명                            |
| --------------- | ------------------------------- |
| path            | 연결된 문서의 경로              |
| pageInherit     | 연결 인쇄 — 쪽 번호 잇기 여부   |
| footnoteInherit | 연결 인쇄 — 각주 번호 잇기 여부 |

#### 9.2.7.2 licensemark 요소

`<licensemark>`는 저작권 관련 정보를 가지고 있는 요소이다.

#### 표 22 — licensemark 요소

| 속성 이름 | 설명             |
| --------- | ---------------- |
| type      | 저작권 유형      |
| flag      | 저작권 제한 정보 |
| lang      | 국가 코드        |

`<licensemark>`의 하위 속성인 flag 값은 저작권 제한 정보에 대한 값을 가지고 있다.

#### 표 23 — flag 값

| flag 값    | 설명                    |
| ---------- | ----------------------- |
| 0x00000001 | 상업적 이용 제한        |
| 0x00000002 | 복제 제한               |
| 0x00000004 | 동일 조건하에 복제 허가 |

### 9.2.8 metaTag 요소

`<metaTag>`는 메타 태그에 대한 정보를 가지고 있는 요소이다. json object 형식으로 표현된다.

#### 그림 14 — metaTag

```
metaTag ─── hc:MetaTagType
(Type: hc:MetaTagType)
```

#### 샘플 10 — metaTag 예

```xml
<!--fieldBegin 요소의 metaTag-->
<hp:fieldBegin id="1795169102" type="CLICK_HEAR" name="" editable="1" dirty="0" zorder="-1"
  fieldid="627272811">
  <hp:parameters cnt="3" name="">
    <hp:integerParam name="Prop">9</hp:integerParam>
    <hp:stringParam name="Command"
      xml:space="preserve">Clickhere:set:66:Direction:wstring:23:이곳을 마우스로 누르고 내용을 입력하세
      요. HelpState:wstring:0: </hp:stringParam>
    <hp:stringParam name="Direction">이곳을 마우스로 누르고 내용을 입력하세
      요.</hp:stringParam>
  </hp:parameters>
  <hp:metaTag>{"name":"#서플릿"}</hp:metaTag>
</hp:fieldBegin>

<!--tbl 요소의 metaTag-->
<hp:tbl id="1793424928" zOrder="0" numberingType="TABLE" textWrap="TOP_AND_BOTTOM"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1"
  rowCnt="2" colCnt="2" cellSpacing="0" borderFillIDRef="3" noAdjust="0">
  ......
  <hp:tr>
    <hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0"
      borderFillIDRef="3">
      <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER"
        linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0"
        metatag="{"name":"","#이름"}">
      ......
    </hp:tc>
  </hp:tr>
  ......
</hp:tbl>

......

<xs:complexType name="MetaTagType" mixed="true"/>
```

## 9.3 문서 설정 정보

### 9.3.1 문서 설정

문서 설정 정보는 문서 내에서 사용되는 각종 글꼴 정보, 글자 모양 정보, 테두리/배경 정보와 같이 문서의 레이아웃 설정 및 모양 설정 등을 가지고 있다.

### 9.3.2 fontfaces 요소

#### 9.3.2.1 fontfaces

문서 내에서 글꼴 정보는 반드시 1개 이상 정의되어 있어야 한다. 내용이 없는 문서라도 기본 글꼴 정보는 정의되어 있어야 한다. 헤더 스키마 상에서는 속성 itemCnt의 값으로 올 수 있는 범위가 1 이상으로(positiveInteger) 제한되어 있으며, 자식 요소인 `<fontface>` 요소의 개수 한정자 역시 1 이상으로 정의되어 있다.

#### 그림 15 — `<fontfaces>`의 구조

```
fontfaces
├── Attributes
│   └── itemCnt
└── fontface (1..∞, Type: FontfaceType)
```

#### 표 24 — fontfaces 요소

| 속성 이름 | 설명             |
| --------- | ---------------- |
| itemCnt   | 글꼴 정보의 개수 |

| 하위 요소 이름 | 설명      |
| -------------- | --------- |
| Fontface       | 글꼴 정보 |

#### 샘플 11 — fontfaces 예

```xml
<hh:fontfaces itemCnt="7">
  <hh:fontface lang="HANGUL" fontCnt="2">
    <hh:font id="0" face="함초롬돋움" type="TTF" isEmbedded="0">
      <hh:typeInfo familyType="FCAT_GOTHIC" weight="6" proportion="4" contrast="0" strokeVariation="1"
        armStyle="1" letterform="1" midline="1" xHeight="1"/>
    </hh:font>
    <hh:font id="1" face="함초롬바탕" type="TTF" isEmbedded="0">
      <hh:typeInfo familyType="FCAT_GOTHIC" weight="6" proportion="4" contrast="0" strokeVariation="1"
        armStyle="1" letterform="1" midline="1" xHeight="1"/>
    </hh:font>
  </hh:fontface>
</hh:fontfaces>
```

#### 9.3.2.2 fontface 요소

##### 9.3.2.2.1 fontface

글꼴 정보는 언어별로 정의된다. 현재 이 문서에서 지원되고 있는 언어 형식으로는 [한글, 라틴, 한자, 일어, 기타, 심볼, 사용자]가 있다. [한글, 라틴, 한자, 일어, 심볼] 언어 형식의 구분은 Unicode 4.0을 참고한다. [기타] 언어 형식의 구분은 RTL(Right to Left) 표기방식의 언어이며 [사용자] 언어형식의 구분은 PUA(Private Unicode Area) 영역을 말한다.

#### 그림 16 — `<fontface>`의 구조

```
fontface (Type: FontfaceType)
├── Attributes
│   ├── lang (Type: restriction of xs:string) — 언어 (한글, 영어, 한자, 일어, 기타, 심볼, 사용자)
│   └── fontCnt (Type: xs:nonNegativeInteger) — 글꼴의 갯수
└── font (1..∞) — 글꼴
```

#### 표 25 — fontface 요소

| 속성 이름 | 설명                    |
| --------- | ----------------------- |
| lang      | 글꼴이 적용될 언어 유형 |
| fontCnt   | 글꼴의 개수             |

| 하위 요소 이름 | 설명 |
| -------------- | ---- |
| font           | 글꼴 |

##### 9.3.2.2.2 font 요소

HWP 문서 스키마는 내장 글꼴을 지원한다. 글꼴이 내장된 경우, 글꼴 데이터 파일은 다른 바이너리 파일과 마찬가지로 컨테이너 내에 바이너리 형태로 포함이 되고 manifest에 해당 정보를 기록한다. `<font>` 엘리먼트에서는 manifest에 정의된 정보를 참조해서 내장된 글꼴에 접근하게 된다.

속성 @isEmbedded의 값이 참(true)인 경우, 반드시 컨테이너 내에 글꼴을 내장하고 속성 @binaryItemIDRef의 값이 유효한 값이어야 한다. 만약 속성 @isEmbedded 값이 참(true)인데 속성 @binaryItemIDRef의 값이 유효하지 않다면 애플리케이션에서는 이를 오류 상황으로 인식해야 한다.

속성 @isEmbedded의 값이 거짓(false)인 경우, 애플리케이션은 사용자 시스템에 내장된 글꼴을 사용해야 한다. 이 경우 속성 @binaryItemIDRef은 사용되지 않는다. 속성 @isEmbedded의 값이 거짓(false)일 때 사용자 시스템 내에 정의된 글꼴이 없는 경우 애플리케이션은 이를 오류 상황으로 인식해야 한다.

정의된 글꼴이 없는 오류 상황에서 애플리케이션은 대체 글꼴을 먼저 사용해야 한다. 대체 글꼴마저 없는 경우에 대한 처리 방법은 이 표준에서는 정의하지 않지만, 시스템 기본 글꼴을 사용하는 것을 권고한다. 이 처리 방법은 애플리케이션마다 다를 수 있으므로, 콘텐츠 제작자는 이를 인지하고서 콘텐츠를 제작해야 한다.

#### 그림 17 — `<font>`의 구조

```
font
├── Attributes
│   ├── id (Type: xs:ID) — 글꼴 아이디
│   ├── face (Type: xs:string) — 글꼴 이름
│   ├── type (Type: restriction of xs:string) — 글꼴의 유형 (rep, 대표글꼴, ttf, 트루타입글꼴, hft, 한글전용글꼴)
│   ├── isEmbedded (Type: xs:boolean, Default: false)
│   └── binaryItemIDRef (Type: xs:IDREF)
└── Elements
    ├── substFont — 대체 글꼴
    └── typeInfo — 글꼴 유형 정보
```

#### 표 26 — font 요소

| 속성 이름       | 설명                                                                                 |
| --------------- | ------------------------------------------------------------------------------------ |
| id              | 글꼴을 식별하기 위한 아이디                                                          |
| face            | 글꼴의 이름                                                                          |
| type            | 글꼴의 유형                                                                          |
| isEmbedded      | 글꼴 파일이 문서 컨테이너 내에 포함되었는지 여부                                     |
| binaryItemIDRef | 글꼴 파일이 문서 컨테이너 내에 포함된 경우 해당 글꼴 파일을 지정하기 위한 ID 참조 값 |

#### 표 27 — font 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| substFont      | 대체 글꼴      |
| typeInfo       | 글꼴 유형 정보 |

애플리케이션에서는 `<font>` 요소에서 정의된 글꼴이 없는 경우 가장 먼저 `<substFont>` 요소에 정의된 글꼴을 사용해야 한다. 대체 글꼴마저 없는 경우 시스템 기본 글꼴을 사용하는 것을 권고한다. 하지만 이에 대한 처리는 애플리케이션마다 다를 수 있다.

#### 그림 18 — `<substFont>`의 구조

```
substFont (대체 글꼴)
└── Attributes
    ├── face (Type: xs:string) — 글꼴 이름
    ├── type (Type: Restriction of xs:string) — 글꼴의 유형
    ├── isEmbedded (Type: xs:boolean, Default: false)
    └── binaryItemIDRef (Type: xs:string)
```

#### 표 28 — substFont 요소

| 속성 이름       | 설명                                                                                |
| --------------- | ----------------------------------------------------------------------------------- |
| face            | 글꼴의 이름                                                                         |
| type            | 글꼴의 유형                                                                         |
| isEmbedded      | 글꼴 파일이 문서 컨테이너 내에 포함되었는지 여부                                    |
| binaryItemIDRef | 글꼴 파일이 문서 컨테이너 내에 포함된 경우 해당 글꼴 파일을 지정하기 위한 ID 참조값 |

### typeInfo 요소

글꼴의 유형 설정을 표현하기 위한 요소이다. 자세한 설명은 다음 스키마 **그림 19**를 참조한다.

#### 그림 19 — `<typeInfo>`의 구조

```
typeInfo
└── Attributes
    ├── familyType (Type: Restriction of xs:string) — 글꼴 계열
    ├── serifStyle (Type: xs:string) — 세리프 유형
    ├── weight (Type: xs:integer) — 굵기
    ├── proportion (Type: xs:integer) — 비례
    ├── contrast (Type: xs:integer) — 대조
    ├── strokeVariation (Type: xs:integer) — 스트로크 편차
    ├── armStyle (Type: xs:boolean) — 자획 유형
    ├── letterform (Type: xs:boolean) — 글자형
    ├── midline (Type: xs:boolean) — 중간선
    └── xHeight (Type: xs:integer) — X-높이
```

#### 표 29 — typeInfo 속성

| 속성 이름       | 설명          |
| --------------- | ------------- |
| familyType      | 글꼴 계열     |
| serifStyle      | 세리프 유형   |
| weight          | 굵기          |
| proportion      | 비례          |
| contrast        | 대조          |
| strokeVariation | 스트로크 편차 |
| armStyle        | 자획 유형     |
| letterform      | 글자형        |
| midline         | 중간선        |
| xHeight         | X-높이        |

## 9.3.3 borderFills 요소

### 9.3.3.1 borderFills

한 문서 내에서는 다양한 테두리/배경 정보들이 사용되는데 이런 테두리/배경 정보를 목록 형태로 가지고 있는 요소이다.

#### 그림 20 — `<borderFills>`의 구조

```
borderFills (테두리/배경/채우기 정보목록)
├── Attributes
│   └── itemCnt
└── borderFill (0..∞, Type: BorderFillType)
```

#### 표 30 — borderFills 요소

| 속성 이름 | 설명                           |
| --------- | ------------------------------ |
| itemCnt   | 테두리/배경/채우기 정보의 개수 |

#### 표 31 — borderFills 하위 요소

| 하위 요소 이름 | 설명                    |
| -------------- | ----------------------- |
| borderFill     | 테두리/배경/채우기 정보 |

#### 샘플 12 — borderFills 예

```xml
<hh:borderFills itemCnt="2">
  <hh:borderFill id="1" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
    <hh:slash type="NONE" Crooked="0" isCounter="0"/>
    <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
    <hh:leftBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:rightBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:topBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
  </hh:borderFill>
  <hh:borderFill id="2" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
    <hh:slash type="NONE" Crooked="0" isCounter="0"/>
    <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
    <hh:leftBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:rightBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:topBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:bottomBorder type="NONE" width="0.1 mm" color="#000000"/>
    <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
    <hh:fillBrush>
      <hc:winBrush faceColor="none" hatchColor="#999999" alpha="0"/>
    </hh:fillBrush>
  </hh:borderFill>
</hh:borderFills>
```

### 9.3.3.2 borderFill 요소

#### 9.3.3.2.1 borderFill

테두리/배경/채우기 정보에는 페이지의 테두리/배경/채우기 정보뿐만 아니라 표, 그림 등의 테두리/배경/채우기 정보까지 포함되어 있다. 이러한 특성으로 인해서 특정 속성 또는 특정 자식 요소는 특정 객체에서 사용되지 않을 수 있다. 대표적으로 속성 **breakCellSEparateLine**은 표에서만 사용되는 속성으로 페이지, 그림 등에서는 사용되지 않는다.

#### 그림 21 — `<borderFill>`의 구조

```
borderFill (Type: BorderFillType)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 테두리/채우기 항목 아이디
│   ├── threeD (Type: xs:boolean, Default: false) — 3D효과 on/off
│   ├── shadow (Type: xs:boolean, Default: false) — 그림자 효과 on/off
│   ├── centerLine (Type: Restriction of xs:string) — 중심선 종류
│   └── breakCellSeparateLine (Type: xs:boolean, Default: false) — 자동으로 나뉜 표의 경계선 설정 여부
└── Elements
    ├── slash (Type: SlashType)
    ├── backSlash (Type: SlashType)
    ├── leftBorder (Type: BorderType) — 왼쪽 테두리
    ├── rightBorder (Type: BorderType) — 오른쪽 테두리
    ├── topBorder (Type: BorderType) — 위쪽 테두리
    ├── bottomBorder (Type: BorderType) — 아래쪽 테두리
    ├── diagonal (Type: BorderType) — 대각선
    └── fillBrush (Type: hc:FillBrushType) — 채우기 정보
```

#### 표 32 — borderFill 요소

| 속성 이름             | 설명                                           |
| --------------------- | ---------------------------------------------- |
| id                    | 테두리/배경/채우기 정보를 구별하기 위한 아이디 |
| threeD                | 3D 효과의 사용 여부                            |
| shadow                | 그림자 효과의 사용 여부                        |
| centerLine            | 중심선 종류                                    |
| breakCellSEparateLine | 자동으로 나뉜 표의 경계선 설정 여부            |

#### 표 33 — borderFill 하위 요소

| 하위 요소 이름 | 설명                                       |
| -------------- | ------------------------------------------ |
| slash          | slash 대각선 모양 설정. 9.3.3.2.2 참조     |
| backSlash      | backSlash 대각선 모양 설정. 9.3.3.2.2 참조 |
| leftBorder     | 왼쪽 테두리. 9.3.3.2.3 참조                |
| rightBorder    | 오른쪽 테두리. 9.3.3.2.3 참조              |
| topBorder      | 위쪽 테두리. 9.3.3.2.3 참조                |
| bottomBorder   | 아래쪽 테두리. 9.3.3.2.3 참조              |
| diagonal       | 대각선. 9.3.3.2.3 참조                     |
| fillBrush      | 채우기 정보                                |

#### 9.3.3.2.2 SlashType

테두리/배경 설정 중, 대각선의 정보를 담기 위한 요소이다.

#### 그림 22 — `<SlashType>`의 구조

```
SlashType
└── Attributes
    ├── type (Type: Restriction of xs:string)
    ├── Crooked (Type: xs:boolean)
    └── isCounter (Type: xs:boolean)
```

#### 표 34 — SlashType 요소

| 속성 이름 | 설명                                                                                                                                                                                |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| type      | Slash/BackSlash의 모양. NONE: 없음, CENTER: 중심선만 (⟋ ⟍), CENTER_BELOW: 중심선 + 중심선 아래선 (⟋ ⟍), CENTER_ABOVE: 중심선 + 중심선 위선 (⟋ ⟍), ALL: 중심선 + 아래선 + 위선 (⟋ ⟍) |
| Crooked   | 꺾인 대각선. Slash/BackSlash의 가운데 대각선이 꺾인 대각선임을 나타냄                                                                                                               |
| isCounter | slash/backSlash 대각선의 역방향 여부                                                                                                                                                |

#### 9.3.3.2.3 BorderType

`<leftBorder>`, `<rightBorder>`, `<topBorder>`, `<bottomBorder>`, `<diagonal>`은 모두 같은 형식을 가진다.

#### 그림 23 — `<BorderType>`의 구조

```
BorderType (테두리 형식)
└── Attributes
    ├── type (Type: LineType2, Default: SOLID) — 테두리선 종류
    ├── width (Type: LineWidth, Default: 0.12) — 테두리선 굵기. 단위는 mm.
    └── color (Type: RGBColorType, Default: #000000) — 테두리선 색상
```

#### 표 35 — BorderType 요소

| 속성 이름 | 설명                       |
| --------- | -------------------------- |
| type      | 테두리선의 종류            |
| width     | 테두리선의 굵기. 단위는 mm |
| color     | 테두리선의 색상            |

#### 샘플 13 — BorderType 예

```xml
<hh:borderFill id="4" threeD="0" shadow="0" centerLine="NONE" breakCellSeparateLine="0">
  <hh:slash type="NONE" Crooked="0" isCounter="0"/>
  <hh:backSlash type="NONE" Crooked="0" isCounter="0"/>
  <hh:leftBorder type="DOUBLE_SLIM" width="0.7 mm" color="#000000"/>
  <hh:rightBorder type="DOUBLE_SLIM" width="0.7 mm" color="#000000"/>
  <hh:topBorder type="DOUBLE_SLIM" width="0.7 mm" color="#000000"/>
  <hh:bottomBorder type="DOUBLE_SLIM" width="0.7 mm" color="#000000"/>
  <hh:diagonal type="SOLID" width="0.1 mm" color="#000000"/>
</hh:borderFill>
```

#### 9.3.3.2.4 fillBrush 요소

`<fillBrush>` 요소는 세 개의 하위 요소 중 하나의 요소를 가질 수 있다(choice). 즉, 채우기는 면 채우기/그라데이션/그림으로 채우기 중 하나의 형식만을 가져야 한다.

#### 그림 24 — `<fillBrush>`의 구조

```
fillBrush (Type: hc:FillBrushType)
├── winBrush — 면 채우기
├── gradation — 그라데이션 효과
└── imgBrush — 그림으로 채우기
```

#### 표 36 — fillBrush 요소

| 하위 요소 이름 | 설명            |
| -------------- | --------------- |
| winBrush       | 면 채우기       |
| gradation      | 그라데이션 효과 |
| imgBrush       | 그림으로 채우기 |

#### 샘플 14 — fillBrush 예

```xml
<hh:fillBrush>
  <hc:winBrush faceColor="none" hatchColor="#999999" alpha="0"/>
</hh:fillBrush>
```

### winBrush 요소

면 채우기 정보를 표현하기 위한 요소이다. 면 채우기 정보에는 면 색, 무늬 색, 무늬 종류, 투명도 등이 있다. 만약 면 채우기가 무늬를 포함하지 않으면 무늬 색은 사용되지 않는다.

#### 그림 25 — `<winBrush>`의 구조

```
winBrush (면 채우기)
└── Attributes
    ├── faceColor (Type: hc:RGBColorType, Default: #FFFFFF) — 면 색
    ├── hatchColor (Type: hc:RGBColorType, Default: #000000) — 무늬 색
    ├── hatchStyle (Type: restriction of xs:string) — 무늬 종류
    └── alpha (Type: xs:float) — 투명도
```

#### 표 37 — winBrush 요소

| 속성 이름  | 설명      |
| ---------- | --------- |
| faceColor  | 면 색     |
| hatchColor | 무늬 색   |
| hatchStyle | 무늬 종류 |
| alpha      | 투명도    |

#### 샘플 15 — winBrush 예

```xml
<hh:fillBrush>
  <hc:winBrush faceColor="#FFD700" hatchColor="#B2B2B2" hatchStyle="VERTICAL" alpha="0"/>
</hh:fillBrush>
```

### gradation 요소

`<gradation>` 요소는 한 색상에서 다른 색상으로 점진적 또는 단계적으로 변화하는 기법을 표현하기 위한 요소이다.

#### 그림 26 — `<gradation>`의 구조

````
gradation (그라데이션 효과)
├── Attributes
│   ├── type (Type: restriction of xs:string) — 그라데이션 유형
│   ├── angle (Type: xs:integer, Default: 90) — 그라데이션의 기울임(시작각)
│   ├── centerX (Type: xs:integer, Default: 0) — 그라데이션의 가로중심(중심 X 좌표)
│   ├── centerY (Type: xs:integer, Default: 0) — 그라데이션의 세로중심(중심 Y 좌표)
│   ├── step (Type: restriction of xs:integer, Default: 50) — 그라데이션 번짐정도 (0~100)
│   ├── colorNum (Type: xs:nonNegativeInteger, Default: 2) — 그라데이션의 색수
│   ├── stepCenter (Type: restriction of xs:integer, Default: 50) — 그라데이션 번짐정도의 중심 (0~100)
│   └── alpha (Type: xs:float) — 투명도
└── color (0..∞) — 그라데이션 색
    └── Attributes


#### 표 38 — gradation 요소

| 속성 이름  | 설명                                |
| ---------- | ----------------------------------- |
| type       | 그라데이션 유형                     |
| angle      | 그라데이션의 기울임(시작 각)        |
| centerX    | 그라데이션의 가로 중심(중심 X 좌표) |
| centerY    | 그라데이션의 세로 중심(중심 Y 좌표) |
| step       | 그라데이션의 번짐 정도              |
| colorNum   | 그라데이션의 색 수                  |
| stepCenter | 그라데이션 번짐 정도의 중심         |
| alpha      | 투명도                              |

#### 표 39 — gradation 하위 요소

| 하위 요소 이름 | 설명               |
| -------------- | ------------------ |
| Color          | 그라데이션 색 정보 |

#### 샘플 16 — gradation 예

```xml
<hh:fillBrush>
  <hh:gradation type="SQUARE" angle="0" centerX="50" centerY="0" step="255" colorNum="2"
    stepCenter="50" alpha="0">
    <hc:color value="#6182D6"/>
    <hc:color value="#FFFFFF"/>
  </hc:gradation>
</hh:fillBrush>
````

### color 요소

그라데이션 색상으로 표현하기 위한 요소로, 점진적으로 또는 단계적으로 변화하는 색상 중 시작 색, 또는 끝 색, 중간 단계 색 등을 표현한다.

#### 그림 27 — `<color>`의 구조

```
color (그라데이션 색)
└── Attributes
    └── value (Type: hc:RGBColorType) — 색 값
```

#### 표 40 — color 요소

| 속성 이름 | 설명  |
| --------- | ----- |
| value     | 색 값 |

### imgBrush 요소

그림으로 특정 부분을 채울 때 사용되는 요소로, 지정된 그림을 지정된 효과를 사용해서 채운다. 사용할 수 있는 효과에는 '크기에 맞추어', '위로/가운데로/아래로', '바둑판식으로' 등이 있다.

#### 그림 28 — `<imgBrush>`의 구조

```
imgBrush (그림으로 채우기)
├── Attributes
│   └── mode (Type: restriction of xs:string, Default: TILE) — 채우기 유형
└── img (Type: hc:imageType)
```

#### 표 41 — imgBrush 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| mode      | 채우기 유형 |

#### 표 42 — imgBrush 하위 요소

| 하위 요소 이름 | 설명      |
| -------------- | --------- |
| img            | 그림 정보 |

#### 샘플 17 — imgBrush 예

```xml
<hh:fillBrush>
  <hc:imgBrush mode="TOTAL">
    <hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
  </hc:imgBrush>
</hh:fillBrush>
```

### img 요소

그림 정보를 표현하기 위한 요소이다. 그림 데이터에 대한 참조 아이디 및 그림에 적용될 몇몇 효과들에 대한 정보를 포함한다.

#### 그림 29 — `<img>`의 구조

```
img (Type: hc:imageType)
└── Attributes
    ├── binaryItemIDRef — manifest의 item 엘리먼트의 아이디 참조 값
    ├── bright — 밝기
    ├── contrast — 명암
    ├── effect — 그림 효과
    └── alpha — 투명도
```

#### 표 43 — img 요소

| 속성 이름       | 설명                                                                                          |
| --------------- | --------------------------------------------------------------------------------------------- |
| bright          | 그림의 밝기                                                                                   |
| contrast        | 그림의 명암                                                                                   |
| effect          | 그림의 추가 효과. REAL_PIC: 원래 그림대로, GRAY_SCALE: 그레이 스케일로, BLACK_WHITE: 흑백으로 |
| binaryItemIDRef | BinDataItem 요소의 아이디 참조값. 그림의 바이너리 데이터에 대한 연결 정보                     |
| alpha           | 투명도                                                                                        |

#### 샘플 18 — img 예

```xml
<hc:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
```

## 9.3.4 charProperties 요소

### 9.3.4.1 charProperties

콘텐츠 내에서 글자 모양 정보는 반드시 한 개 이상 정의되어 있어야 한다. 내용이 없는 콘텐츠라도 기본 글자 모양 정보는 정의되어 있어야 한다. 헤더 스키마 상에서는 속성 @itemCnt의 값으로 올 수 있는 범위가 1 이상으로(positiveInteger) 제한되어 있으며, 자식 요소인 `<charPr>` 요소의 개수 한정자 역시 1 이상으로 정의되어 있다.

#### 그림 30 — `<charProperties>`의 구조

```
charProperties (글자 모양 정보)
├── Attributes
│   └── itemCnt
└── charPr (1..∞, Type: CharShapeType)
```

#### 표 44 — charProperties 요소

| 속성 이름 | 설명                  |
| --------- | --------------------- |
| itemCnt   | 글자 모양 정보의 개수 |

#### 표 45 — charProperties 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| charPr         | 글자 모양 정보 |

#### 샘플 19 — charProperties 예

```xml
<hh:charProperties itemCnt="11">
  <hh:charPr id="0" height="1000" textColor="#000000" shadeColor="none" useFontSpace="0" useKerning="0"
    symMark="NONE" borderFillIDRef="2">
    <hh:fontRef hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
    <hh:ratio hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100" user="100"/>
    <hh:spacing hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
    <hh:relSz hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100" user="100"/>
    <hh:offset hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
  </hh:charPr>
</hh:charProperties>
```

### 9.3.4.2 charPr 요소

#### 9.3.4.2.1 글자 모양

글자 모양 설정 정보를 표현하기 위한 요소이다.

#### 그림 31 — `<charPr>`의 구조

```
charPr (Type: CharShapeType)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 글자 모양 아이디
│   ├── height (Type: xs:integer, Default: 1000) — 글자 크기 (hwpunit 단위, 10 pt = 1000 hwpunit)
│   ├── textColor (Type: hc:RGBColorType, Default: #000000) — 글자색
│   ├── shadeColor (Type: hc:RGBColorType, Default: #FFFFFF) — 음영 색
│   ├── useFontSpace (Type: xs:boolean, Default: false) — 글꼴에 어울리는 빈칸
│   ├── useKerning (Type: xs:boolean, Default: false) — 커닝
│   ├── symMark (Type: Restriction of xs:string, Default: NONE) — 강조점 종류
│   └── borderFillIDRef (Type: xs:nonNegativeInteger) — 글자테두리 기능
└── Elements
    ├── fontRef — 언어별 글꼴 참조
    ├── ratio — 언어별 장평 (단위는 %)
    ├── spacing — 언어별 자간 (단위는 %)
    ├── relSz — 언어별 상대 크기 (단위는 %)
    ├── offset — 언어별 오프셋 (단위는 %)
    ├── italic — 글자 속성: 기울임
    ├── bold — 글자 속성: 진하게
    ├── underline — 글자 속성: 밑줄
    ├── strikeout — 글자 속성: 취소선
    ├── outline — 글자 속성: 외곽선
    ├── shadow — 글자 속성: 그림자
    ├── emboss — 글자 속성: 양각
    ├── engrave — 글자 속성: 음각
    ├── supscript — 글자 속성: 위첨자
    └── subscript — 글자 속성: 아래첨자
```

#### 표 46 — charPr 요소

| 속성 이름       | 설명                                                                                                                    |
| --------------- | ----------------------------------------------------------------------------------------------------------------------- |
| id              | 글자 모양 정보를 구별하기 위한 아이디                                                                                   |
| height          | 글자 크기. 단위는 HWPUNIT                                                                                               |
| textColor       | 글자 색                                                                                                                 |
| shadeColor      | 음영 색                                                                                                                 |
| useFontSpace    | 글꼴에 어울리는 빈칸을 사용할지 여부                                                                                    |
| useKerning      | 커닝 사용 여부                                                                                                          |
| symMark         | 강조점 종류                                                                                                             |
| borderFillIDRef | 글자 테두리 기능. 만약 글자 테두리를 사용한다면 해당 속성이 존재하고, 속성의 값은 테두리/채우기 정보의 아이디 참조이다. |

#### 표 47 — charPr 하위 요소

| 하위 요소 이름 | 설명                                                                                                           |
| -------------- | -------------------------------------------------------------------------------------------------------------- |
| fontRef        | 언어별 글꼴. 각 글꼴 타입에 맞게(한글이면 한글 글꼴 타입), 참조하는 글꼴 ID를 언어별로 기술                    |
| ratio          | 언어별 장평. 단위는 %                                                                                          |
| spacing        | 언어별 자간. 단위는 %                                                                                          |
| relSz          | 언어별 글자의 상대 크기. 단위는 %                                                                              |
| offset         | 언어별 오프셋. 단위는 %                                                                                        |
| italic         | 글자 속성: 기울임. 해당 요소가 존재하면 기울임 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략     |
| bold           | 글자 속성: 진하게. 해당 요소가 존재하면 진하게 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략     |
| underline      | 글자 속성: 밑줄                                                                                                |
| strikeout      | 글자 속성: 취소선                                                                                              |
| outline        | 글자 속성: 외곽선                                                                                              |
| shadow         | 글자 속성: 그림자. 해당 요소가 존재하면 그림자 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략     |
| emboss         | 글자 속성: 양각. 해당 요소가 존재하면 양각 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략         |
| engrave        | 글자 속성: 음각. 해당 요소가 존재하면 음각 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략         |
| supscript      | 글자 속성: 위첨자. 해당 요소가 존재하면 위첨자 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략     |
| subscript      | 글자 속성: 아래첨자. 해당 요소가 존재하면 아래첨자 글자 속성이 지정된 것이다. 해당 요소의 추가적인 설명은 생략 |

속성 @symMark는 글자 속성 중 강조점을 나타내기 위한 속성이다. 속성 @symMark에 사용할 수 있는 값은 다음과 같다. 해당되는 값은 유니코드의 값으로 표현한다.

#### 표 48 — symMark 유니코드 값

| 속성 값    | 유니코드 값 | 속성 값      | 유니코드 값 |
| ---------- | ----------- | ------------ | ----------- |
| NONE       | 없음        | GRAVE_ACCENT | 0x0300      |
| DOT_ABOVE  | 0x0307      | ACUTE_ACCENT | 0x0301      |
| RING_ABOVE | 0x030A      | CIRCUMFLEX   | 0x0302      |
| TILDE      | 0x030C      | MACRON       | 0x0304      |
| CARON      | 0x0303      | HOOK_ABOVE   | 0x0309      |
| SIDE       | 0x302E      | DOT_BELOW    | 0x0323      |
| COLON      | 0x302F      |              |             |

#### 9.3.4.2.2 fontRef 요소

각 언어별 글자에서 참조하는 글꼴들에 대한 정보를 가지고 있는 요소이다.

#### 그림 32 — `<fontRef>`의 구조

```
fontRef (언어별 글꼴 참조)
└── Attributes
    ├── hangul (Type: xs:nonNegativeInteger) — 한글
    ├── latin (Type: xs:nonNegativeInteger) — 라틴
    ├── hanja (Type: xs:nonNegativeInteger) — 한자
    ├── japanese (Type: xs:nonNegativeInteger) — 일본어
    ├── other (Type: xs:nonNegativeInteger) — 기타
    ├── symbol (Type: xs:nonNegativeInteger) — 심볼
    └── user (Type: xs:nonNegativeInteger) — 사용자
```

#### 표 49 — fontRef 요소

| 속성 이름 | 설명                                        |
| --------- | ------------------------------------------- |
| hangul    | 한글 글자에서 사용될 글꼴의 아이디 참조값   |
| latin     | 라틴 글자에서 사용될 글꼴의 아이디 참조값   |
| hanja     | 한자 글자에서 사용될 글꼴의 아이디 참조값   |
| japanese  | 일본어 글자에서 사용될 글꼴의 아이디 참조값 |
| other     | 기타 글자에서 사용될 글꼴의 아이디 참조값   |
| symbol    | 심볼 글자에서 사용될 글꼴의 아이디 참조값   |
| user      | 사용자 글자에서 사용될 글꼴의 아이디 참조값 |

#### 샘플 20 — fontRef

```xml
<hh:fontRef hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
```

#### 9.3.4.2.3 ratio 요소

각 언어별로 글자 장평 설정을 가지고 있는 요소이다. 글자가 시작되는 부분을 기준으로 장평을 적용한다. 즉, 글자 방향이 가로쓰기인 경우 글자의 왼쪽 시작되는 부분이 기준이다.

#### 그림 33 — `<ratio>`의 구조

```
ratio
└── Attributes
    ├── hangul (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── latin (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── hanja (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── japanese (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── other (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── symbol (Type: Restriction of xs:positiveInteger, Default: 100)
    └── user (Type: Restriction of xs:positiveInteger, Default: 100)
```

#### 표 50 — ratio 요소

| 속성 이름 | 설명                         |
| --------- | ---------------------------- |
| hangul    | 한글 글자의 장평. 단위는 %   |
| latin     | 라틴 글자의 장평. 단위는 %   |
| hanja     | 한자 글자의 장평. 단위는 %   |
| japanese  | 일본어 글자의 장평. 단위는 % |
| other     | 기타 글자의 장평. 단위는 %   |
| symbol    | 심볼 글자의 장평. 단위는 %   |
| user      | 사용자 글자의 장평. 단위는 % |

#### 샘플 21 — ratio 예

```xml
<hh:ratio hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100"
  user="100"/>
```

#### 9.3.4.2.4 spacing 요소

각 언어별로 글자 자간 설정을 가지고 있는 요소이다. 자간은 글자 사이의 간격이기 때문에 한 글자가 끝나는 부분을 기준으로 자간을 적용해야 한다.

#### 그림 34 — `<spacing>`의 구조

```
spacing
└── Attributes
    ├── hangul (Type: Restriction of xs:integer, Default: 0)
    ├── latin (Type: Restriction of xs:integer, Default: 0)
    ├── hanja (Type: Restriction of xs:integer, Default: 0)
    ├── japanese (Type: Restriction of xs:integer, Default: 0)
    ├── other (Type: Restriction of xs:integer, Default: 0)
    ├── symbol (Type: Restriction of xs:integer, Default: 0)
    └── user (Type: Restriction of xs:integer, Default: 0)
```

#### 표 51 — spacing 요소

| 속성 이름 | 설명                         |
| --------- | ---------------------------- |
| hangul    | 한글 글자의 자간. 단위는 %   |
| latin     | 라틴 글자의 자간. 단위는 %   |
| hanja     | 한자 글자의 자간. 단위는 %   |
| japanese  | 일본어 글자의 자간. 단위는 % |
| other     | 기타 글자의 자간. 단위는 %   |
| symbol    | 심볼 글자의 자간. 단위는 %   |
| user      | 사용자 글자의 자간. 단위는 % |

#### 샘플 22 — spacing 예

```xml
<hh:spacing hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
```

#### 9.3.4.2.5 relSz 요소

각 언어별로 글자의 상대 크기 설정 정보를 가지고 있는 요소이다.

#### 그림 35 — `<relSz>`의 구조

```
relSz
└── Attributes
    ├── hangul (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── latin (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── hanja (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── japanese (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── other (Type: Restriction of xs:positiveInteger, Default: 100)
    ├── symbol (Type: Restriction of xs:positiveInteger, Default: 100)
    └── user (Type: Restriction of xs:positiveInteger, Default: 100)
```

#### 표 52 — relSz 요소

| 속성 이름 | 설명                              |
| --------- | --------------------------------- |
| hangul    | 한글 글자의 상대 크기. 단위는 %   |
| latin     | 라틴 글자의 상대 크기. 단위는 %   |
| hanja     | 한자 글자의 상대 크기. 단위는 %   |
| japanese  | 일본어 글자의 상대 크기. 단위는 % |
| other     | 기타 글자의 상대 크기. 단위는 %   |
| symbol    | 심볼 글자의 상대 크기. 단위는 %   |
| user      | 사용자 글자의 상대 크기. 단위는 % |

#### 샘플 23 — relSz 예

```xml
<hh:relSz hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100"
  user="100"/>
```

#### 9.3.4.2.6 offset 요소

각 언어별로 글자의 위치 정보를 가지고 있는 요소이다. 글자 하단 끝부분을 기준으로 위치가 결정된다.

#### 그림 36 — `<offset>`의 구조

```
offset (언어별 오프셋. 단위는 %)
└── Attributes
    ├── hangul (Type: restriction of xs:integer, Default: 0)
    ├── latin (Type: restriction of xs:integer, Default: 0)
    ├── hanja (Type: restriction of xs:integer, Default: 0)
    ├── japanese (Type: restriction of xs:integer, Default: 0)
    ├── other (Type: restriction of xs:integer, Default: 0)
    ├── symbol (Type: restriction of xs:integer, Default: 0)
    └── user (Type: restriction of xs:integer, Default: 0)
```

#### 표 53 — offset 요소

| 속성 이름 | 설명                           |
| --------- | ------------------------------ |
| hangul    | 한글 글자의 오프셋. 단위는 %   |
| latin     | 라틴 글자의 오프셋. 단위는 %   |
| hanja     | 한자 글자의 오프셋. 단위는 %   |
| japanese  | 일본어 글자의 오프셋. 단위는 % |
| other     | 기타 글자의 오프셋. 단위는 %   |
| symbol    | 심볼 글자의 오프셋. 단위는 %   |
| user      | 사용자 글자의 오프셋. 단위는 % |

#### 샘플 24 — offset 예

```xml
<hh:offset hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
```

#### 9.3.4.2.7 underline 요소

글자 속성 중 밑줄을 표현하기 위한 요소이다. 이 요소가 존재하면 글자 속성 중 밑줄 속성이 지정된 것이다. 이 요소는 밑줄 속성을 표현하기 위한 몇몇 정보를 가지고 있다.

#### 그림 37 — `<underline>`의 구조

```
underline (글자 속성: 밑줄)
└── Attributes
    ├── type (Type: restriction of xs:string) — 밑줄 종류
    ├── shape (Type: hc:LineType2) — 밑줄 모양
    └── color (Type: hc:RGBColorType) — 밑줄 색
```

#### 표 54 — underline 요소

| 속성 이름 | 설명                                                                                                                                                             |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| type      | 밑줄의 종류. BOTTOM, CENTER, TOP 세 가지 값 중 하나를 가질 수 있음. 현재 CENTER 값은 `<strikeout>`으로 대체되어서 사용되고 있지 않음. 하위 호환성을 위해 남겨둠. |
| shape     | 밑줄의 모양                                                                                                                                                      |
| color     | 밑줄의 색                                                                                                                                                        |

#### 샘플 25 — underline 예

```xml
<hh:underline type="BOTTOM" shape="DOUBLE_SLIM" color="#B2B2B2"/>
```

#### 9.3.4.2.8 strikeout 요소

글자 속성 중 취소선을 표현하기 위한 요소이다. 취소선의 위치는 글자의 가운데가 기준이 된다. 이 요소가 존재하면 글자 속성 중 취소선 속성이 지정된 것이다. 이 요소는 취소선 속성을 표현하기 위한 몇몇 정보를 가지고 있다.

#### 그림 38 — `<strikeout>`의 구조

```
strikeout (글자 속성: 취소선)
└── Attributes
    ├── shape (Type: hc:LineType2) — 취소선 모양
    └── color (Type: hc:RGBColorType) — 취소선 색
```

#### 표 55 — strikeout 요소

| 속성 이름 | 설명          |
| --------- | ------------- |
| shape     | 취소선의 모양 |
| color     | 취소선의 색   |

#### 샘플 26 — strikeout 예

```xml
<hh:strikeout shape="LONG_DASH" color="#FFD700"/>
```

#### 9.3.4.2.9 outline 요소

글자 속성 중 외곽선을 표현하기 위한 요소이다. 외곽선은 글자가 들어가 있는 박스에 선이 들어가는 것이다. 이 요소가 존재하면 글자 속성 중 외곽선 속성이 지정된 것이다. 본 요소는 외곽선 속성을 표현하기 위한 몇몇 정보를 가지고 있다.

#### 그림 39 — `<outline>`의 구조

```
outline (글자 속성: 외곽선)
└── Attributes
    └── type (Type: hc:LineType1) — 외곽선 종류
```

#### 표 56 — outline 요소

| 속성 이름 | 설명          |
| --------- | ------------- |
| type      | 외곽선의 종류 |

#### 샘플 27 — outline 예

```xml
<hh:outline type="DASH"/>
```

#### 9.3.4.2.10 shadow 요소

글자 속성 중 그림자을 표현하기 위한 요소이다. 이 요소가 존재하면 글자 속성 중 그림자 속성이 지정된 것이다. 이 요소는 그림자 속성을 표현하기 위한 몇몇 정보를 가지고 있다.

#### 그림 40 — `<shadow>`의 구조

```
shadow (글자 속성: 그림자)
└── Attributes
    ├── type (Type: Restriction of xs:string) — 그림자 종류
    ├── color (Type: hc:RGBColorType) — 그림자 색
    ├── offsetX (Type: Restriction of xs:integer) — 그림자 간격 X. 단위는 %
    └── offsetY (Type: Restriction of xs:integer) — 그림자 간격 Y. 단위는 %
```

#### 표 57 — shadow 요소

| 속성 이름 | 설명 |
| --------- | ---- |

| 속성 이름 | 설명                    |
| --------- | ----------------------- |
| offsetX   | 그림자 간격 X. 단위는 % |
| offsetY   | 그림자 간격 Y. 단위는 % |

#### 샘플 28 — shadow 예

```xml
<hh:shadow type="CONTINUOUS" color="#9D5CBB" offsetX="15" offsetY="10"/>
```

### 9.3.5 tabProperties 요소

#### 9.3.5.1 탭 정보

탭 정보 목록을 가지고 있는 요소이다.

#### 그림 41 — `<tabProperties>`의 구조

```
tabProperties (탭 정보 목록)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 탭 정보의 개수
└── tabPr (0..∞) (Type: TabDefType) — 탭 정보
```

#### 표 58 — tabProperties 요소

| 속성 이름 | 설명           |
| --------- | -------------- |
| itemCnt   | 탭 정보의 개수 |

#### 표 59 — tabProperties 하위 요소

| 하위 요소 이름 | 설명    |
| -------------- | ------- |
| tabPr          | 탭 정보 |

#### 샘플 29 — tabProperties 예

```xml
<hh:tabProperties itemCnt="3">
  <hh:tabPr id="0" autoTabLeft="0" autoTabRight="0"/>
  <hh:tabPr id="1" autoTabLeft="1" autoTabRight="0"/>
  <hh:tabPr id="2" autoTabLeft="0" autoTabRight="1"/>
  <hh:tabPr id="3" autoTabLeft="0" autoTabRight="0">
    <hh:tabItem pos="32992" type="CENTER" leader="CIRCLE"/>
  </hh:tabPr>
</hh:tabProperties>
```

#### 9.3.5.2 tabPr 요소

##### 9.3.5.2.1 tabPr

탭(Tab) 정보는 한꺼번에 일정한 거리로 본문을 띄울 때 사용하는 요소이다. 탭은 여러 개의 항목을 세로로 가지런히 나열해 입력할 때에도 사용할 수 있다.

#### 그림 42 — `<tabPr>`의 구조

```
tabPr (탭 정보)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 탭 정보를 구별하기 위한 아이디
│   ├── autoTabLeft (Type: xs:boolean, Default: false) — 문단 왼쪽 끝 자동 탭 여부 (내어쓰기용 자동 탭)
│   └── autoTabRight (Type: xs:boolean, Default: false) — 문단 오른쪽 끝 자동 탭 여부
└── tabItem (0..∞) — 탭 정의 정보
```

#### 표 60 — tabPr 요소

| 속성 이름    | 설명                                           |
| ------------ | ---------------------------------------------- |
| id           | 탭 정보를 구별하기 위한 아이디                 |
| autoTabLeft  | 문단 왼쪽 끝 자동 탭 여부 (내어쓰기용 자동 탭) |
| autoTabRight | 문단 오른쪽 끝 자동 탭 여부                    |

#### 표 61 — tabPr 하위 요소

| 하위 요소 이름 | 설명         |
| -------------- | ------------ |
| tabItem        | 탭 정의 정보 |

##### 9.3.5.2.2 tabItem 요소

탭의 모양 및 위치 정보 등을 표현하기 위한 요소이다.

#### 그림 43 — `<tabItem>`의 구조

```
tabItem (탭 정의)
└── Attributes
    ├── pos (Type: xs:integer) — 탭의 위치. 단위는 HWPUNIT.
    ├── type (Type: restriction of 'xs:string') — 탭의 종류
    └── leader (Type: hc:LineType2) — 채움 종류
```

#### 표 62 — tabItem 요소

| 속성 이름 | 설명                                                                  |
| --------- | --------------------------------------------------------------------- |
| pos       | 탭의 위치. 단위는 HWPUNIT                                             |
| type      | 탭의 종류. LEFT: 왼쪽, RIGHT: 오른쪽, CENTER: 가운데, DECIMAL: 소수점 |
| leader    | 탭 채움 종류                                                          |

#### 샘플 30 — tabItem 예

```xml
<hh:tabItem pos="32992" type="CENTER" leader="CIRCLE"/>
```

### 9.3.6 numberings 요소

문단 번호 모양 정보 목록을 가지고 있는 요소이다.

#### 그림 44 — `<numberings>`의 구조

```
numberings (번호 매긴 목록)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 문단 번호 모양 정보의 개수
└── numbering (0..∞) (Type: NumberingType) — 문단 번호 정보
```

#### 표 63 — numberings 요소

| 속성 이름 | 설명                       |
| --------- | -------------------------- |
| itemCnt   | 문단 번호 모양 정보의 개수 |

#### 표 64 — numberings 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| numbering      | 문단 번호 정보 |

#### 샘플 31 — numberings 예

```xml
<hh:numberings itemCnt="1">
  <hh:numbering id="1" start="0">
    <hh:paraHead start="1" level="1" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295"
      checkable="0">^1.</hh:paraHead>
    <hh:paraHead start="1" level="2" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE" charPrIDRef="4294967295"
      checkable="0">^2.</hh:paraHead>
    <hh:paraHead start="1" level="3" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295"
      checkable="0">^3)</hh:paraHead>
    <hh:paraHead start="1" level="4" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE" charPrIDRef="4294967295"
      checkable="0">^4)</hh:paraHead>
    <hh:paraHead start="1" level="5" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295"
      checkable="0">(^5)</hh:paraHead>
    <hh:paraHead start="1" level="6" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="HANGUL_SYLLABLE" charPrIDRef="4294967295"
      checkable="0">(^6)</hh:paraHead>
    <hh:paraHead start="1" level="7" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="CIRCLED_DIGIT" charPrIDRef="4294967295"
      checkable="1">^7</hh:paraHead>
    <hh:paraHead start="1" level="8" align="LEFT" useInstWidth="0" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295" checkable="0"/>
    <hh:paraHead start="1" level="9" align="LEFT" useInstWidth="0" autoIndent="1" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295" checkable="0"/>
    <hh:paraHead start="1" level="10" align="LEFT" useInstWidth="0" autoIndent="0" widthAdjust="0"
      textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295" checkable="0"/>
  </hh:numbering>
</hh:numberings>
```

#### 9.3.6.1 numbering 요소

##### 9.3.6.1.1 numbering 일반 항목

여러 개의 항목을 나열할 때 문단의 머리에 번호를 매기거나 글머리표, 그림 글머리표를 붙일 수 있다. 문단 번호는 7 수준까지 다단계 번호를 매겨 주고, 문단 번호를 사용한 문장의 순서가 바뀌면 문단 번호도 그에 맞게 자동으로 바뀌어야 한다.

#### 그림 45 — `<numbering>`의 구조

```
numbering (문단 번호 정보)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 번호 문단 모양을 구별하기 위한 아이디
│   └── start (Type: xs:integer, Default: 1) — 번호 문단에서 시작되는 숫자 번호
└── paraHead (1..∞) (Type: ParaHeadType) — 번호/글머리표 문단 머리의 정보
```

#### 표 65 — numbering 요소

| 속성 이름 | 설명                                  |
| --------- | ------------------------------------- |
| id        | 번호 문단 모양을 구별하기 위한 아이디 |
| start     | 번호 문단에서 시작되는 숫자 번호      |

#### 표 66 — numbering 하위 요소

| 하위 요소 이름 | 설명                           |
| -------------- | ------------------------------ |
| paraHead       | 번호/글머리표 문단 머리의 정보 |

##### 9.3.6.1.2 paraHead 요소

각 번호/글머리표 문단 머리의 정보이다. 문자열 내 특정 문자에 제어코드(^ 0x005E)를 붙임으로써 한글 워드프로세서에서 표시되는 번호/글머리표 문단 머리의 포맷을 제어한다.

- ^n: 레벨 경로를 표시한다(예: 1.1.1.1.1).
- ^N: 레벨 경로를 표시하며 마지막에 마침표를 하나 더 찍는다(예: 1.1.1.1.1.).
- ^레벨번호(1 ~ 7): 해당 레벨에 해당하는 숫자 또는 문자 또는 기호를 표시한다.

#### 그림 46 — `<paraHead>`의 구조

```
paraHead (번호/글머리표 문단 머리 정보)
├── Attributes
│   ├── start (Type: xs:unsignedInt, Default: 1) — 사용자 지정 문단 시작번호
│   ├── level (Type: Restriction of 'xs:positiveInteger') — 수준 (번호/글머리표의 수준)
│   ├── align (Type: Restriction of 'xs:string', Default: LEFT) — 번호의 정렬 종류
│   ├── useInstWidth (Type: xs:boolean, Default: true) — 번호 너비를 실제 인스턴스 문자열의 너비에 따를지 여부
│   ├── autoIndent (Type: xs:boolean, Default: true) — 자동 내어 쓰기 여부
│   ├── widthAdjust (Type: xs:integer, Default: 0) — 번호 너비 보정 값. 단위는 HWPUNIT
│   ├── textOffsetType (Type: Restriction of 'xs:string', Default: PERCENT) — 수준별 본문과의 거리 단위 종류. 단위 종류: PERCENT, HWPUNIT
│   ├── textOffset (Type: xs:integer, Default: 50) — 수준별 본문과의 거리
│   ├── numFormat (Type: hc:NumberType1, Default: DIGIT) — 번호 형식(글머리표 문단의 경우에는 사용되지 않음)
│   ├── charPrIDRef (Type: xs:nonNegativeInteger) — 글자 모양 아이디 참조
│   └── checkable (Type: xs:boolean) — 확인용 글머리표 여부
└── (Mixed Content) — ^n 레벨 경로를 나타냄. 예: ^1은 첫 번째 레벨로, 1.1.1.1(^n). ^N은 레벨 경로 뒤에 점을 하나 더 찍는다. 0은 마침표 없음, 1.1.1.1.(^N) 등
```

#### 표 67 — paraHead 요소

| 속성 이름      | 설명                                                        |
| -------------- | ----------------------------------------------------------- |
| start          | 사용자 지정 문단 시작번호                                   |
| level          | 번호/글머리표의 수준                                        |
| align          | 문단의 정렬 종류. LEFT: 왼쪽, RIGHT: 오른쪽, CENTER: 가운데 |
| useInstWidth   | 번호 너비를 실제 인스턴스 문자열의 너비에 따를지 여부       |
| autoIndent     | 자동 내어 쓰기 여부                                         |
| widthAdjust    | 번호 너비 보정 값. 단위는 HWPUNIT                           |
| textOffsetType | 수준별 본문과의 거리 단위 종류. 단위 종류: PERCENT, HWPUNIT |
| textOffset     | 수준별 본문과의 거리                                        |
| numFormat      | 번호 형식(글머리표 문단의 경우에는 사용되지 않음)           |
| charPrIDRef    | 글자 모양 아이디 참조값                                     |
| checkable      | 확인용 글머리표 여부                                        |

#### 샘플 32 — paraHead 예

```xml
<hh:paraHead start="1" level="1" align="LEFT" useInstWidth="1" autoIndent="1" widthAdjust="0"
  textOffsetType="PERCENT" textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295"
  checkable="0">^1.</hh:paraHead>
```

### 9.3.7 bullets 요소

#### 9.3.7.1 bullets 일반 항목

글머리표 문단 모양 정보 목록을 가지고 있는 요소이다.

#### 그림 47 — `<bullets>`의 구조

```
bullets (글머리표 목록)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 글머리표 문단 모양 정보의 개수
└── bullet (0..∞) (Type: BulletType) — 글머리표 문단 모양 정보
```

#### 표 68 — bullets 요소

| 속성 이름   | 설명                           |
| ----------- | ------------------------------ |
| bulletCount | 글머리표 문단 모양 정보의 개수 |

#### 표 69 — bullets 하위 요소

| 하위 요소 이름 | 설명                    |
| -------------- | ----------------------- |
| bullet         | 글머리표 문단 모양 정보 |

#### 샘플 33 — bullets 예

```xml
<hh:bullets itemCnt="1">
  <hh:bullet id="1" char="λ" useImage="0">
    <hh:paraHead level="0" align="LEFT" useInstWidth="0" autoIndent="1" widthAdjust="0" textOffsetType="PERCENT"
      textOffset="50" numFormat="DIGIT" charPrIDRef="4294967295" checkable="0"/>
  </hh:bullet>
</hh:bullets>
```

#### 9.3.7.2 bullet 요소

글머리표 문단 모양 정보를 사용하면 문단의 머리에 번호 대신 글머리표 또는 그림 글머리표를 붙일 수 있다. 속성 @useImg의 값이 참(true)으로 설정되면 반드시 `<img>` 요소를 자식 요소로 가지고 있어야 한다. 즉, 글머리표로 사용되는 이미지에 대한 참조 정보를 가지고 있어야 한다.

#### 그림 48 — `<bullet>`의 구조

```
bullet (글머리표 문단 모양 정보)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 글머리표 문단 모양을 구별하기 위한 아이디
│   ├── char (Type: xs:string) — 글머리표 문자
│   ├── checkedChar (Type: xs:string) — 선택 글머리표 문자
│   └── useImg (Type: xs:boolean) — 글머리표 문자 대신 글머리표 그림을 사용할지 여부
├── img (Type: hc:imageType) — 글머리표 그림에 사용되는 그림에 대한 정보
└── paraHead (Type: ParaHeadType) — 번호/글머리표 문단 머리의 정보
```

#### 표 70 — bullet 요소

| 속성 이름   | 설명                                             |
| ----------- | ------------------------------------------------ |
| id          | 글머리표 문단 모양을 구별하기 위한 아이디        |
| char        | 글머리표 문자                                    |
| checkedChar | 선택 글머리표 문자                               |
| useImg      | 글머리표 문자 대신 글머리표 그림을 사용할지 여부 |

#### 표 71 — bullet 하위 요소

| 하위 요소 이름 | 설명                                      |
| -------------- | ----------------------------------------- |
| img            | 글머리표 그림에 사용되는 그림에 대한 정보 |
| paraHead       | 번호/글머리표 문단 머리의 정보            |

### 9.3.8 paraProperties 요소

#### 9.3.8.1 paraProperties 일반 항목

문단 모양 정보 목록을 가지고 있는 요소이다.

#### 그림 49 — `<paraProperties>`의 구조

```
paraProperties (문단 모양)
├── Attributes
│   └── itemCnt (Type: xs:positiveInteger) — 문단 모양 정보의 개수
└── paraPr (1..∞) (Type: ParaShapeType) — 문단 모양 정보
```

#### 표 72 — paraProperties 요소

| 속성 이름 | 설명                  |
| --------- | --------------------- |
| itemCnt   | 문단 모양 정보의 개수 |

#### 표 73 — paraProperties 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| paraPr         | 문단 모양 정보 |

#### 샘플 34 — paraProperties 예

```xml
<hh:paraProperties itemCnt="21">
  <hh:paraPr id="0" tabPrIDRef="0" condense="0" fontLineHeight="0" snapToGrid="1" suppressLineNumbers="0"
    checked="0" textDir="LTR">
    <hh:align horizontal="JUSTIFY" vertical="BASELINE"/>
    <hh:heading type="NONE" idRef="0" level="0"/>
    <hh:breakSetting breakLatinWord="KEEP_WORD" breakNonLatinWord="KEEP_WORD" widowOrphan="0"
      keepWithNext="0" keepLines="0" pageBreakBefore="0" lineWrap="BREAK"/>
    <hh:autoSpacing eAsianEng="0" eAsianNum="0"/>
    <hh:margin>
      <hh:intent value="0" unit="HWPUNIT"/>
      <hh:left value="0" unit="HWPUNIT"/>
      <hh:right value="0" unit="HWPUNIT"/>
      <hh:prev value="0" unit="HWPUNIT"/>
      <hh:next value="0" unit="HWPUNIT"/>
    </hh:margin>
    <hh:lineSpacing type="PERCENT" value="160" unit="HWPUNIT"/>
    <hh:border borderFillIDRef="2" offsetLeft="0" offsetRight="0" offsetTop="0" offsetBottom="0" connect="0"
      ignoreMargin="0"/>
  </hh:paraPr>
</hh:paraProperties>
```

#### 9.3.8.2 paraPr 요소

##### 9.3.8.2.1 paraPr 일반 항목

문단 모양 정보는 문단 내 정렬, 문단 테두리 등 문단을 표현할 때 필요한 각종 설정 정보를 가지고 있는 요소이다.

#### 그림 50 — `<paraPr>`의 구조

```
paraPr (문단 모양 정보)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 문단 모양 정보를 구별하기 위한 아이디
│   ├── tabPrIDRef (Type: xs:nonNegativeInteger) — 탭 정의 아이디 참조
│   ├── condense (Type: Restriction of 'xs:integer') — 공백 최소값. 단위는 %
│   ├── fontLineHeight (Type: xs:boolean, Default: false) — 글꼴에 어울리는 줄 높이 사용 여부
│   ├── snapToGrid (Type: xs:boolean, Default: true) — 편집 용지의 줄 격자 사용 여부
│   ├── suppressLineNumbers (Type: xs:boolean, Default: false) — 줄 번호 건너뜀 사용 여부
│   ├── checked (Type: xs:boolean, Default: false) — 선택 글머리표 여부
│   └── textDir (Type: Restriction of 'xs:string') — 문단 방향 정보. RTL: 오른쪽에서 왼쪽, LTR: 왼쪽에서 오른쪽
├── align — 문단 내 정렬 설정
├── heading — 문단 머리 번호/글머리표 설정
├── breakSetting — 문단 줄나눔 설정
├── margin — 문단 여백 설정
├── lineSpacing — 줄 간격 설정
├── border — 문단 테두리 설정
└── autoSpacing — 문단 자동 간격 조절 설정
```

#### 표 74 — paraPr 요소

| 속성 이름           | 설명                                                       |
| ------------------- | ---------------------------------------------------------- |
| id                  | 문단 모양 정보를 구별하기 위한 아이디                      |
| tabPrIDRef          | 탭 정의 아이디 참조값                                      |
| condense            | 공백 최소값. 단위는 %                                      |
| fontLineHeight      | 글꼴에 어울리는 줄 높이 사용 여부                          |
| snapToGrid          | 편집 용지의 줄 격자 사용 여부                              |
| suppressLineNumbers | 줄 번호 건너뜀 사용 여부                                   |
| checked             | 선택 글머리표 여부                                         |
| textDir             | 문단 방향 정보. RTL: 오른쪽에서 왼쪽, LTR: 왼쪽에서 오른쪽 |

#### 표 75 — paraPr 하위 요소

| 하위 요소 이름 | 설명                         |
| -------------- | ---------------------------- |
| align          | 문단 내 정렬 설정            |
| heading        | 문단 머리 번호/글머리표 설정 |
| breakSetting   | 문단 줄나눔 설정             |
| margin         | 문단 여백 설정               |
| lineSpacing    | 줄 간격 설정                 |
| border         | 문단 테두리 설정             |
| autoSpacing    | 문단 자동 간격 조절 설정     |

##### 9.3.8.2.2 align 요소

문단 내 정렬 방식을 표현하기 위한 요소이다.

#### 그림 51 — `<align>`의 구조

```
align (문단 내 정렬)
└── Attributes
    ├── horizontal (Type: restriction of 'xs:string') — 정렬 방식
    └── vertical (Type: restriction of 'xs:string') — 세로 정렬
```

#### 표 76 — align 요소

| 속성 이름  | 설명                                                                                                                                                            |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| horizontal | 가로 정렬 방식. JUSTIFY: 양쪽 정렬, LEFT: 왼쪽 정렬, RIGHT: 오른쪽 정렬, CENTER: 가운데 정렬, DISTRIBUTE: 배분 정렬, DISTRIBUTE_SPACE: 나눔 정렬(공백에만 배분) |
| vertical   | 세로 정렬 방식. BASELINE: 글꼴 기준, TOP: 위쪽, CENTER: 가운데, BOTTOM: 아래                                                                                    |

#### 샘플 35 — align 예

```xml
<hh:align horizontal="JUSTIFY" vertical="BASELINE"/>
```

##### 9.3.8.2.3 heading 요소

문단 머리 모양 설정 정보를 가지고 있는 요소이다.

#### 그림 52 — `<heading>`의 구조

```
heading (문단 머리 번호/글머리표)
└── Attributes
    ├── type (Type: Restriction of 'xs:string') — 문단 머리 모양 종류
    ├── idRef (Type: xs:nonNegativeInteger) — 번호/글머리표 문단 모양 아이디 참조
    └── level (Type: Restriction of 'xs:nonNegativeInteger') — 단계
```

#### 표 77 — heading 요소

| 속성 이름 | 설명 |
| --------- | ---- |

| 속성 이름 | 설명                                       |
| --------- | ------------------------------------------ |
| idRef     | 문단 머리 번호/글머리표 모양 아이디 참조값 |
| level     | 문단 단계                                  |

#### 샘플 36 — heading 예

```xml
<hh:heading type="NUMBER" idRef="2" level="0"/>
```

##### 9.3.8.2.4 breakSetting 요소

문단의 줄나눔 설정 정보를 가지고 있는 요소이다.

#### 그림 53 — `<breakSetting>`의 구조

```
breakSetting (문단 줄 나눔 설정)
└── Attributes
    ├── breakLatinWord (Type: restriction of 'xs:string') — 한글 문자의 줄 나눔 단위
    ├── breakNonLatinWord (Type: restriction of 'xs:string') — 한글 문자 이외의 문자의 줄나눔 단위
    ├── widowOrphan (Type: xs:boolean) — 외톨이줄 보호 여부
    ├── keepWithNext (Type: xs:boolean) — 다음 문단과 함께 여부
    ├── keepLines (Type: xs:boolean) — 문단 보호 여부
    ├── pageBreakBefore (Type: xs:boolean) — 문단 앞에서 항상 쪽 나눔 여부
    └── lineWrap (Type: restriction of 'xs:string') — 한 줄로 입력 사용 시의 형식
```

#### 표 78 — breakSetting 요소

| 속성 이름         | 설명                                |
| ----------------- | ----------------------------------- |
| breakLatinWord    | 라틴 문자의 나눔 단위               |
| breakNonLatinWord | 라틴 문자 이외의 문자의 줄나눔 단위 |
| widowOrphan       | 외톨이줄 보호 여부                  |
| keepWithNext      | 다음 문단과 함께 여부               |
| keepLines         | 문단 보호 여부                      |
| pageBreakBefore   | 문단 앞에서 항상 쪽 나눔 여부       |
| lineWrap          | 한 줄로 입력 사용 시의 형식         |

#### 샘플 37 — breakSetting 예

```xml
<hh:breakSetting breakLatinWord="KEEP_WORD" breakNonLatinWord="KEEP_WORD"
  widowOrphan="0" keepWithNext="0" keepLines="0" pageBreakBefore="0" lineWrap="BREAK"/>
```

##### 9.3.8.2.5 margin 요소

문단의 여백 정보를 가지고 있는 요소이다.

#### 그림 54 — `<margin>`의 구조

```
margin (문단 여백)
├── intent (Type: hc:HWPValue) — 들여쓰기/내어쓰기. n이 0보다 크면 들여쓰기 n, n이 0이면 보통, n이 0보다 작으면 내어쓰기 n.
├── left (Type: hc:HWPValue) — 왼쪽 여백. 단위 종류: hwpunit 또는 문자(문자로 표기할 경우 한 문자로)
├── right (Type: hc:HWPValue) — 오른쪽 여백
├── prev (Type: hc:HWPValue) — 문단 간격 위
└── next (Type: hc:HWPValue) — 문단 간격 아래
```

#### 표 79 — margin 요소

| 하위 요소 이름 | 설명                                                                                      |
| -------------- | ----------------------------------------------------------------------------------------- |
| intent         | 들여쓰기/내어쓰기. n이 0보다 크면 들여쓰기 n, n이 0이면 보통, n이 0보다 작으면 내어쓰기 n |
| left           | 왼쪽 여백                                                                                 |
| right          | 오른쪽 여백                                                                               |
| prev           | 위쪽 문단 간격                                                                            |
| next           | 아래쪽 문단 간격                                                                          |

#### 샘플 38 — margin 예

```xml
<hh:margin>
  <hh:intent value="0" unit="HWPUNIT"/>
  <hh:left value="0" unit="HWPUNIT"/>
  <hh:right value="0" unit="HWPUNIT"/>
  <hh:prev value="0" unit="HWPUNIT"/>
  <hh:next value="0" unit="HWPUNIT"/>
</hh:margin>
```

##### 9.3.8.2.6 lineSpacing 요소

문단의 줄 간격 설정 정보를 가지고 있는 요소이다.

#### 그림 55 — `<lineSpacing>`의 구조

```
lineSpacing (줄 간격)
└── Attributes
    ├── type (Type: restriction of 'xs:string') — 줄 간격 종류
    ├── value (Type: xs:integer) — 줄 간격 값. type이 PERCENT이면 0%~500%로 제한
    └── unit (Type: restriction of 'xs:string', Default: HWPUNIT) — 줄 간격 값의 단위
```

#### 표 80 — lineSpacing 요소

| 속성 이름 | 설명                                            |
| --------- | ----------------------------------------------- |
| type      | 줄 간격 종류                                    |
| value     | 줄 간격 값. type이 PERCENT이면 0% ~ 500%로 제한 |
| unit      | 줄 간격 값의 단위                               |

#### 샘플 39 — lineSpacing 예

```xml
<hh:lineSpacing type="PERCENT" value="160" unit="HWPUNIT"/>
```

##### 9.3.8.2.7 border 요소

문단의 테두리 설정 정보를 가지고 있는 요소이다.

#### 그림 56 — `<border>`의 구조

```
border (문단 테두리)
└── Attributes
    ├── borderFillIDRef (Type: xs:IDREF) — 테두리/배경 모양 아이디 참조
    ├── offsetLeft (Type: xs:integer, Default: 0) — 문단 테두리 왼쪽 간격. 단위는 HWPUNIT
    ├── offsetRight (Type: xs:integer, Default: 0) — 문단 테두리 오른쪽 간격. 단위는 HWPUNIT
    ├── offsetTop (Type: xs:integer, Default: 0) — 문단 테두리 위쪽 간격. 단위는 HWPUNIT
    ├── offsetBottom (Type: xs:integer, Default: 0) — 문단 테두리 아래쪽 간격. 단위는 HWPUNIT
    ├── connect (Type: xs:boolean, Default: false) — 문단 테두리 연결 여부
    └── ignoreMargin (Type: xs:boolean, Default: false) — 문단 테두리 여백 무시 여부
```

#### 표 81 — border 요소

| 속성 이름       | 설명                                    |
| --------------- | --------------------------------------- |
| borderFillIDRef | 테두리/배경 모양 아이디 참조값          |
| offsetLeft      | 문단 테두리 왼쪽 간격. 단위는 HWPUNIT   |
| offsetRight     | 문단 테두리 오른쪽 간격. 단위는 HWPUNIT |
| offsetTop       | 문단 테두리 위쪽 간격. 단위는 HWPUNIT   |
| offsetBottom    | 문단 테두리 아래쪽 간격. 단위는 HWPUNIT |
| connect         | 문단 테두리 연결 여부                   |
| ignoreMargin    | 문단 테두리 여백 무시 여부              |

#### 샘플 40 — border 예

```xml
<hh:border borderFillIDRef="2" offsetLeft="0" offsetRight="0" offsetTop="0" offsetBottom="0"
  connect="0" ignoreMargin="0"/>
```

##### 9.3.8.2.8 autoSpacing 요소

문단 내에서 한글, 영어, 숫자 사이의 간격에 대한 자동 조절 설정 정보를 가지고 있는 요소이다.

#### 그림 57 — `<autoSpacing>`의 구조

```
autoSpacing (문단 자동 간격)
└── Attributes
    ├── eAsianEng (Type: xs:boolean) — 한글과 영어 간격을 자동 조절 여부
    └── eAsianNum (Type: xs:boolean) — 한글과 숫자 간격을 자동 조절 여부
```

#### 표 82 — autoSpacing 요소

| 속성 이름 | 설명                              |
| --------- | --------------------------------- |
| eAsianEng | 한글과 영어 간격을 자동 조절 여부 |
| eAsianNum | 한글과 숫자 간격을 자동 조절 여부 |

#### 샘플 41 — autoSpacing 예

```xml
<hh:autoSpacing eAsianEng="0" eAsianNum="0"/>
```

### 9.3.9 styles 요소

#### 9.3.9.1 styles

스타일 정보 목록을 가지고 있는 요소이다.

#### 그림 58 — `<styles>`의 구조

```
styles (스타일)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 스타일 정보의 개수
└── style (0..∞) (Type: StyleType) — 스타일 정보
```

#### 표 83 — styles 요소

| 속성 이름 | 설명               |
| --------- | ------------------ |
| itemCnt   | 스타일 정보의 개수 |

#### 표 84 — styles 하위 요소

| 하위 요소 이름 | 설명        |
| -------------- | ----------- |
| style          | 스타일 정보 |

#### 샘플 42 — styles 예

```xml
<hh:styles itemCnt="21">
  <hh:style id="0" type="PARA" name="바탕글" engName="Normal" paraPrIDRef="0" charPrIDRef="6"
    nextStyleIDRef="0" langID="1042" lockForm="0"/>
  <hh:style id="1" type="PARA" name="본문" engName="Body" paraPrIDRef="1" charPrIDRef="6"
    nextStyleIDRef="1" langID="1042" lockForm="0"/>
  <hh:style id="2" type="PARA" name="개요 1" engName="Outline 1" paraPrIDRef="2" charPrIDRef="6"
    nextStyleIDRef="2" langID="1042" lockForm="0"/>
  <hh:style id="3" type="PARA" name="개요 2" engName="Outline 2" paraPrIDRef="3" charPrIDRef="6"
    nextStyleIDRef="3" langID="1042" lockForm="0"/>
  <hh:style id="4" type="PARA" name="개요 3" engName="Outline 3" paraPrIDRef="4" charPrIDRef="6"
    nextStyleIDRef="4" langID="1042" lockForm="0"/>
  <hh:style id="5" type="PARA" name="개요 4" engName="Outline 4" paraPrIDRef="5" charPrIDRef="6"
    nextStyleIDRef="5" langID="1042" lockForm="0"/>
  <hh:style id="6" type="PARA" name="개요 5" engName="Outline 5" paraPrIDRef="6" charPrIDRef="6"
    nextStyleIDRef="6" langID="1042" lockForm="0"/>
  <hh:style id="7" type="PARA" name="개요 6" engName="Outline 6" paraPrIDRef="7" charPrIDRef="6"
    nextStyleIDRef="7" langID="1042" lockForm="0"/>
  <hh:style id="8" type="PARA" name="개요 7" engName="Outline 7" paraPrIDRef="8" charPrIDRef="6"
    nextStyleIDRef="8" langID="1042" lockForm="0"/>
  <hh:style id="9" type="PARA" name="개요 8" engName="Outline 8" paraPrIDRef="18" charPrIDRef="6"
    nextStyleIDRef="9" langID="1042" lockForm="0"/>
  <hh:style id="10" type="PARA" name="개요 9" engName="Outline 9" paraPrIDRef="16" charPrIDRef="6"
    nextStyleIDRef="10" langID="1042" lockForm="0"/>
  <hh:style id="11" type="PARA" name="개요 10" engName="Outline 10" paraPrIDRef="17" charPrIDRef="6"
    nextStyleIDRef="11" langID="1042" lockForm="0"/>
  <hh:style id="12" type="CHAR" name="쪽 번호" engName="Page Number" paraPrIDRef="0" charPrIDRef="0"
    nextStyleIDRef="0" langID="1042" lockForm="0"/>
  <hh:style id="13" type="PARA" name="머리말" engName="Header" paraPrIDRef="9" charPrIDRef="1"
    nextStyleIDRef="13" langID="1042" lockForm="0"/>
  <hh:style id="14" type="PARA" name="각주" engName="Footnote" paraPrIDRef="10" charPrIDRef="2"
    nextStyleIDRef="14" langID="1042" lockForm="0"/>
  <hh:style id="15" type="PARA" name="미주" engName="Endnote" paraPrIDRef="10" charPrIDRef="2"
    nextStyleIDRef="15" langID="1042" lockForm="0"/>
  <hh:style id="16" type="PARA" name="메모" engName="Memo" paraPrIDRef="11" charPrIDRef="3"
    nextStyleIDRef="16" langID="1042" lockForm="0"/>
  <hh:style id="17" type="PARA" name="차례 제목" engName="TOC Heading" paraPrIDRef="12" charPrIDRef="4"
    nextStyleIDRef="17" langID="1042" lockForm="0"/>
  <hh:style id="18" type="PARA" name="차례 1" engName="TOC 1" paraPrIDRef="13" charPrIDRef="5"
    nextStyleIDRef="18" langID="1042" lockForm="0"/>
  <hh:style id="19" type="PARA" name="차례 2" engName="TOC 2" paraPrIDRef="14" charPrIDRef="5"
    nextStyleIDRef="19" langID="1042" lockForm="0"/>
  <hh:style id="20" type="PARA" name="차례 3" engName="TOC 3" paraPrIDRef="15" charPrIDRef="5"
    nextStyleIDRef="20" langID="1042" lockForm="0"/>
</hh:styles>
```

#### 9.3.9.2 style 요소

스타일은 자주 사용하는 글자 모양이나 문단 모양을 미리 정해 놓고서 이를 사용할 수 있게 해주는 기능이다. `<style>` 요소는 설정된 스타일 기능을 표현하기 위한 요소이다.

#### 그림 59 — `<style>`의 구조

```
style (스타일 정보)
└── Attributes
    ├── id (Type: xs:nonNegativeInteger) — 스타일 정보를 구별하기 위한 아이디
    ├── type (Type: Restriction of 'xs:string') — 스타일 종류
    ├── name (Type: xs:string) — 스타일의 로컬 이름. 한글 윈도에서는 한글 스타일 이름
    ├── engName (Type: xs:string) — 영문 스타일 이름
    ├── paraPrIDRef (Type: xs:nonNegativeInteger) — 문단 모양 아이디 참조. 스타일의 종류가 문단인 경우 반드시 지정해야 함
    ├── charPrIDRef (Type: xs:nonNegativeInteger) — 글자 모양 아이디 참조값. 스타일의 종류가 글자인 경우 반드시 지정해야 함
    ├── nextStyleIDRef (Type: xs:nonNegativeInteger) — 다음 스타일 아이디 참조값. 문단 스타일에서 사용자가 리턴 키를 입력하여 다음 문단으로 이동하였을 때 적용될 문단 스타일을 지정함
    ├── langID (Type: xs:unsignedShort) — 언어 아이디. http://www.w3.org/WAI/ER/IG/ert/iso639.htm 참조
    └── lockForm (Type: xs:boolean, Default: false) — 양식 모드에서 Style 보호하기 여부
```

#### 표 85 — style 요소

| 속성 이름      | 설명                                                                                                                           |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| id             | 스타일 정보를 구별하기 위한 아이디                                                                                             |
| type           | 스타일 종류. PARA: 문단 스타일, CHAR: 글자 스타일                                                                              |
| name           | 스타일의 로컬 이름. 한글 윈도에서는 한글 스타일 이름                                                                           |
| engName        | 스타일의 영문 이름                                                                                                             |
| paraPrIDRef    | 문단 모양 아이디 참조값. 스타일의 종류가 문단인 경우 반드시 지정해야 함                                                        |
| charPrIDRef    | 글자 모양 아이디 참조값. 스타일의 종류가 글자인 경우 반드시 지정해야 함                                                        |
| nextStyleIDRef | 다음 스타일 아이디 참조값. 문단 스타일에서 사용자가 리턴 키를 입력하여 다음 문단으로 이동하였을 때 적용될 문단 스타일을 지정함 |
| langID         | 언어 아이디. http://www.w3.org/WAI/ER/IG/ert/iso639.htm 참조                                                                   |
| lockForm       | 양식 모드에서 Style 보호하기 여부                                                                                              |

### 9.3.10 memoProperties 요소

#### 9.3.10.1 memoProperties

메모 모양 정보 목록을 가지고 있는 요소이다.

#### 그림 60 — `<memoProperties>`의 구조

```
memoProperties (메모 모양)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 메모 모양 정보의 개수
└── memoPr (0..∞) (Type: MemoShapeType) — 메모 모양 정보
```

#### 표 86 — memoProperties 요소

| 속성 이름 | 설명                  |
| --------- | --------------------- |
| itemCnt   | 메모 모양 정보의 개수 |

#### 표 87 — memoProperties 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| memoPr         | 메모 모양 정보 |

#### 샘플 43 — memoProperties 예

```xml
<hh:memoProperties itemCnt="1">
  <hh:memoPr id="1" width="15591" lineWidth="1" lineType="SOLID" lineColor="#B6D7AE"
    fillColor="#F0FFE9" activeColor="#CFF1C7" memoType="NOMAL"/>
</hh:memoProperties>
```

#### 9.3.10.2 memoPr 요소

메모는 문서 작성 또는 수정 중 간략한 내용을 기록해 둘 수 있는 기능이다. `<memoPr>` 요소는 실제 메모 내용을 담고 있는 것이 아니라, 화면에 표시될 메모들의 모양 정보를 가지고 있는 요소이다. 즉, 메모 선의 색, 메모의 색 등 화면 표시를 위한 설정들을 담고 있다.

#### 그림 61 — `<memoPr>`의 구조

```
memoPr (메모 모양 정보)
└── Attributes
    ├── id (Type: xs:nonNegativeInteger) — 메모 모양 정보를 구별하기 위한 아이디
    ├── width (Type: xs:nonNegativeInteger) — 메모가 보이는 넓이
    ├── lineType (Type: hc:LineType2) — 메모의 선 종류
    ├── lineColor (Type: hc:RGBColorType) — 메모의 선 색
    ├── fillColor (Type: hc:RGBColorType) — 메모의 색
    ├── activeColor (Type: hc:RGBColorType) — 메모가 활성화되었을 때의 색
    ├── memoType (Type: Restriction of 'xs:string') — 메모 변경 추적을 위한 속성
    └── lineWidth (Type: xs:string) — 메모의 라인 두께
```

#### 표 88 — memoPr 요소

| 속성 이름   | 설명                                  |
| ----------- | ------------------------------------- |
| id          | 메모 모양 정보를 구별하기 위한 아이디 |
| width       | 메모가 보이는 넓이                    |
| lineType    | 메모의 선 종류                        |
| lineColor   | 메모의 선 색                          |
| fillColor   | 메모의 색                             |
| activeColor | 메모가 활성화되었을 때의 색           |
| memoType    | 메모 변경 추적을 위한 속성            |
| lineWidth   | 메모의 라인 두께                      |

### 9.3.11 trackChanges 요소

#### 9.3.11.1 trackChanges

변경 추적 정보 목록을 가지고 있는 요소이다.

#### 그림 62 — `<trackChanges>`의 구조

```
trackChanges (변경 추적)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 변경 추적의 개수
└── trackChange (Type: TrackChange) — 변경 추적 정보
```

#### 표 89 — trackChanges 요소

| 속성 이름 | 설명             |
| --------- | ---------------- |
| itemCnt   | 변경 추적의 개수 |

#### 표 90 — trackChanges 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| trackChange    | 변경 추적 정보 |

#### 샘플 44 — trackChanges 예

```xml
<hh:trackChanges itemCnt="5">
  <hh:trackChange type="Insert" date="2021-10-15T01:08:00Z" authorID="1" hide="0" id="1"/>
  <hh:trackChange type="Insert" date="2021-10-15T01:47:00Z" authorID="1" hide="0" id="2"/>
  <hh:trackChange type="ParaShape" date="2021-10-15T01:47:00Z" authorID="1" hide="0" id="3" parashapeID="0"/>
  <hh:trackChange type="Insert" date="2021-10-15T01:51:00Z" authorID="1" hide="0" id="4"/>
  <hh:trackChange type="ParaShape" date="2021-10-15T01:51:00Z" authorID="1" hide="0" id="5" parashapeID="20"/>
</hh:trackChanges>
```

#### 9.3.11.2 trackChange 요소

변경 추적 정보를 가지고 있는 요소이다.

#### 그림 63 — `<trackChange>`의 구조

```
trackChange (변경 추적 정보)
└── Attributes
    ├── type (Type: hc:TrackChangeType) — 변경 추적의 종류
    ├── date (Type: xs:dateTime) — 변경 추적 시간
    ├── authorID (Type: xs:nonNegativeInteger) — 변경 추적 검토자를 구별하기 위한 아이디
    ├── charShapeID (Type: xs:nonNegativeInteger) — 변경 추적 글자의 서식 정보
    ├── paraShapeID (Type: xs:nonNegativeInteger) — 변경 추적 문단의 서식 정보
    ├── hide (Type: xs:boolean) — 변경 추적 화면 표시 여부
    └── id (Type: xs:nonNegativeInteger) — 변경 추적 적용 문서 구분 아이디
```

#### 표 91 — trackChange 요소

| 속성 이름   | 설명                                                                                                              |
| ----------- | ----------------------------------------------------------------------------------------------------------------- |
| type        | 변경 추적의 종류. UnKnown: 없음, Insert: 삽입, Delete: 삭제, CharShape: 글자 서식 변경, ParaShape: 문단 서식 변경 |
| date        | 변경 추적 시간. "%04d-%02d-%02dT%d:%d:%dZ": 년,월,일,시,분                                                        |
| authorID    | 변경 추적 검토자를 구별하기 위한 아이디                                                                           |
| charShapeID | 변경 추적 글자의 서식 정보                                                                                        |
| paraShapeID | 변경 추적 문단의 서식 정보                                                                                        |
| hide        | 변경 추적 화면 표시 여부                                                                                          |
| id          | 변경 추적 적용 문서 구분 아이디                                                                                   |

### 9.3.12 trackChangeAuthors 요소

#### 9.3.12.1 trackChangeAuthors 일반 항목

변경 추적 검토자 목록을 가지고 있는 요소이다.

#### 그림 64 — `<trackChangeAuthors>`의 구조

```
trackChangeAuthors (변경추적 작성자)
├── Attributes
│   └── itemCnt (Type: xs:nonNegativeInteger) — 변경 추적 검토자 수
└── trackChangeAuthor (Type: TrackChangeAuthor) — 변경 추적 검토자
```

#### 표 92 — trackChangeAuthors 요소

| 속성 이름 | 설명                |
| --------- | ------------------- |
| itemCnt   | 변경 추적 검토자 수 |

#### 표 93 — trackChangeAuthors 하위 요소

| 하위 요소 이름    | 설명             |
| ----------------- | ---------------- |
| trackChangeAuthor | 변경 추적 검토자 |

#### 샘플 45 — trackChangeAuthors 예

```xml
<hh:trackChangeAuthors itemCnt="1">
  <hh:trackChangeAuthor name="hancom" mark="1" id="1"/>
</hh:trackChangeAuthors>
```

#### 9.3.12.2 trackChangeAuthor 요소

```
trackChangeAuthor (변경 추적 사용자 정보)
└── Attributes
    ├── name (Type: xs:string) — 검토자 이름
    ├── mark (Type: xs:boolean) — 검토 표시 여부
    ├── color (Type: hc:RGBColorType) — 검토 표시 색상
    └── id (Type: xs:nonNegativeInteger) — 검토자를 구별하기 위한 아이디
```

#### 표 94 — trackChangeAuthor 요소

| 속성 이름 | 설명                          |
| --------- | ----------------------------- |
| name      | 검토자 이름                   |
| mark      | 검토 표시 여부                |
| color     | 검토 표시 색상                |
| id        | 검토자를 구별하기 위한 아이디 |

#### 샘플 46 — trackChangeAuthor 예

```xml
<hh:trackChangeAuthors itemCnt="1">
  <hh:trackChangeAuthor name="hancom" mark="1" id="1"/>
</hh:trackChangeAuthors>
```
