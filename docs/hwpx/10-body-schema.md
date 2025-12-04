# 10 본문 XML 스키마

> **출처**: KS X 6101:2024, 페이지 92-111

## 10.1 네임스페이스

Body XML은 기본적으로 "http://www.owpml.org/owpml/2024/body"을 기본 네임스페이스로 사용한다. 기본 네임스페이스의 접두어(prefix)는 기본적으로 "hb"를 사용한다. 잘못된 사용을 줄이기 위해서 "hb"를 기본 네임스페이스(http://www.owpml.org/owpml/2024/body) 이외의 네임스페이스에 사용하지 않는 것을 권고한다.

## 10.2 본문 개요

#### 그림 66 — 논리적 구조

```
Body ─────┬───── Section ─────┬───── P ─────── ParaList ─────── P
          │                   │
          │                   └───── P ─────── ParaList ─────── P
          │                   │
          │                   └───── P
          │
          └───── Section ─────────── P
```

본문의 논리적인 구조는 '본문-구역-문단'이다. 위의 그림은 논리적인 구조를 도식화한 그림이다. 본문은 구역들의 목록으로 구성된다. 이 문서에서 서술하는 규격에서는 본문(Body)은 따로 존재하지 않고, 각 구역(Section)은 개별 파일로 저장된다. 구역은 반드시 한 개 이상 존재해야 하며, 한 구역은 반드시 한 개 이상의 문단을 가지고 있어야 한다. 표/글상자와 같은 특수한 경우, 문단은 다시 문단 목록을 가지고 있을 수 있다. 이 경우 문단은 여러 개의 문단 목록을 자식 요소로서 가지고 있을 수 있다. 문단은 실제 문서 내용이 가지고 있는 단위로, 단순 텍스트뿐만 아니라 표, 그림, 그리기 객체 등 다양한 형태의 콘텐츠를 가지고 있을 수 있다.

## 10.3 sec 요소

`<sec>` 요소는 내부적으로 구역에 대한 설정 정보를 가지게 되는데, 이에 대한 자세한 내용은 **10.6**을 참조한다.

#### 그림 67 — `<sec>`의 구조

```
sec (Type: hp:SectionType)    ─────── hp:SectionType
    Root Element                         └── p (1..∞)
```

#### 표 95 — sec 요소

| 하위 요소 이름 | 설명 |
| -------------- | ---- |
| p              | 문단 |

## 10.4 p 요소

`<p>` 요소는 HWP 문서에서 내용 표현을 위한 기본 단위이며 문단을 나타낸다.

#### 그림 68 — `<p>`의 구조

```
PType (문단)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 문단을 식별하기 위한 아이디
│   ├── paraPrIDRef (Type: xs:nonNegativeInteger) — 문단 모양 아이디 참조값
│   ├── styleIDRef (Type: xs:nonNegativeInteger) — 문단 스타일 아이디 참조값
│   ├── pageBreak (Type: xs:boolean, Default: false) — 쪽 나눔 여부
│   ├── columnBreak (Type: xs:boolean, Default: false) — 단 나눔 여부
│   ├── merged (Type: xs:boolean, Default: false) — 문단 병합 여부
│   └── paraTcId (Type: xs:nonNegativeInteger) — 문단 번호 변경 추적 아이디
├── run — 구역 속성 정보
└── metaTag (Type: hc:MetaTagType) — 메타태그 관련 정보
```

#### 표 96 — p 요소

| 속성 이름   | 설명                        |
| ----------- | --------------------------- |
| id          | 문단을 식별하기 위한 아이디 |
| paraPrIDRef | 문단 모양 아이디 참조값     |
| styleIDRef  | 문단 스타일 아이디 참조값   |
| pageBreak   | 쪽 나눔 여부                |
| columnBreak | 단 나눔 여부                |
| merged      | 문단 병합 여부              |
| paraTcId    | 문단 번호 변경 추적 아이디  |

#### 표 97 — p 하위 요소

| 하위 요소 이름 | 설명               |
| -------------- | ------------------ |
| run            | 구역 속성 정보     |
| metaTag        | 메타태그 관련 정보 |

#### 샘플 47 — p 예

```xml
<hp:p id="3121190098" paraPrIDRef="0" styleIDRef="0" pageBreak="0" columnBreak="0"
  merged="0">
  <hp:run charPrIDRef="0">
    <hp:t>샘플 문서</hp:t>
  </hp:run>
</hp:p>
```

## 10.5 run 요소

run은 글자 속성 컨테이너를 의미한다. 하나 혹은 여러 개의 글자가 가지고 있는 동일한 속성을 나타낸다. 문서의 모든 콘텐츠와 제어 관련 요소들은 `<run>` 요소로 묶여서 구성된다. `<run>` 요소는 크게 두 가지 형태의 자식 요소를 가진다. 하나는 구역, 단, 문단의 제어에 관련된 요소들을 가지는 `<ctrl>` 요소와, 다른 하나는 문자열, 표, 그림 등의 실제 내용을 가지는 `<t>` 요소이다.

#### 그림 69 — `<run>`의 구조

```
run (글자 속성 컨테이너)
├── Attributes
│   ├── charPrIDRef — 글자 모양 설정 아이디 참조값
│   └── charTcId — 글자 모양 변경 추적 아이디
├── secPr — 구역 설정 정보
├── ctrl — 문단 제어 정보
├── t — 텍스트 문자열
├── tbl — 표
├── pic — 그림
├── container — 묶음 객체
├── ole — OLE
├── equation — 수식
├── line — 선
├── rect — 사각형
├── ellipse — 호
├── arc — 타원
├── polygon — 다각형
├── curve — 곡선
├── connectLine — 연결선
├── textart — 글맵시
├── compose — 글자 겹침
├── dutmal — 덧말
├── btn — 버튼
├── radioBtn — 라디오 버튼
├── checkBtn — 체크 버튼
├── comboBox — 콤보 박스
├── listBox — 리스트 박스
├── edit — 에디트
├── scrollBar — 스크롤바
├── video — 비디오
└── chart — 차트
```

#### 표 98 — run 요소

| 속성 이름   | 설명                         |
| ----------- | ---------------------------- |
| charPrIDRef | 글자 모양 설정 아이디 참조값 |
| charTcId    | 글자 모양 변경 추적 아이디   |

#### 표 99 — run 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| secPr          | 구역 설정 정보 |
| ctrl           | 문단 제어 정보 |
| t              | 텍스트 문자열  |
| tbl            | 표             |
| pic            | 그림           |
| container      | 묶음 객체      |
| ole            | OLE            |
| equation       | 수식           |
| line           | 선             |
| rect           | 사각형         |
| ellipse        | 호             |
| arc            | 타원           |
| polygon        | 다각형         |
| curve          | 곡선           |
| connectLine    | 연결선         |
| textart        | 글맵시         |
| compose        | 글자 겹침      |
| dutmal         | 덧말           |
| btn            | 버튼           |
| radioBtn       | 라디오 버튼    |
| checkBtn       | 체크 버튼      |
| comboBox       | 콤보 박스      |
| listBox        | 리스트 박스    |
| edit           | 에디트         |
| scrollBar      | 스크롤바       |
| video          | 비디오         |
| chart          | 차트           |

## 10.6 secPr 요소

### 10.6.1 구역(Section)

구역(Section)은 콘텐츠의 영역을 구분 짓는 가장 큰 단위이다. `<secPr>` 요소는 구역 내에서의 각종 설정 정보를 가지고 있는 요소이다.

#### 그림 70 — `<secPr>`의 구조

```
secPr (Type: hp:SectionDefinitionType)
├── Attributes
│   ├── id (Type: xs:string) — 구역 정의를 식별하기 위한 아이디
│   ├── textDirection (Type: Restriction of 'xs:string', Default: HORIZONTAL) — 구역 내 텍스트 방향
│   ├── spaceColumns (Type: xs:integer, Default: 0) — 동일한 폴리지에서 서로 다른 단 사이의 간격
│   ├── tabStop (Type: xs:integer, Default: 8000) — 기본 탭 간격
│   ├── tabStopVal (Type: xs:integer) — 기본 탭 간격(1.31 이후 버전)
│   ├── tabStopUnit (Type: Restriction of 'xs:string', Default: HWPUNIT) — 기본 탭 간격 단위(1.31 이후 버전)
│   ├── outlineShapeIDRef (Type: xs:nonNegativeInteger) — 개요 번호 모양 아이디 참조값
│   ├── memoShapeIDRef (Type: xs:nonNegativeInteger) — 메모 모양 아이디 참조값
│   ├── textVerticalWidthHead (Type: xs:boolean, Default: false) — 머리말/꼬리말 세로 쓰기 여부
│   └── masterPageCnt (Type: xs:nonNegativeInteger, Default: 0) — 구역 내에서 정의된 바탕쪽 설정의 개수
├── startNum — 구역 내 각 객체들의 시작 번호 정보
├── grid — 줄맞춤 정보
├── visibility — 감추기/보여주기 정보
├── lineNumberShape — 줄 번호 정보
├── pagePr — 용지 설정 정보
├── footNotePr (Type: hp:FootNoteShapeType) — 각주 모양 정보
├── endNotePr (Type: hp:EndNoteShapeType) — 미주 모양 정보
├── pageBorderFill (0..3) — 쪽 테두리/배경 정보
├── masterPage (0..∞) — 바탕쪽 설정 정보
└── presentation — 프레젠테이션 정보
```

#### 표 100 — secPr 요소

| 속성 이름             | 설명                                       |
| --------------------- | ------------------------------------------ |
| id                    | 구역 정의를 식별하기 위한 아이디           |
| textDirection         | 구역 내 텍스트 방향                        |
| spaceColumns          | 동일한 페이지에서 서로 다른 단 사이의 간격 |
| tabStop               | 기본 탭 간격                               |
| tabStopVal            | 기본 탭 간격(1.31 이후 버전)               |
| tabStopUnit           | 기본 탭 간격 단위(1.31 이후 버전)          |
| outlineShapeIDRef     | 개요 번호 모양 아이디 참조값               |
| memoShapeIDRef        | 메모 모양 아이디 참조값                    |
| textVerticalWidthHead | 머리말/꼬리말 세로 쓰기 여부               |
| masterPageCnt         | 구역 내에서 정의된 바탕쪽 설정의 개수      |

#### 표 101 — secPr 하위 요소

| 하위 요소 이름  | 설명                               |
| --------------- | ---------------------------------- |
| startNum        | 구역 내 각 객체들의 시작 번호 정보 |
| grid            | 줄맞춤 정보                        |
| visibility      | 감추기/보여주기 정보               |
| lineNumberShape | 줄 번호 정보                       |
| pagePr          | 용지 설정 정보                     |
| footNotePr      | 각주 모양 정보                     |
| endNotePr       | 미주 모양 정보                     |
| pageBorderFill  | 쪽 테두리/배경 정보                |
| masterPage      | 바탕쪽 설정 정보                   |
| presentation    | 프레젠테이션 정보                  |

#### 샘플 48 — secPr 예

```xml
<hp:secPr id="" textDirection="HORIZONTAL" spaceColumns="1134" tabStop="8000"
  tabStopVal="4000" tabStopUnit="HWPUNIT" outlineShapeIDRef="1" memoShapeIDRef="0"
  textVerticalWidthHead="0" masterPageCnt="0">
  <hp:grid lineGrid="0" charGrid="0" wonggojiFormat="0"/>
  <hp:startNum pageStartsOn="BOTH" page="0" pic="0" tbl="0" equation="0"/>
  <hp:visibility hideFirstHeader="0" hideFirstFooter="0" hideFirstMasterPage="0" border="SHOW_ALL"
    fill="SHOW_ALL" hideFirstPageNum="0" hideFirstEmptyLine="0"
    showLineNumber="0"/>
  <hp:lineNumberShape restartType="0" countBy="0" distance="0" startNumber="0"/>
  ......
</hp:secPr>
```

### 10.6.2 startNum 요소

구역 내에서 각종 시작 번호들에 대한 설정을 가지고 있는 요소이다.

#### 그림 71 — `<startNum>`의 구조

```
startNum (시작 번호 정보)
└── Attributes
    ├── pageStartsOn (Type: restriction of 'xs:string', Default: BOTH) — 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션
    ├── page (Type: xs:nonNegativeInteger, Default: 0) — 쪽 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작
    ├── pic (Type: xs:nonNegativeInteger, Default: 0) — 그림 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작
    ├── tbl (Type: xs:nonNegativeInteger, Default: 0) — 표 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작
    └── equation (Type: xs:nonNegativeInteger, Default: 0) — 수식 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작
```

#### 표 102 — startNum 요소

| 속성 이름    | 설명                                                                                     |
| ------------ | ---------------------------------------------------------------------------------------- |
| pageStartsOn | 구역 나눔으로 새 페이지가 생길 때 페이지 번호 적용 옵션                                  |
| page         | 쪽 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작   |
| pic          | 그림 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작 |
| tbl          | 표 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작   |
| Equation     | 수식 시작 번호. 값이 0이면 앞 구역에 이어서 번호를 매기고, 1 이상이면 임의의 번호로 시작 |

#### 샘플 49 — startNum 예

```xml
<hp:startNum pageStartsOn="BOTH" page="0" pic="0" tbl="0" equation="0"/>
```

### 10.6.3 grid 요소

구역 내의 줄맞춤 설정 정보를 표현하기 위한 요소이다.

#### 그림 72 — `<grid>`의 구조

```
grid (줄맞춤 정보)
└── Attributes
    ├── lineGrid (Type: xs:nonNegativeInteger, Default: 0) — 세로로 줄맞춤을 할지 여부
    └── charGrid (Type: xs:nonNegativeInteger, Default: 0) — 가로로 줄맞춤을 할지 여부
```

#### 표 103 — grid 요소

| 속성 이름 | 설명                      |
| --------- | ------------------------- |
| lineGrid  | 세로로 줄맞춤을 할지 여부 |
| charGrid  | 가로로 줄맞춤을 할지 여부 |

#### 샘플 50 — grid 예

```xml
<hp:grid lineGrid="0" charGrid="0" wonggojiFormat="0"/>
```

### 10.6.4 visibility 요소

구역 내의 각 요소들에 대한 보여주기/감추기 설정 정보를 표현하기 위한 요소이다.

#### 그림 73 — `<visibility>`의 구조

```
visibility (감추기/보여주기 정보)
└── Attributes
    ├── hideFirstHeader (Type: xs:boolean, Default: false) — 첫 쪽에만 머리말 감추기 여부
    ├── hideFirstFooter (Type: xs:boolean, Default: false) — 첫 쪽에만 꼬리말 감추기 여부
    ├── hideFirstMasterPage (Type: xs:boolean, Default: false) — 첫 쪽에만 바탕쪽 감추기 여부
    ├── border (Type: hp:VisibilityValue) — 테두리 감추기/보여주기 여부(첫 쪽에만 감추기, 첫 쪽에만 보여주기, 모두 보여주기)
    ├── fill (Type: hp:VisibilityValue) — 배경 감추기/보여주기 여부(첫 쪽에만 감추기, 첫 쪽에만 보여주기, 모두 보여주기)
    ├── hideFirstPageNum (Type: xs:boolean, Default: false) — 첫 쪽에만 쪽번호 감추기 여부
    ├── hideFirstEmptyLine (Type: xs:boolean, Default: false) — 첫 쪽에만 빈줄 감추기 여부
    └── showLineNumber (Type: xs:boolean, Default: false) — 줄 번호 표시 여부
```

#### 표 104 — visibility 요소

| 속성 이름           | 설명                                                                             |
| ------------------- | -------------------------------------------------------------------------------- |
| hideFirstHeader     | 첫 쪽에만 머리말 감추기 여부                                                     |
| hideFirstFooter     | 첫 쪽에만 꼬리말 감추기 여부                                                     |
| hideFisrtMasterPage | 첫 쪽에만 바탕쪽 감추기 여부                                                     |
| border              | 테두리 감추기/보여주기 여부(첫 쪽에만 감추기, 첫 쪽에만 보여주기, 모두 보여주기) |
| fill                | 배경 감추기/보여주기 여부(첫 쪽에만 감추기, 첫 쪽에만 보여주기, 모두 보여주기)   |
| hideFistPageNumber  | 첫 쪽에만 쪽번호 감추기 여부                                                     |
| hideFirstEmptyLine  | 첫 쪽에만 빈 줄 감추기 여부                                                      |
| showLineNumber      | 줄 번호 표시 여부                                                                |

#### 샘플 51 — visibility 예

```xml
<hp:visibility hideFirstHeader="0" hideFirstFooter="0" hideFirstMasterPage="0" border="SHOW_ALL"
  fill="SHOW_ALL" hideFirstPageNum="0" hideFirstEmptyLine="0" showLineNumber="0"/>
```

### 10.6.5 lineNumberShape 요소

#### 그림 74 — lineNumberShape 요소

```
lineNumberShape ──── Attributes
                      (줄 번호 정보)
```

구역 내의 줄 번호 정보를 표현하기 위한 요소이다.

#### 표 105 — linNumberShape 요소

| 속성 이름   | 설명                  |
| ----------- | --------------------- |
| restartType | 줄 번호 방식          |
| countBy     | 줄 번호 표시 간격     |
| distance    | 본문과의 줄 번호 위치 |
| startNumber | 줄 번호 시작 번호     |

#### 샘플 52 — lineNumberShape 예

```xml
<hp:lineNumberShape restartType="3" countBy="1" distance="2834" startNumber="1"/>
```

### 10.6.6 pagePr 요소

#### 10.6.6.1 pagePr

구역 내의 용지 설정 정보를 표현하기 위한 요소이다.

#### 그림 75 — `<pagePr>`의 구조

```
pagePr (용지 설정 정보)
├── Attributes
│   ├── landscape (Type: restriction of 'xs:string', Default: NARROWLY) — 용지 방향
│   ├── width (Type: xs:positiveInteger, Default: 59528) — 용지 가로 크기. 단위는 HWPUNIT
│   ├── height (Type: xs:positiveInteger, Default: 84188) — 용지 세로 크기. 단위는 HWPUNIT
│   └── gutterType (Type: restriction of 'xs:string', Default: LEFT_ONLY) — 제책 방법
└── margin — 용지 여백 정보
```

#### 표 106 — pagePr 요소

| 속성 이름  | 설명                                                           |
| ---------- | -------------------------------------------------------------- |
| landscape  | 용지 방향                                                      |
| width      | 용지 가로 크기. 단위는 HWPUNIT                                 |
| height     | 용지 세로 크기. 단위는 HWPUNIT                                 |
| gutterType | 제책 방법. LEFT_ONLY: 왼쪽, LEFT_RIGHT: 맞쪽, TOP_BOTTOM: 위쪽 |

#### 표 107 — pagePr 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| margin         | 용지 여백 정보 |

#### 샘플 53 — pagePr 예

```xml
<hp:pagePr landscape="WIDELY" width="59528" height="84186" gutterType="LEFT_ONLY">
  <hp:margin header="4252" footer="4252" gutter="0" left="8504" right="8504" top="5668" bottom="4252"/>
</hp:pagePr>
```

#### 10.6.6.2 MarginAttributeGroup

[MarginAttributeGroup]은 여백 정보를 표현할 때 공통적으로 사용되는 속성들을 묶은 형식이다. [MarginAttributeGroup]은 `<margin>` 요소, `<outMargin>` 요소 등에서 사용된다.

#### 그림 76 — [MarginAttributeGroup]의 구조

```
MarginAttributeGroup (여백 정보)
├── left (Type: xs:nonNegativeInteger) — 왼쪽 여백. 단위는 HWPUNIT
├── right (Type: xs:nonNegativeInteger) — 오른쪽 여백. 단위는 HWPUNIT
├── top (Type: xs:nonNegativeInteger) — 위쪽 여백. 단위는 HWPUNIT
└── bottom (Type: xs:nonNegativeInteger) — 아래쪽 여백. 단위는 HWPUNIT
```

#### 표 108 — MarginAttributeGroup 요소

| 속성 이름 | 설명                        |
| --------- | --------------------------- |
| left      | 왼쪽 여백. 단위는 HWPUNIT   |
| right     | 오른쪽 여백. 단위는 HWPUNIT |
| top       | 위쪽 여백. 단위는 HWPUNIT   |
| bottom    | 아래쪽 여백. 단위는 HWPUNIT |

#### 10.6.6.3 margin 요소

`<margin>` 요소는 속성에 [MarginAttributeGroup]을 포함한다. [MarginAttributeGroup]은 **10.6.6.2**를 참조한다.

#### 그림 77 — `<margin>`의 구조

```
margin (여백 정보)
├── Attributes
│   ├── [MarginAttributeGroup] — 여백 정보
│   ├── header (Type: xs:nonNegativeInteger, Default: 4252) — 머리말 여백. 단위는 HWPUNIT
│   ├── footer (Type: xs:nonNegativeInteger, Default: 4252) — 꼬리말 여백. 단위는 HWPUNIT
│   └── gutter (Type: xs:nonNegativeInteger, Default: 0) — 제본 여백
```

#### 표 109 — margin 요소

| 속성 이름              | 설명                        |
| ---------------------- | --------------------------- |
| [MarginAttributeGroup] | 10.6.6.2 참조               |
| header                 | 머리말 여백. 단위는 HWPUNIT |
| footer                 | 꼬리말 여백. 단위는 HWPUNIT |
| gutter                 | 제본 여백. 단위는 HWPUNIT   |

#### 샘플 54 — margin 예

```xml
<hp:margin header="4252" footer="4252" gutter="0" left="8504" right="8504" top="5668"
  bottom="4252"/>
```

### 10.6.7 footNotePr 요소

#### 10.6.7.1 footNotePr

각주 모양 정보를 가지고 있는 요소이다.

#### 그림 78 — `<footNotePr>`의 구조

```
footNotePr (각주 모양)
├── hp:FootNoteShapeType
│   Base Type: hp:NoteShapeType
│   └── hp:NoteShapeType (extension base)
│       Abstract: true
│       ├── autoNumFormat (Type: hp:AutoNumFormatType) — 자동 번호 매김 모양 정보
│       ├── noteLine — 구분선 모양 정보
│       └── noteSpacing — 여백 정보
└── 각주/미주 모양 정보
    ├── numbering — 번호 매김 형식
    └── placement — 위치 정보
```

#### 표 110 — footNotePr 요소

| 하위 요소 이름 | 설명                     |
| -------------- | ------------------------ |
| autoNumFormat  | 자동 번호 매김 모양 정보 |
| noteLine       | 구분선 모양 정보         |
| noteSpacing    | 여백 정보                |
| numbering      | 번호 매김 형식           |
| placement      | 위치 정보                |

#### 샘플 55 — footNotePr 예

```xml
<hp:footNotePr>
  <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")" supscript="0"/>
  <hp:noteLine length="-1" type="SOLID" width="0.12 mm" color="#000000"/>
  <hp:noteSpacing betweenNotes="283" belowLine="567" aboveLine="850"/>
  <hp:numbering type="CONTINUOUS" newNum="1"/>
  <hp:placement place="EACH_COLUMN" beneathText="0"/>
</hp:footNotePr>
```

#### 10.6.7.2 autoNumFormat 요소

각주/미주 내에서 사용되는 자동 번호 매김 모양 정보를 가지고 있는 요소이다.

#### 그림 79 — `<autoNumFormat>`의 구조

```
autoNumFormat (자동 번호 형식)
└── Attributes
    ├── type (Type: hc:NumberType2, Default: DIGIT) — 번호 모양 종류
    ├── userChar (Type: xs:string) — 사용자 정의 기호. type이 USER_CHAR로 설정된 경우, 번호 모양으로 사용될 사용자 정의 글자
    ├── prefixChar (Type: xs:string) — 앞 장식 문자
    ├── suffixChar (Type: xs:string, Default: )) — 뒤 장식 문자
    └── supscript (Type: xs:boolean, Default: false) — 각주/미주 내용 중 번호 코드의 모양을 위첨자 형식으로 할지 여부
```

#### 표 111 — autoNumFormat 요소

| 속성 이름  | 설명                                                                                    |
| ---------- | --------------------------------------------------------------------------------------- |
| type       | 번호 모양 종류                                                                          |
| userChar   | 사용자 정의 기호. type이 USER_CHAR로 설정된 경우, 번호 모양으로 사용될 사용자 정의 글자 |
| prefixChar | 앞 장식 문자                                                                            |
| suffixChar | 뒤 장식 문자                                                                            |
| supscript  | 각주/미주 내용 중 번호 코드의 모양을 위첨자 형식으로 할지 여부                          |

#### 샘플 56 — autoNumFormat 예

```xml
<hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")" supscript="0"/>
```

#### 10.6.7.3 noteLine 요소

각주/미주 내에서 사용되는 구분선 모양 정보를 가지고 있는 요소이다.

#### 그림 80 — `<noteLine>`의 구조

```
noteLine (구분선 정보)
└── Attributes
    ├── length (Type: xs:string, Default: 0) — 구분선 길이. 0(구분선 없음), 5cm, 2cm, Column/3(단 크기의 1/3), Column(단 크기), 그 외 (HWPUNIT 단위의 사용자 지정 길이)
    ├── type (Type: hc:LineType2, Default: SOLID) — 구분선 종류
    ├── width (Type: hc:LineWidth, Default: 0.12 mm) — 구분선 굵기. 단위는 mm
    └── color (Type: hc:RGBColorType, Default: #000000) — 구분선 색
```

#### 표 112 — noteLine 요소

| 속성 이름 | 설명                                                                                                                       |
| --------- | -------------------------------------------------------------------------------------------------------------------------- |
| length    | 구분선 길이. 0(구분선 없음), 5 cm, 2 cm, Column/3(단 크기의 1/3), Column(단 크기), 그 외 (HWPUNIT 단위의 사용자 지정 길이) |
| type      | 구분선 종류                                                                                                                |
| width     | 구분선 굵기. 단위는 mm                                                                                                     |
| color     | 구분선 색                                                                                                                  |

#### 10.6.7.4 noteSpacing 요소

각주/미주 내에서 사용되는 여백 정보를 가지고 있는 요소이다.

#### 그림 81 — `<noteSpacing>`의 구조

```
noteSpacing (여백 정보)
└── Attributes
    ├── betweenNotes (Type: xs:nonNegativeInteger, Default: 850) — 주석 사이 여백
    ├── belowLine (Type: xs:nonNegativeInteger, Default: 567) — 구분선 아래 여백
    └── aboveLine (Type: xs:nonNegativeInteger, Default: 567) — 구분선 위 여백
```

#### 표 113 — noteSpacing 요소

| 속성 이름    | 설명             |
| ------------ | ---------------- |
| betweenNotes | 주석 사이 여백   |
| belowLine    | 구분선 아래 여백 |
| aboveLine    | 구분선 위 여백   |

#### 샘플 57 — noteSpacing 예

```xml
<hp:noteSpacing betweenNotes="283" belowLine="567" aboveLine="850"/>
```

#### 10.6.7.5 footNotePr의 numbering 요소

`<footNotePr>` 요소의 `<numbering>` 요소와 `<endNotePr>` 요소의 `<numbering>` 요소는 구조상 동일하다. 하지만 속성에서 허용되는 값의 범위가 다르다. `<footNotePr>` 요소의 `<numbering>` 요소의 경우 속성 @type이 가질 수 있는 값의 범위는 CONTINUOUS, ON_SECTION, ON_PAGE이다. `<endNotePr>` 요소의 `<numbering>` 요소의 경우 속성 @type이 가질 수 있는 값의 범위는 CONTINUOUS, ON_SECTION이다.

#### 그림 82 — `<numbering>`의 구조

```
numbering (번호 매김 형식)
└── Attributes
    ├── type (Type: restriction of 'xs:string', Default: CONTINUOUS) — 번호 매기기 형식
    └── newNum (Type: xs:positiveInteger, Default: 1) — 시작 번호. type이 ON_SECTION일 때에만 사용됨
```

#### 표 114 — numbering 요소

| 속성 이름    | 설명                                         |
| ------------ | -------------------------------------------- |
| type         | 번호 매기기 형식                             |
| newNumbering | 시작 번호. type이 ON_SECTION일 때에만 사용됨 |

#### 샘플 58 — numbering 예

```xml
<hp:numbering type="CONTINUOUS" newNum="1"/>
```

#### 10.6.7.6 footNotePr의 placement 요소

`<footNotePr>` 요소의 `<placement>` 요소와 `<endNotePr>` 요소의 `<placement>` 요소는 구조상 동일하다. 하지만 속성에서 허용되는 값의 범위가 다르다. `<footNotePr>` 요소의 `<placement>` 요소의 경우 속성 @place에서 가질 수 있는 값의 범위는 EACH_COLUMN, MERGED_COLUMN, RIGHT_MOST_COLUMN이다. `<endNotePr>` 요소의 `<placement>` 요소의 경우 속성 @place에서 가질 수 있는 값의 범위는 END_OF_DOCUMENT, END_OF_SECTION이다.

#### 그림 83 — `<placement>`의 구조

```
placement (위치 정보)
└── Attributes
    ├── place (Type: Restriction of 'xs:string', Default: EACH_COLUMN) — 한 페이지 내에서 각주를 다단에 어떻게 위치시킬지에 대한 설정
    └── beneathText (Type: xs:boolean, Default: false) — 텍스트에 이어 바로 출력할지 여부
```

#### 표 115 — placement 요소

| 속성 이름   | 설명                                                         |
| ----------- | ------------------------------------------------------------ |
| place       | 한 페이지 내에서 각주를 다단에 어떻게 위치시킬지에 대한 설정 |
| beneathText | 텍스트에 이어 바로 출력할지 여부                             |

#### 샘플 59 — placement 예

```xml
<hp:placement place="EACH_COLUMN" beneathText="0"/>
```

### 10.6.8 endNotePr 요소

#### 10.6.8.1 endNotePr

미주 모양 정보를 가지고 있는 요소이다.

#### 그림 84 — `<endNotePr>`의 구조

```
endNotePr (미주 모양)
├── hp:EndNoteShapeType
│   Base Type: hp:NoteShapeType
│   └── hp:NoteShapeType (extension base)
│       Abstract: true
│       ├── autoNumFormat (Type: hp:AutoNumFormatType) — 자동 번호 매김 모양 정보
│       ├── noteLine — 구분선 모양 정보
│       └── noteSpacing — 여백 정보
└── 미주 모양 정보
    ├── numbering — 번호 매김 형식
    └── placement — 위치 정보
```

#### 표 116 — endNotePr 요소

| 하위 요소 이름 | 설명                     |
| -------------- | ------------------------ |
| autoNumFormat  | 자동 번호 매김 모양 정보 |
| noteLine       | 구분선 모양 정보         |
| noteSpacing    | 여백 정보                |
| numbering      | 번호 매김 형식           |
| placement      | 위치 정보                |

```xml
<hp:endNotePr>
  <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")" supscript="0"/>
  <hp:noteLine length="14692344" type="SOLID" width="0.12 mm" color="#000000"/>
  <hp:noteSpacing betweenNotes="0" belowLine="567" aboveLine="850"/>
  <hp:numbering type="CONTINUOUS" newNum="1"/>
  <hp:placement place="END_OF_DOCUMENT" beneathText="0"/>
</hp:endNotePr>
```

#### 10.6.8.2 endNotePr의 numbering 요소

`<footNotePr>` 요소의 `<numbering>` 요소와 `<endNotePr>` 요소의 `<numbering>` 요소는 구조상 동일하다. 하지만 속성에서 허용되는 값의 범위가 다르다. `<footNotePr>` 요소의 `<numbering>` 요소의 경우 속성 @type이 가질 수 있는 값의 범위는 CONTINUOUS, ON_SECTION, ON_PAGE이다. `<endNotePr>` 요소의 `<numbering>` 요소의 경우 속성 @type이 가질 수 있는 값의 범위는 CONTINUOUS, ON_SECTION이다.

#### 그림 85 — `<numbering>`의 구조

```
numbering (번호 매김 형식)
└── Attributes
    ├── type (Type: restriction of 'xs:string', Default: CONTINUOUS) — 번호 매기기 형식
    └── newNum (Type: xs:positiveInteger, Default: 1) — 시작 번호. type이 ON_SECTION일 때에만 사용됨
```

#### 표 117 — numbering 요소

| 속성 이름 | 설명                                         |
| --------- | -------------------------------------------- |
| type      | 번호 매기기 형식                             |
| newNum    | 시작 번호. type이 ON_SECTION일 때에만 사용됨 |

#### 샘플 61 — numbering 예

```xml
<hp:numbering type="CONTINUOUS" newNum="1"/>
```

#### 10.6.8.3 endNotePr의 placement 요소

`<footNotePr>` 요소의 `<placement>` 요소와 `<endNotePr>` 요소의 `<placement>` 요소는 구조상 동일하다. 하지만 속성에서 허용되는 값의 범위가 다르다. `<footNotePr>` 요소의 `<placement>` 요소의 경우 속성 @place에서 가질 수 있는 값의 범위는 EACH_COLUMN, MERGED_COLUMN, RIGHT_MOST_COLUMN이다. `<endNotePr>` 요소의 `<placement>` 요소의 경우 속성 @place에서 가질 수 있는 값의 범위는 END_OF_DOCUMENT, END_OF_SECTION이다.

#### 그림 86 — `<placement>`의 구조

```
placement (위치 정보)
└── Attributes
    ├── place (Type: restriction of 'xs:string', Default: END_OF_DOCUMENT) — 한 페이지 내에서 미주를 다단에 어떻게 위치시킬지에 대한 설정
    └── beneathText (Type: xs:boolean, Default: false) — 텍스트에 이어 바로 출력할지 여부
```

#### 표 118 — placement 요소

| 속성 이름   | 설명                                                         |
| ----------- | ------------------------------------------------------------ |
| place       | 한 페이지 내에서 미주를 다단에 어떻게 위치시킬지에 대한 설정 |
| beneathText | 텍스트에 이어 바로 출력할지 여부                             |

#### 샘플 62 — placement 예

```xml
<hp:placement place="END_OF_DOCUMENT" beneathText="0"/>
```

### 10.6.9 pageBorderFill 요소

#### 10.6.9.1 pageBorderFill

`<pageBorderFill>`은 구역 내에서 사용되는 테두리/배경 설정 정보를 가지고 있는 요소이다.

#### 그림 87 — `<pageBorderFill>`의 구조

```
pageBorderFill (쪽 테두리/배경 정보)
├── Attributes
│   ├── type (Type: Restriction of 'xs:string') — 종류
│   ├── borderFillIDRef (Type: xs:nonNegativeInteger) — 테두리/배경 정보 아이디 참조값
│   ├── textBorder (Type: Restriction of 'xs:string') — 쪽 테두리 위치 기준
│   ├── headerInside (Type: xs:boolean, Default: false) — 머리말 포함 여부
│   ├── footerInside (Type: xs:boolean, Default: false) — 꼬리말 포함 여부
│   └── fillArea (Type: Restriction of 'xs:string') — 채움 영역
└── offset — 테두리/배경 위치
```

#### 표 119 — pageBorderFill 요소

| 속성 이름       | 설명                           |
| --------------- | ------------------------------ |
| type            | 종류                           |
| borderFillIDRef | 테두리/배경 정보 아이디 참조값 |
| textBorder      | 쪽 테두리 위치 기준            |
| headerInside    | 머리말 포함 여부               |
| footerInside    | 꼬리말 포함 여부               |
| fillArea        | 채움 영역                      |

#### 표 120 — pageBorderFill 하위 요소

| 하위 요소 이름 | 설명                  |
| -------------- | --------------------- |
| offset         | 테두리/배경 위치 정보 |

#### 샘플 63 — pageBorderFill 예

```xml
<hp:pageBorderFill type="BOTH" borderFillIDRef="1" textBorder="PAPER" headerInside="0" footerInside="0"
  fillArea="PAPER">
  <hp:offset left="1417" right="1417" top="1417" bottom="1417"/>
</hp:pageBorderFill>
```

#### 10.6.9.2 offset 요소

구역 내에서 사용되는 테두리/배경에 대한 위치 정보를 가지고 있는 요소이다.

#### 그림 88 — `<offset>`의 구조

```
offset (테두리/배경 위치)
└── Attributes
    ├── left (Type: xs:nonNegativeInteger, Default: 1417) — 왼쪽 간격. 단위는 HWPUNIT
    ├── right (Type: xs:nonNegativeInteger, Default: 1417) — 오른쪽 간격. 단위는 HWPUNIT
    ├── top (Type: xs:nonNegativeInteger, Default: 1417) — 위쪽 간격. 단위는 HWPUNIT
    └── bottom (Type: xs:nonNegativeInteger, Default: 1417) — 아래쪽 간격. 단위는 HWPUNIT
```

#### 표 121 — offset 요소

| 속성 이름 | 설명                        |
| --------- | --------------------------- |
| left      | 왼쪽 간격. 단위는 HWPUNIT   |
| right     | 오른쪽 간격. 단위는 HWPUNIT |
| top       | 위쪽 간격. 단위는 HWPUNIT   |
| bottom    | 아래쪽 간격. 단위는 HWPUNIT |

#### 샘플 64 — offset 예

```xml
<hp:offset left="1417" right="1417" top="1417" bottom="1417"/>
```

### 10.6.10 masterPage 요소

`<masterPage>` 요소는 바탕쪽 스키마에서 설정된 정보를 참조한다. 한 섹션 내에서 바탕쪽은 여러 개가 올 수 있다.

#### 그림 89 — `<masterPage>`의 구조

```
masterPage (바탕쪽 정보)
└── Attributes
    └── idRef — 바탕쪽 설정 정보 아이디 참조값
```

#### 표 122 — masterPage 요소

| 속성 이름 | 설명                           |
| --------- | ------------------------------ |
| idRef     | 바탕쪽 설정 정보 아이디 참조값 |

#### 샘플 65 — masterPage 예

```xml
<hp:masterPage idRef="masterpage0"/>
```

### 10.6.11 presentation 요소

#### 10.6.11.1 presentation

문서의 프레젠테이션 설정 정보를 갖고 있는 요소이다.

#### 그림 90 — `<presentation>`의 구조

```
presentation (프레젠테이션 정보)
├── Attributes
│   ├── effect (Type: xs:nonNegativeInteger) — 화면 전환 효과
│   ├── soundIDRef (Type: Restriction of 'xs:string') — 효과음 바이너리 데이터에 대한 아이디 참조값
│   ├── invertText (Type: xs:boolean, Default: false) — 글자색 반전 효과 여부
│   ├── autoshow (Type: xs:boolean, Default: false) — 자동 시연 여부
│   ├── showtime (Type: xs:nonNegativeInteger) — 화면 전환 시간(초 단위)
│   └── applyto (Type: xs:nonNegativeInteger) — 적용범위
└── fillBrush (Type: hc:FillBrushType) — 채우기 정보
```

#### 표 123 — presentation 요소

| 속성 이름  | 설명                                                                             |
| ---------- | -------------------------------------------------------------------------------- |
| effect     | 화면 전환 효과                                                                   |
| soundIDRef | 효과음 바이너리 데이터에 대한 아이디 참조값                                      |
| invertText | 글자색 반전 효과 여부                                                            |
| autoshow   | 자동 시연 여부                                                                   |
| showtime   | 화면 전환 시간(초 단위)                                                          |
| applyto    | 적용범위. PRAT_WHOLE_DOCUMENT: 문서 전체, PRAT_NEWSECTION: 현재 위치부터 새 구역 |

#### 표 124 — presentation 하위 요소

| 하위 요소 이름 | 설명        |
| -------------- | ----------- |
| fillBrush      | 채우기 정보 |

#### 샘플 66 — presentation 예

```xml
<hp:presentation effect="overLeft" soundIDRef="" invertText="0" autoshow="0" showtime="0" applyto="WholeDoc">
  <hp:fillBrush>
    <hc:winBrush faceColor="#FF6600" hatchColor="#FF6600" alpha="0"/>
  </hp:fillBrush>
</hp:presentation>
```

#### 10.6.11.2 화면 전환 효과

#### 표 125 — 화면 전환 효과 요소

| 화면 전환 효과      | 설명                |
| ------------------- | ------------------- |
| PRE_NONE            | 없음                |
| PRE_OVER_LEFT       | 왼쪽으로 펼치기     |
| PRE_OVER_RIGHT      | 오른쪽으로 펼치기   |
| PRE_OVER_UP         | 위로 펼치기         |
| PRE_OVER_DOWN       | 아래로 펼치기       |
| PRE_RECT_OUT        | 상자형으로 펼치기   |
| PRE_RECT_IN         | 상자형으로 오므리기 |
| PRE_BLIND_LEFT      | 왼쪽 블라인드       |
| PRE_BLIND_RIGHT     | 오른쪽 블라인드     |
| PRE_BLIND_UP        | 위쪽 블라인드       |
| PRE_BLIND_DOWN      | 아래쪽 블라인드     |
| PRE_CUTTON_HORZ_OUT | 수평 커튼 열기      |
| PRE_CUTTON_HORZ_IN  | 수평 커튼 닫기      |
| PRE_CUTTON_VERT_OUT | 수직 커튼 열기      |
| PRE_CUTTON_VERT_IN  | 수직 커튼 닫기      |
| PRE_MOVE_LEFT       | 왼쪽으로 가리기     |
| PRE_MOVE_RIGHT      | 오른쪽으로 가리기   |
| PRE_MOVE_UP         | 위로 가리기         |
| PRE_MOVE_DOWN       | 아래로 가리기       |
| PRE_RANDOM          | 임의선택            |

## 10.7 ctrl 요소

`<ctrl>` 요소는 콘텐츠에서 본문 내 제어 관련 요소들을 모은 요소이다.

#### 그림 91 — `<ctrl>`의 구조

```
ctrl (제어 요소)
├── colPr — 단 설정 정보
├── fieldBegin — 필드 시작
├── fieldEnd — 필드 끝
├── bookmark — 책갈피
├── header — 머리말
├── footer — 꼬리말
├── footNote — 각주
├── endNote — 미주
├── autoNum — 자동 번호
├── newNum — 새 번호
├── pageNumCtrl — 홀/짝수 조정
├── pageHiding — 감추기
├── pageNum — 쪽번호 위치
├── indexmark — 찾아보기 표식
└── hiddenComment — 숨은 설명
```

#### 표 126 — ctrl 요소

| 하위 요소 이름 | 설명                                        |
| -------------- | ------------------------------------------- |
| colPr          | 단 설정 정보                                |
| fieldBegin     | 필드 시작                                   |
| fieldEnd       | 필드 끝                                     |
| bookmark       | 책갈피                                      |
| header         | 머리말. 10.7.5 머리말/꼬리말 요소 형식 참조 |
| footer         | 꼬리말. 10.7.5 머리말/꼬리말 요소 형식 참조 |
| footNote       | 각주. 10.7.6 각주/미주 요소 형식 참조       |
| endNote        | 미주. 10.7.6 각주/미주 요소 형식 참조       |
| autoNum        | 자동 번호                                   |
| newNum         | 새 번호                                     |
| pageNumCtrl    | 홀/짝수 조정                                |
| pageHiding     | 감추기                                      |
| pageNum        | 쪽번호 위치                                 |
| indexmark      | 찾아보기 표식                               |
| hiddenComment  | 숨은 설명                                   |

#### 샘플 67 — ctrl 예

```xml
<hp:ctrl>
  <hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="1" sameSz="1" sameGap="0"/>
</hp:ctrl>
```

### 10.7.1 colPr 요소

#### 10.7.1.1 colPr

단 설정 정보를 가지고 있는 요소이다.

#### 그림 92 — `<colPr>`의 구조

```
colPr (단 설정 정보)
├── Attributes
│   ├── id — 단 설정 정보를 구별하기 위한 아이디
│   ├── type — 단 종류
│   ├── layout — 단 방향 지정
│   ├── colCount — 단 개수
│   ├── sameSz — 단 너비를 동일하게 지정할지 여부. true이면 동일한 너비, false이면 각기 다른 너비
│   └── sameGap — 단 사이 간격. 단 너비를 동일하게 지정했을 경우에만 사용됨
├── colLine — 단 구분선
└── colSz (0..255) — 단 사이 간격. sameSz가 false일 때, 각 단의 크기 및 사이 간격
```

#### 표 127 — colPr 요소

| 속성 이름 | 설명                                                                             |
| --------- | -------------------------------------------------------------------------------- |
| id        | 단 설정 정보를 구별하기 위한 아이디                                              |
| type      | 단 종류                                                                          |
| layout    | 단 방향 지정                                                                     |
| colCount  | 단 개수                                                                          |
| sameSz    | 단 너비를 동일하게 지정할지 여부. true이면 동일한 너비, false이면 각기 다른 너비 |
| sameGap   | 단 사이 간격. 단 너비를 동일하게 지정했을 경우에만 사용됨                        |

#### 표 128 — colPr 하위 요소

| 하위 요소 이름 | 설명                                                         |
| -------------- | ------------------------------------------------------------ |
| colLine        | 단 구분선                                                    |
| colSz          | 단 사이 간격. 단 너비를 각기 다르게 지정했을 경우에만 사용됨 |

#### 샘플 68 — colPr 예

```xml
<hp:ctrl>
  <hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="1" sameSz="1" sameGap="0"/>
</hp:ctrl>
```

#### 10.7.1.2 colLine 요소

단 사이의 구분선 설정 정보를 가지고 있는 요소이다.

#### 그림 93 — `<colLine>`의 구조

```
colLine (단 구분선)
└── Attributes
    ├── type (Type: hc:LineType2, Default: SOLID) — 구분선 종류
    ├── width (Type: hc:LineWidth, Default: 0.12 mm) — 구분선 굵기
    └── color (Type: hc:RGBColorType, Default: #000000) — 구분선 색
```

#### 표 129 — colLine 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| type      | 구분선 종류 |
| width     | 구분선 굵기 |
| color     | 구분선 색   |

#### 샘플 69 — colLine 예

```xml
<hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="2" sameSz="1" sameGap="14174">
  <hp:colLine type="DOUBLE_SLIM" width="0.7 mm" color="#3A3C84"/>
</hp:colPr>
```

#### 10.7.1.3 colSz 요소

`<colPr>`의 속성 중 @sameSz 속성이 false(각기 다른 단 사이 간격을 가짐)로 설정되었을 때에만 사용되는 요소이다.

#### 그림 94 — `<colSz>`의 구조

```
colSz (단 크기)
└── Attributes
    ├── width (Type: xs:positiveInteger) — 단의 크기
    └── gap (Type: xs:nonNegativeInteger) — 단 사이 간격
```

#### 표 130 — colSz 요소

| 속성 이름 | 설명         |
| --------- | ------------ |
| width     | 단의 크기    |
| gap       | 단 사이 간격 |

#### 샘플 70 — colSz 예

```xml
<hp:colPr id="" type="NEWSPAPER" layout="LEFT" colCount="2" sameSz="0" sameGap="2268">
  <hp:colLine type="DOUBLE_SLIM" width="0.7 mm" color="#3A3C84"/>
  <hp:colSz width="20097" gap="1747"/>
  <hp:colSz width="10924" gap="0"/>
</hp:colPr>
```

### 10.7.2 fieldBegin 요소

#### 10.7.2.1 fieldBegin

메모, 외부 연결, 북마크 등 문서 내에서 부가적인 부분들을 표현하기 위한 요소이다.

#### 그림 95 — `<fieldBegin>`의 구조

```
fieldBegin (필드 시작)
├── Attributes
│   ├── id (Type: xs:nonNegativeInteger) — 필드 시작을 구별하기 위한 아이디
│   ├── type (Type: hp:FieldType) — 필드 종류
│   ├── name (Type: xs:string) — 필드 이름
│   ├── editable (Type: xs:boolean, Default: true) — 읽기 전용 상태에서도 수정 가능한지 여부
│   ├── dirty (Type: xs:boolean, Default: false) — 필드 내용이 수정되었는지 여부
│   ├── zorder (Type: xs:integer) — Z-Order
│   └── fieldid (Type: xs:nonNegativeInteger) — 필드 객체 ID
├── parameters (Type: hp:ParameterList) — 필드 동작에 필요한 인자들
├── subList (Type: hp:ParaListType) — 내용. 특정 필드에서 사용됨
└── metaTag (Type: hc:MetaTagType) — 메타태그 관련 정보
```

#### 표 131 — fieldBegin 요소

| 속성 이름 | 설명                                    |
| --------- | --------------------------------------- |
| id        | 필드 시작을 구별하기 위한 아이디        |
| type      | 필드 종류                               |
| name      | 필드 이름                               |
| editable  | 읽기 전용 상태에서도 수정 가능한지 여부 |
| dirty     | 필드 내용이 수정되었는지 여부           |
| zorder    | Z-Order                                 |
| fieldid   | 필드 객체 ID                            |

#### 표 132 — fieldBegin 하위 요소

| 하위 요소 이름 | 설명                       |
| -------------- | -------------------------- |
| parameters     | 필드 동작에 필요한 인자들  |
| subList        | 내용. 특정 필드에서 사용됨 |
| metaTag        | 메타태그 관련 정보         |

#### 샘플 71 — fieldBegin 예

```xml
<hp:fieldBegin id="1795169102" type="CLICK_HERE" name="" editable="1" dirty="0" zorder="-1"
  fieldid="627272811">
  <hp:parameters cnt="3" name="">
    <hp:integerParam name="Prop">9</hp:integerParam>
    <hp:stringParam name="Command" xml:space="preserve">Clickhere:set:66:Direction:wstring:23:이곳을
마우스로 누르고 내용을 입력하세요. HelpState:wstring:0: </hp:stringParam>
    <hp:stringParam name="Direction">이곳을 마우스로 누르고 내용을 입력하세요.</hp:stringParam>
  </hp:parameters>
  <hp:metaTag>{"name":"#누름틀"}</hp:metaTag>
</hp:fieldBegin>
```

#### 10.7.2.2 CLICK_HERE

누름틀은 문서마당을 불러왔을 때 화면에 불린 문서마당의 빈 곳을 채워 넣을 안내문과 안내문에 대한 간단한 메모 내용을 입력하는 기능이다.

##### 10.7.2.2.1 필요한 인자들

#### 표 133 — CLICK_HERE 요소

| 인자 이름 | 인자 형식   | 설명          |
| --------- | ----------- | ------------- |
| Direction | stringParam | 안내문 문자열 |
| HelpState | stringParam | 안내문 도움말 |

#### 샘플 72 — CLICK_HERE 예

```xml
<fieldBegin id="fb01" type="CLICK_HERE" name="title" editable="true" dirty="false">
  <parameters count="2">
    <stringParam name="Direction">이 곳에 내용 입력</stringParam>
    <stringParam name="HelpState">제목</stringParam>
  </parameters>
</fieldBegin>
```

#### 10.7.2.3 HYPERLINK

##### 10.7.2.3.1 HYPERLINK

하이퍼링크는 문서의 특정한 위치에 현재 문서나 다른 문서, 웹 페이지, 전자우편 주소 등을 연결하여 쉽게 참조하거나 이동할 수 있게 해 주는 기능이다.

문서 내에서 그룹 객체를 사용할 경우 하이퍼링크 종류를 결정할 수 없는 경우가 발생할 수 있다. 각 개별 객체별로 하이퍼링크를 사용하고, 이 객체들을 하나의 그룹으로 묶을 경우 그룹 객체가 생성된다. 이때 생성된 그룹 객체는 그룹 내 객체들이 모두 같은 내용의 하이퍼링크 설정을 가지고 있지 않다면 하이퍼링크 종류, 하이퍼링크 대상, 문서창 옵션 종류를 결정할 수 없게 된다. 이런 경우 그룹 객체의 하이퍼링크 설정은 HWPHYPERLINK_TYPE_DONTCARE, HWPHYPERLINK_TARGET_DOCUMENT_DONTCARE, HWPHYPERLINK_JUMP_DONTCARE의 값을 가져야 한다.

##### 10.7.2.3.2 필요한 인자들

#### 표 134 — HYPERLINK 요소

| 인자 이름   | 인자 형식   | 설명                                                            |
| ----------- | ----------- | --------------------------------------------------------------- |
| Path        | stringParam | 링크 경로                                                       |
| Category    | stringParam | 하이퍼링크의 종류                                               |
| TargetType  | stringParam | 하이퍼링크의 종류가 한글 문서인 경우, 한글 문서에서 대상의 종류 |
| DocOpenType | stringParam | 이동 시 문서창 옵션                                             |

- **하이퍼링크의 종류**

#### 표 135 — 하이퍼링크 종류

| 하이퍼링크 종류            | 설명                                                                              |
| -------------------------- | --------------------------------------------------------------------------------- |
| HWPHYPERLINK_TYPE_DONTCARE | 동일 그룹 객체 내의 개별 객체들의 하이퍼링크 설정에서 하이퍼링크 종류가 다른 경우 |
| HWPHYPERLINK_TYPE_HWP      | HWP 문서 내부의 객체                                                              |
| HWPHYPERLINK_TYPE_URL      | 웹 주소                                                                           |
| HWPHYPERLINK_TYPE_EMAIL    | 메일 주소                                                                         |
| HWPHYPERLINK_TYPE_EX       | 외부 애플리케이션 문서                                                            |

- **HWP 문서에서 대상의 종류**

#### 표 136 — 대상의 종류

| HWP 문서에서 대상의 종류              | 설명                                                                          |
| ------------------------------------- | ----------------------------------------------------------------------------- |
| HWPHYPERLINK_TARGET_DOCUMENT_DONTCARE | 동일 그룹 객체 내의 개별 객체들의 하이퍼링크 설정에서 연결 문서가 다른 경우   |
| HWPHYPERLINK_TARGET_OBJECT_DONTCARE   | 동일 그룹 객체 내의 개별 객체들의 하이퍼링크 설정에서 책갈피 내용이 다른 경우 |
| HWPHYPERLINK_TARGET_BOOKMARK          | 책갈피                                                                        |
| HWPHYPERLINK_TARGET_OUTLINE           | 개요                                                                          |
| HWPHYPERLINK_TARGET_TABLE             | 표                                                                            |
| HWPHYPERLINK_TARGET_FIGURE            | 그림, 그리기 객체                                                             |
| HWPHYPERLINK_TARGET_EQUATION          | 수식                                                                          |
| HWPHYPERLINK_TARGET_HYPERLINK         | 하이퍼링크                                                                    |

- **이동 시 문서창 옵션**

#### 표 137 — 문서창 옵션

| 이동 시 문서창 옵션 종류     | 설명                                                                               |
| ---------------------------- | ---------------------------------------------------------------------------------- |
| HWPHYPERLINK_JUMP_DONTCARE   | 동일 그룹 객체 내의 개별 객체들의 하이퍼링크 설정에서 문서창 옵션 종류가 다른 경우 |
| HWPHYPERLINK_JUMP_CURRENTTAB | 현재 문서탭에서 열기                                                               |
| HWPHYPERLINK_JUMP_NEWTAB     | 새로운 문서탭에서 열기                                                             |
| HWPHYPERLINK_JUMP_NEWWINDOW  | 새로운 문서창에서 열기                                                             |

#### 샘플 73 — HYPERLINK 예

```xml
<fieldBegin id="fb02" type="HYPERLINK" editable="false" dirty="false">
  <parameters count="2">
    <stringParam name="Path">http://www.hancom.co.kr</stringParam>
    <stringParam name="Category">HWPHYPERLINK_TYPE_URL</stringParam>
    <stringParam name="TargetType">HWPHYPERLINK_TARGET_DOCUMENT_DONTCARE</stringParam>
    <stringParam name="DocOpenType">HWPHYPERLINK_JUMP_NEWTAB</stringParam>
  </parameters>
</fieldBegin>
```

#### 10.7.2.4 BOOKMARK

##### 10.7.2.4.1 BOOKMARK

두꺼운 책을 읽을 때 책의 중간에 책갈피를 꽂아 두고 필요할 때마다 들춰 보면 편리하듯이, [책갈피] 기능은 문서를 편집하는 도중에 본문의 여러 곳에 표시를 해 두었다가 현재 커서의 위치에 상관없이 표시해 둔 곳으로 커서를 곧바로 이동시키는 기능이다.

##### 10.7.2.4.2 XML 예

#### 샘플 74 — BOOKMARK 예

```xml
<fieldBegin id="fb03" type="BOOKMARK" name="bm01" editable="false" dirty="false"/>
```

#### 10.7.2.5 FORMULA

##### 10.7.2.5.1 FORMULA

표 계산식은 표에서 덧셈, 뺄셈, 곱셈, 나눗셈의 간단한 사칙연산은 물론이고, sum과 avg의 시트 함수와 sum(left) 등과 같은 left, right, below, above의 범위 지정자로 구성된 수식을 사용할 수 있게 하는 기능이다.

##### 10.7.2.5.2 필요한 인자들

#### 표 138 — FORMULA 요소

| 인자 이름         | 인자 형식   | 설명                   |
| ----------------- | ----------- | ---------------------- |
| FunctionName      | stringParam | 계산식 함수 이름       |
| FunctionArguments | listParam   | 계산식에 필요한 인자들 |
| ResultFormat      | stringParam | 결과 출력 형식         |
| LastResult        | stringParam | 마지막으로 계산된 결과 |

- **함수 목록**

#### 표 139 — FORMULA 함수 목록

| 함수 종류 | 설명                                                             |
| --------- | ---------------------------------------------------------------- |
| SUM       | 지정한 범위의 셀들에 대한 합을 계산                              |
| AVG       | 지정한 범위의 셀들에 대한 평균을 계산                            |
| PRODUT    | 지정한 범위의 셀들에 대한 곱(곱셈)계산                           |
| MIN       | 지정한 범위의 셀들 중 최소값을 찾음                              |
| MAX       | 지정한 범위의 셀들 중 최대값을 찾음                              |
| COUNT     | 지정한 범위의 셀들에 대해 공백이 아닌 셀의 수를 계산             |
| ROUND     | 하나의 셀에 대하여 지정한 자릿수에서 반올림                      |
| MOD       | 두 개의 셀에 대한 나눗셈의 나머지 계산                           |
| SQRT      | 하나의 셀에 대한 양의 제곱근을 계산                              |
| DEGTORAD  | 하나의 셀에 대한 도(일반각)를 라디안(호도법)으로 변환            |
| RADTODEG  | 하나의 셀에 대한 라디안(호도법)을 도(일반각)로 변환              |
| COS       | 하나의 셀에 대한 코사인 값 계산                                  |
| SIN       | 하나의 셀에 대한 사인 값 계산                                    |
| TAN       | 하나의 셀에 대한 탄젠트 값 계산                                  |
| ACOS      | 하나의 셀에 대한 아크 코사인 값 계산                             |
| ASIN      | 하나의 셀에 대한 아크 사인 값 계산                               |
| ATAN      | 하나의 셀에 대한 아크 탄젠트 값 계산                             |
| ABS       | 하나의 셀에 대한 절대값을 계산                                   |
| INT       | 하나의 셀에 대하여 소수점을 무시하고 정수 값만을 계산            |
| SIGN      | 하나의 셀에 대하여 양수 값이면 1, 0이면 0, 음수 값이면 -1로 계산 |
| CEILING   | 하나의 셀에 대하여 크거나 같은 최소 정수를 계산                  |
| FLOOR     | 하나의 셀에 대하여 작거나 같은 최대 정수를 계산                  |
| EXP       | 하나의 셀에 대한 자연 지수 e의 거듭 제곱 값을 계산               |
| LN        | 하나의 셀에 대한 자연 로그 값(밑이 자연 지수 e인 로그 값)을 계산 |
| LOG       | 하나의 셀에 대한 상용 로그 값(밑이 10인 로그 값)을 계산          |

- **함수 인자**

#### 표 140 — FORMULA 함수 인자

| 함수 인자 형태 | 설명                                                                                                  |
| -------------- | ----------------------------------------------------------------------------------------------------- |
| LEFT           | 현재 셀 왼쪽의 모든 셀                                                                                |
| RIGHT          | 현재 셀 오른쪽의 모든 셀                                                                              |
| ABOVE          | 현재 셀 위쪽의 모든 셀                                                                                |
| BELOW          | 현재 셀 아래쪽의 모든 셀                                                                              |
| 셀 주소        | A1, A2, B4 등과 같은 개별 셀 주소. 개별 셀 주소와 LEFT, RIGHT, ABOVE, BELOW는 혼합해서 사용할 수 없음 |

- **셀 번호**

커서를 움직여서 셀과 셀 사이를 이동하면 상황 선에 A1, A2, A3...과 같이 현재 커서가 놓여있는 셀의 이름이 표시된다. 즉 가로로는 A, B, C, D, E...의 순서로 이름이 정해지고, 세로로는 1, 2, 3, 4, 5...와 같은 순서로 이름이 정해진다.

#### 표 141 — 셀 번호

| A1  | B1  | C1  | D1  | E1  |
| --- | --- | --- | --- | --- |
| A2  | B2  | C2  | D2  | E2  |
| A3  | B3  | C3  | D3  | E3  |
| A4  | B4  | C4  | D4  | E4  |
| A5  | B5  | C5  | D5  | E5  |

- **결과 출력 형식**

#### 표 142 — 결과 출력 형식

| 결과 출력 형식 | 설명                         |
| -------------- | ---------------------------- |
| %g             | 기본 형식                    |
| %.0f           | 정수형                       |
| %.1f           | 소수점 이하 1자리까지만 표시 |
| %.2f           | 소수점 이하 2자리까지만 표시 |
| %.3f           | 소수점 이하 3자리까지만 표시 |
| %.4f           | 소수점 이하 4자리까지만 표시 |

##### 10.7.2.5.3 XML 예

#### 샘플 75 — FORMULA 예

```xml
<fieldBegin id="fb04" type="FORMULA" editable="false" dirty="false">
  <parameters count="4">
    <stringParam name="FunctionName">SUM</stringParam>
    <listParam name="FunctionArguments" cnt="1">
      <stringParam>LEFT</stringParam>
    </listParam>
    <stringParam name="ResultFormat">%g</stringParam>
    <stringParam name="LastResult">77</stringParam>
  </parameters>
</fieldBegin>
```

#### 10.7.2.6 DATE 및 DOC_DATE

날짜/시간 표시. DATE 형식은 하위 호환성을 위해 남겨둔 형식이다. DATE 형식은 되도록 사용하지 않는 것을 권고한다.

##### 10.7.2.6.1 필요한 인자들

#### 표 143 — DATE 요소

| 인자 이름  | 인자 형식   | 설명                |
| ---------- | ----------- | ------------------- |
| DateNation | stringParam | 국가 코드           |
| DateFormat | stringParam | 날짜/시간 표시 형식 |

- **국가 코드**

국가 코드는 기본적으로 ISO 국가 코드 표기법(ISO 3166-1의 alpha-3)을 따른다. 단 모든 국가를 지원하지 않고 다음의 다섯 개 국가의 날짜/시간 형태만을 지원한다.

#### 표 144 — 국가 코드

| 국가 코드 | 설명     |
| --------- | -------- |
| KOR       | 대한민국 |
| USA       | 미국     |
| JPN       | 일본     |
| CHN       | 중국     |
| TWN       | 대만     |

- **날짜/시간 표시 형식**

날짜/시간 표시 형식은 기본적으로 ISO 날짜/시각 표기법(KS X ISO 8601 참조)을 따른다. 단, ISO 날짜/시각 표기법에서 지원하지 않는 표시 형식은 확장해서 사용한다. ISO 날짜/시각 표기법의 자세한 내용은 표준을 참조한다. 이 문서에서는 표준의 간략한 내용과 확장한 내용만을 설명한다.

#### 표 145 — 날짜/시간 표시 기호

| 날짜/시간 표시 기호 | 설명                                                                                                                                                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Y                   | 년(year) 요소를 표현                                                                                                                                                                                                                        |
| M                   | 월(month) 요소를 표현. M으로 표기된 경우 1과 같이 한 자리 수로 표현, MM으로 표현된 경우 01과 같이 2자리 수로 표현, MMM으로 표현된 경우 Jan과 같이 축약된 영어식 표현, MMMMMMMM으로 표현된 경우 January와 같이 영어 전체 단어를 표현         |
| D                   | 일(day) 요소를 표현                                                                                                                                                                                                                         |
| w                   | 주(week) 요소를 표현. 해당 연도에서 몇 번째 주인지 숫자로 표현. ex) 금주는 w번째 주이다. => 금주는 16번째 주이다.                                                                                                                           |
| h                   | 시(hour) 요소를 표현. 24시간제(0 ~ 23)                                                                                                                                                                                                      |
| m                   | 분(minute) 요소를 표현                                                                                                                                                                                                                      |
| s                   | 초(second) 요소를 표현                                                                                                                                                                                                                      |
| n                   | 0 또는 양의 정수를 표현                                                                                                                                                                                                                     |
| ±                   | [+]                                                                                                                                                                                                                                         |
| E                   | 확장 요소. 요일(day of the week) 요소를 표현. 국가 코드에 따라서 표현이 다름. 대한민국의 경우 월/화/수/목/금/토/일, 미국의 경우 Monday/Tuesday/Wednesday/Thursday/Friday/Saturday/Sunday, 일본/중국/대만의 경우 月/火/水/木/金/土/日로 표현 |

## 10.7.2.6.2 필요한 인자들

### 표 145 — 날짜/시간 표시 기호 (계속)

| 날짜/시간 표시 기호 | 설명                                                                                                                                                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| a                   | 오전 요소<br/>오전/오후 요소를 표현<br/>국가 코드에 따라서 표현이 다름<br/>영문권의 경우 오전/오후, 미국의 경우 AM/PM, 일본의 경우 午前/午後,<br/>중국/대만의 경우 上午/下午 표현 |
| A                   | 위치 요소<br/>A.M./P.M. 요소를 표현<br/>국가 코드에 상관없이 A.M./P.M. 을 출력하도록 표현                                                                                         |
| I                   | 위치 요소<br/>연호/주덕 요소를 표현<br/>일본의 경우 干支, 대만의 경우 民国, 그 외의 지역은 해당 요소는 무시                                                                       |
| L                   | 위치 요소<br/>연호/주덕의 연도 요소를 표현<br/>일본/대만의 경우 각 나라의 연호/주덕에 맞는 연도가 표시되고, 그 외의<br/>지역은 y 요소와 동일하게 표현                             |
| k                   | 위치 요소<br/>시(hour) 요소를 표현<br/>12시간제(1~12)                                                                                                                             |

### 표 146 — 날짜/시간 표시

| 형식                  | 예                     |
| --------------------- | ---------------------- |
| YYYY-MM-DD hh:mm:ss   | 2011-01-01 01:00:00    |
| YYYY년 M월 D일 E요일  | 2011년 1월 1일 토요일  |
| a k:mm                | 오전 1:00              |
| YYYY 年 M 月 1 日 (B) | 2011 年 1 月 1 日 (六) |
| MMMMMMMM D, YYYY      | January 1, 2011        |
| 1L년 1월 1일          | 평성 23년 1월 1일      |

### 예를 76 — DOC_DATE 예

```xml
<fieldBegin id="fb05" type="DOC_DATE" editable="false" dirty="false">
  <parameters count="2">
    <stringParam name="DateNation">KOR</stringParam>
    <stringParam name="DateFormat">YYYY-MM-DD hh:mm:ss</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.7 SUMMARY

### 10.7.2.7.1 Summary

문서 요약 정보는 현재 문서에 대한 제목, 주제, 저작이, 중심 낱말(키워드), 저자, 압력자, 교정자, 내용 요약, 주석사항 등을 간단히 기록할 수 있는 기능이다.

### 10.7.2.7.2 필요한 인자들

### 표 147 — SUMMARY 요소

| 인자 이름 | 인자 형식   | 설명                |
| --------- | ----------- | ------------------- |
| Property  | stringParam | 문서 요약 정보 속성 |

#### • 문서 요약 속성

### 표 148 — 문서 요약 요소

| 국가 코드     | 설명                          |
| ------------- | ----------------------------- |
| $title        | 문서 제목                     |
| $subject      | 문서 주제                     |
| $author       | 문서 저자                     |
| $keywords     | 문서 키워드                   |
| $comments     | 문서 주석                     |
| $lastAuthor   | 문서 마지막 수정한 사람       |
| $revNumber    | 문서 어떤 번호                |
| $lastPrinted  | 문서가 마지막으로 출력된 시각 |
| $createDate   | 문서가 생성된 시각            |
| $lastSaveDate | 문서가 마지막으로 저장된 시각 |
| $pageCount    | 문서 페이지 수                |
| $wordCount    | 문서 단어 수                  |
| $charCount    | 문서 글자 수                  |

### 10.7.2.7.3 XML 예

### 예를 77 — SUMMARY 예

```xml
<fieldBegin id="fb06" type="SUMMARY" editable="false" dirty="false">
  <parameters count="1">
    <stringParam name="Property">$title</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.8 USE_INFO

### 10.7.2.8.1 USE_INFO

사용자 정보는 현재 문서의 작성자에 대한 이름, 회사명, 전화번호 등을 간단히 기록할 수 있는 기능이다.

### 10.7.2.8.2 필요한 인자들

### 표 149 — USE_INFO 요소

| 인자 이름 | 인자 형식   | 설명                  |
| --------- | ----------- | --------------------- |
| Category  | stringParam | 사용자 전보 정보 항목 |

#### • 사용자 정보 정보 항목

### 표 150 — 사용자 정보 항목

| 국가 코드        | 설명                    |
| ---------------- | ----------------------- |
| $UserName        | 사용자 이름             |
| $Company         | 회사 이름               |
| $Department      | 부서 이름               |
| $Position        | 직책 이름               |
| $OfficeTelephone | 회사 전화번호           |
| $Fax             | 팩스 번호               |
| $HomeTelephone   | 집 전화번호             |
| $Mobilephone     | 휴대폰 번호             |
| $UMS1            | UMS 번호 1              |
| $UMS2            | UMS 번호 2              |
| $Homepage        | 홈페이지 주소           |
| $Email1          | 전자우편 주소 1         |
| $Email2          | 전자우편 주소 2         |
| $Email3          | 전자우편 주소 3         |
| $OfficeZipcode   | 회사 우편번호           |
| $OfficeAddress   | 회사 주소               |
| $HomeZipcode     | 집 우편번호             |
| $HomeAddress     | 집 주소                 |
| $Etc             | 기타                    |
| $UserDefineName  | 사용자 정의 아이템 이름 |
| $UserDefineValue | 사용자 정의 아이템 값   |

### 10.7.2.8.3 XML 예

### 예를 78 — USER_INFO 예

```xml
<fieldBegin id="fb07" type="USER_INFO" editable="false" dirty="false">
  <parameters count="1">
    <stringParam name="Category">$UserName</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.9 PATH

### 10.7.2.9.1 Path

현재 문서의 물리적인 파일 경로를 문서에 표시해 주는 기능이다.

### 10.7.2.9.2 필요한 인자들

### 표 151 — PATH 요소

| 인자 이름 | 인자 형식   | 설명           |
| --------- | ----------- | -------------- |
| Format    | stringParam | 파일 경로 형식 |

#### • 파일 경로 형식

### 표 152 — 파일 경로 형식

| 국가 코드 | 설명      |
| --------- | --------- |
| $P        | 파일 경로 |
| $F        | 파일 이름 |

### 10.7.2.9.3 XML 예

### 예를 79 — PATH 예

```xml
<fieldBegin id="fb08" type="PATH" editable="false" dirty="false">
  <parameters count="1">
    <stringParam name="Format">$P$F</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.10 CROSSREF

### 10.7.2.10.1 CROSSREF

상호 참조는 다른 쪽의 그림, 표 등을 현재의 문서에서 참상 참조할 수 있도록 그 위치를 표시해 주는 기능이다.

### 10.7.2.10.2 필요한 인자들

### 표 153 — CROSSREF 요소

| 인자 이름      | 인자 형식    | 설명                                                                                           |
| -------------- | ------------ | ---------------------------------------------------------------------------------------------- |
| RefPath        | stringParam  | 참조 경로                                                                                      |
| RefType        | stringParam  | 참조 대상 종류                                                                                 |
| RefContentType | stringParam  | 참조 내용                                                                                      |
| RefHyperLink   | booleanParam | 하이퍼링크 여부                                                                                |
| RefOpenType    | stringParam  | 하이퍼링크 이동 시 문서를 열리 옵션<br/>참조 경로가 현재 문서가 아닐 외부 문서인 경우에 사용됨 |

#### • 참조 경로 형식

### 표 154 — 참조 경로 형식

| 분류                  | 형식                                                  |
| --------------------- | ----------------------------------------------------- |
| 외부 문서 참조인 경우 | {문서의 파일 경로}?#{참조 대상의 ID 또는 책갈피 이름} |
| 현재 문서 참조인 경우 | ?#{참조 대상의 ID 또는 책갈피 이름}                   |

#### • 참조 대상 종류

### 표 155 — 참조 대상 종류

| 참조 대상 종류  | 설명   |
| --------------- | ------ |
| TARGET_TABLE    | 표     |
| TARGET_PICTURE  | 그림   |
| TARGET_EQUATION | 수식   |
| TARGET_FOOTNOTE | 각주   |
| TARGET_ENDNOTE  | 미주   |
| TARGET_OUTLINE  | 개요   |
| TARGET_BOOKMARK | 책갈피 |

#### • 참조 내용

### 표 156 — 참조 내용

| 참조 내용             | 설명                                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------------------------- |
| OBJECT_TYPE_PAGE      | 참조 대상이 있는 쪽 번호                                                                                |
| OBJECT_TYPE_NUMBER    | 참조 대상의 번호                                                                                        |
| OBJECT_TYPE_CONTENTS  | 참조 대상의 제시 내용 또는 책갈피의 경우, 책갈피 내용<br/>미주/각주의 경우 해당 텍스트를 사용할 수 없음 |
| OBJECT_TYPE_UPDOWNPOS | 현재 위치 기준으로 참조 대상이 있는 위치(위/아래)                                                       |

### 10.7.2.10.3 XML 예

### 예를 80 — CROSSREF 예

```xml
<fieldBegin id="fb09" type="CROSSREF" editable="false" dirty="false">
  <parameters count="5">
    <stringParam name="RefPath">?#table23</stringParam>
    <stringParam name="RefType">TARGET_TABLE</stringParam>
    <stringParam name="RefContentType">OBJECT_TYPE_NUMBER</stringParam>
    <booleanParam name="RefHyperLink">true</booleanParam>
    <stringParam name="RefOpenType">HYPERLINK_JUMP_DONTCARE</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.11 MAILMERGE

### 10.7.2.11.1 MAILMERGE

메일 머지는 여러 사람의 이름, 주소 등이 들어 있는 '데이터 파일'(data file)과 '서식 파일'(form letter file)을 결합함(merging)으로써, 이름이나 직책, 주소 부분 등만 다르고 나머지 내용은 같은 수십, 수백 통의 편지자료를 한꺼번에 만드는 기능이다.

### 10.7.2.11.2 필요한 인자들

### 표 157 — MAILMERGE 요소

| 인자 이름  | 인자 형식   | 설명                                             |
| ---------- | ----------- | ------------------------------------------------ |
| FieldType  | stringParam | WAB, USER_DEFINE 중 하나의<br/>필드 개성 수 있음 |
| FieldValue | stringParam | 필드 엔트리 이름                                 |

#### • 필드 엔트리 이름

필드 형식이 USER_DEFINE인 경우 필드의 정체된 이름 규칙은 없다.

필드 형식이 WAB인 경우에는 다음의 이름만을 사용해야 한다.

### 표 158 — 필드 엔트리 이름

| 참조 대상 종류                     | 설명                                             |
| ---------------------------------- | ------------------------------------------------ |
| ENTRYID                            | Windows Address Book에서 각 엔트리의 고유 아이디 |
| OBJECT_TYPE                        | 엔트리 객체 형식                                 |
| DISPLAY_NAME                       | 사용자 표시 이름                                 |
| SURNAME                            | 사용자 성                                        |
| GIVEN_NAME                         | 사용자 이름                                      |
| NICKNAME                           | 사용자 애칭                                      |
| TITLE                              | 직책                                             |
| COMPANY_NAME                       | 회사 이름                                        |
| DEPARTMENT_NAME                    | 부서 이름                                        |
| SPOUSE_NAME                        | 배우자 이름                                      |
| MOBILE_TELEPHONE_NUMBER            | 휴대폰 번호                                      |
| PAGER_TELEPHONE_NUMBER             | 호출기 번호                                      |
| EMAIL_ADDRESS                      | 전자우편 주소                                    |
| HOME_ADDRESS_COUNTRY               | 집 주소 국가/지역                                |
| HOME_ADDRESS_STATE_OR_PROVINCE     | 집 주소 시/도                                    |
| HOME_ADDRESS_CITY                  | 집 주소 구/군/시                                 |
| HOME_ADDRESS_STREET                | 집 주소 나머지                                   |
| HOME_TELEPHONE_NUMBER              | 집 전화번호                                      |
| HOME_FAX_NUMBER                    | 집 팩스 번호                                     |
| HOME_ADDRESS_POSTAL_CODE           | 집 주소 우편 번호                                |
| BUSINESS_ADDRESS_COUNTRY           | 직장 주소 국가/지역                              |
| BUSINESS_ADDRESS_STATE_OR_PROVINCE | 직장 주소 시/도                                  |
| BUSINESS_ADDRESS_CITY              | 직장 주소 구/군/시                               |
| BUSINESS_ADDRESS_STREET            | 직장 주소 나머지                                 |
| BUSINESS_TELEPHONE_NUMBER          | 직장 전화 번호                                   |
| BUSINESS_FAX_NUMBER                | 직장 팩스 번호                                   |
| BUSINESS_ADDRESS_POSTAL_CODE       | 직장 주소 우편 번호                              |

### 10.7.2.11.3 XML 예

### 예를 81 — MAILMERGE 예

```xml
<fieldBegin id="fb10" type="MAILMERGE" editable="false" dirty="false">
  <parameters count="2">
    <stringParam name="FieldType">WAB</stringParam>
    <stringParam name="FieldValue">SURNAME</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.12 MEMO

| -------------- | ------------ | ----------------------------------------------------------------------- |
| ID | stringParam | 메모를 식별하기 위한 아이디 |
| Number | integerParam | 메모 번호 |
| CreateDateTime | stringParam | 메모 작성 시각<br/>KS X ISO 8601에 따라 "YYYY-MM-DD hh:mm:ss" 형식 사용 |
| Author | stringParam | 메모 작성자 |
| MemoShapeIDRef | stringParam | 메모 모양 설정 정보 아이디 참조값 |

### 10.7.2.12.3 XML 예

### 샘플 82 — MEMO 예

```xml
<fieldBegin id="fb11" type="MEMO" editable="true" dirty="true">
  <parameters count="5">
    <stringParam name="ID">memo1</stringParam>
    <integerParam name="Number">1</integerParam>
    <stringParam name="CreateDateTime">2011-01-01 10:00:00</stringParam>
    <stringParam name="Author">hancom</stringParam>
    <stringParam name="MemoShapeID">memoShape3</stringParam>
  </parameters>
  <subList id="subList2" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP"
    linkListIDRef="subList1" linkListNextIDRef="subList1">
    <p id="para21" paraPrIDRef="pshape2" styleIDRef="style6" pageBreak="false" columnBreak="false">
      <t charPrIDRef="cshape5">
        <char>메모 내용</char>
      </t>
    </p>
  </subList>
</fieldBegin>
```

## 10.7.2.13 PROOFREADING_MARKS

### 10.7.2.13.1 PROOFREADING_MARKS

교정 부호는 맞춤법, 띄어쓰기, 활자 크기, 문장 부호, 줄바꿈, 오자, 탈자, 어색한 표현 등을 바로잡기 위하여 특정 부호를 문서 내에 삽입하는 기능이다.

### 10.7.2.13.2 필요한 인자들

교정 부호 종류가 "메모 고침표"인 경우 MEMO 형식에서 사용되는 인자들을 사용한다. 즉, Type, Number, CreateDateTime, Author, MemoShapeIDRef 인자들을 사용한다. Type을 제외한 나머지 인자들에 대한 자세한 설명은 **10.7.2.12**를 참조한다.

교정 부호 종류가 "자료 연결"인 경우 HYPERLINK 형식에서 사용되는 인자들을 사용한다. 즉, Type, Path, Category, TargetType, DocOpenType 인자들이 사용된다. Type을 제외한 나머지 인자들에 대한 자세한 설명은 **10.7.2.3**을 참조한다.

### 표 160 — PROOFREADING_MARKS 요소

| 인자 이름            | 인자 형식    | 설명                                                                                          |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------- |
| Type                 | stringParam  | 교정 부호 종류                                                                                |
| ProofreadingContents | stringParam  | 교정 내용<br/>넣음표, 부호 넣음표, 고침표에서 사용됨                                          |
| MovingMargin         | integerParam | 자리 옮김 여백<br/>오른/왼자리 옮김표에서 사용됨                                              |
| MovingStart          | integerParam | 자리 옮김 시작위치<br/>오른/왼자리 옮김표에서 사용됨                                          |
| SplitType            | stringParam  | "자리 바꿈 나눔표"인지 "줄 서로 바꿈 나눔표"인지 여부<br/>자리/줄 서로 바꿈 나눔표에서 사용됨 |

#### • 교정 부호 종류

### 표 161 — 교정 부호 종류

| 참조 내용        | 설명            |
| ---------------- | --------------- |
| WORD_SPACING     | 띄움표          |
| CONTENT_INSERT   | 넣음표          |
| SIGN_INSERT      | 부호 넣음표     |
| LINE_SPLIT       | 줄바꿈표        |
| LINE_SPACE       | 줄비움표        |
| MEMO_CHANGE      | 메모 고침표     |
| SIMPLE_CHANGE    | 고침표          |
| CLIPPING         | 뺌표            |
| DELETE           | 지움표          |
| ATTACH           | 붙임표          |
| LINE_ATTACH      | 줄붙임표        |
| LINE_LINK        | 줄이음표        |
| SAWTOOTH         | 톱니표          |
| THINKING         | 생각표          |
| PRAISE           | 칭찬표          |
| LINE             | 줄표            |
| POSITON_TRANSFER | 자리 바꿈표     |
| LINE_TRANSFER    | 줄 서로 바꿈표  |
| TRANSFER_SPLIT   | 바꿈 나눔표     |
| RIGHT_MOVE       | 오른자리 옮김표 |
| LEFT_MOVE        | 왼자리 옮김표   |
| LINK_DATA        | 자료 연결       |

#### • SplitType

### 표 162 — SplitType 요소

| 참조 내용 | 설명                       |
| --------- | -------------------------- |
| POSITION  | 자리 바꿈 나눔표를 지정    |
| LINE      | 줄 서로 바꿈 나눔표를 지정 |

### 10.7.2.13.3 XML 예

### 샘플 83 — PROOFREADING_MARKS 예

```xml
<fieldBegin id="fb12" type="PROOFREADING_MARKS" editable="false" dirty="true">
  <parameters count="2">
    <stringParam name="Type">SIMPLE_CHANGE</stringParam>
    <integerParam name="ProofreadingContents">고침표 내용</integerParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.14 PRIVATE_INFO

### 10.7.2.14.1 PRIVATE_INFO 요소

선택 글자 보호는 현재 화면에서 편집하고 있는 문서 내용 중 사용자가 블록으로 지정한 영역을 암호를 걸어 사용자가 선택한 문자로 변경하는 기능이다.

### 10.7.2.14.2 필요한 인자들

### 표 163 — PRIVATE_INFO 요소

| 인자 이름     | 인자 형식    | 설명                                      |
| ------------- | ------------ | ----------------------------------------- |
| EncryptMode   | stringParam  | 암호화 방식                               |
| EncryptLength | integerParam | 암호화된 결과의 길이                      |
| DecryptLength | integerParam | 복호화한 후의 길이                        |
| EncryptString | stringParam  | 암호화된 결과를 BASE64로 인코딩한 문자열  |
| MarkChar      | stringParam  | 암호화된 문자열 대신에 화면에 표시될 문자 |
| Pattern       | stringParam  | Pattern                                   |
| Type          | stringParam  | Type                                      |

#### • 암호화 방식

### 표 164 — 암호화 방식

| 참조 내용 | 설명                                       |
| --------- | ------------------------------------------ |
| AES       | AES(Advanced Encryption Standard) 알고리즘 |

### 10.7.2.14.3 XML 예

### 샘플 84 — PRIVATE_INFO 예

```xml
<fieldBegin id="fb13" type="PRIVATE_INFO" editable="false" dirty="true">
  <parameters count="5">
    <stringParam name="EncryptMode">AES</stringParam>
    <integerParam name="EncryptLength">80</integerParam>
    <integerParam name="DecryptLength">35</integerParam>
    <stringParam
      name="EncryptString">fgtM4BN7AzseLJHkYEfC7hjjH/OZ3fJXm30S8vmPfMWTI2odMR4YGk2zlmov4
      NUj8w99wczISLtzi8BZDPdIHfEbSkJZKAwhYNCot2jjvQk=</stringParam>
    <stringParam name="MarkChar">*</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.15 METADATA

### 10.7.2.15.1 METADATA

특정 단어나 블록으로 설정한 문자열에 대한 추가적인 의미 정보를 기록하는 기능이다. 사용하는 인자의 값인 Property, Resource, Content, Datatype의 자세한 내용은 RDFa의 xhtml:property, xhtml:resource, xhtml:content, xhtml:datatype을 참고한다.

세부적인 규격은 RDFa를 참고한다("http://www.w3.org/TR/2008/REC-rdfa-syntax-20081014/").

### 10.7.2.15.2 필요한 인자들

### 표 165 — METADATA 요소

| 인자 이름 | 인자 형식   | 설명                   |
| --------- | ----------- | ---------------------- |
| ID        | stringParam | 고유 식별 아이디       |
| Property  | stringParam | 주제(subject)와의 관계 |
| Resource  | stringParam | 참조되는 URI           |
| Content   | stringParam | 문자열                 |
| Datatype  | stringParam | Content의 데이터형     |

### 10.7.2.15.3 XML 예

### 샘플 85 — METADATA 예

```xml
<fieldBegin id="fb13" type="METADATA" editable="false" dirty="true">
  <parameters count="4">
    <stringParam name="ID">103e9eab2c70</stringParam>
    <stringParam name="Property">http://www.w3.org/2002/12/cal/ical/dtstart</stringParam>
    <stringParam name="Content">2007-09-16T16:00:00-05:00</stringParam>
    <stringParam name="Datatype">xsd:dateTime</stringParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.16 CITATION

### 10.7.2.16.1 CITATION

인용은 연구논문이나 다른 여러의 원본을 인용해야 하는 문서를 작성할 때 사용하는 기능이다. 인용은 다양한 형식의 인용 스타일을 선택하여 적용할 수 있다.

### 10.7.2.16.2 필요한 인자들

### 표 166 — CITATION 요소

| 인자 이름 | 인자 형식   | 설명                        |
| --------- | ----------- | --------------------------- |
| GUID      | stringParam | 인용 고유 번호              |
| Result    | stringParam | 스타일이 적용된 인용 문자열 |

### 10.7.2.16.3 XML 예

### 샘플 86 — CITATION 예

```xml
<fieldBegin id="fb13" type="CITATION" editable="false" dirty="true">
  <parameters count="2">
    <stringParam name="GUID">A25C5BE1-391D-4088-9B2C-3E0C521730F1</stringParam>
    <integerParam name="Result">공연_작가_퍼스트공연_작가_라스트, 공연_작가_퍼레공연_작가_라, 공연_작
      가_퍼퍼공연_작가_라라 1948</integerParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.17 BIBLIOGRAPHY

### 10.7.2.17.1 BIBLIOGRAPHY

참고문헌은 참조한 원본에 대한 출처 정보를 적용하는 기능이다. 참고문헌 스타일을 선택하거나 다른 참고문헌 스타일을 적용할 수 있다. 참고문헌에 대한 xml 데이터는 OOXML의 형식을 사용하며 Custom/Bibliography.xml (8.2 참조)에 기입된다. 해당 데이터는 참고문헌 스타일에 의해 표현된다.

### 10.7.2.17.1 필요한 인자들

### 표 167 — BIBLIOGRAPHY 요소

| 인자 이름    | 인자 형식   | 설명                 |
| ------------ | ----------- | -------------------- |
| StyleName    | stringParam | 참고문헌 스타일      |
| StyleVersion | stringParam | 참고문헌 스타일 버전 |

### 10.7.2.17.2 XML 예

### 샘플 87 — BIBLIOGRAPHY 예

```xml
<fieldBegin id="fb13" type="BIBLIOGRAPHY" editable="false" dirty="true">
  <parameters count="2">
    <stringParam name="StyleName">APA</stringParam>
    <integerParam name="StyleVersion">6</integerParam>
  </parameters>
</fieldBegin>
```

## 10.7.2.18 METATAG

메타태그는 본문의 메타 정보를 기록하는 기능이다.

### 샘플 88 — METATAG 예

```xml
<fieldBegin id="fb13" type="METATAG" editable="false" dirty="true" zorder="1">
  <hp:metaTag>{"name":"#전화번호"}</hp:metaTag>
</fieldBegin>
```

## 10.7.3 fieldEnd 요소

`<fieldBegin>` 요소와 짝을 이루는 요소이다.

### 표 168 — fieldEnd 요소

| 속성 이름  | 설명                    |
| ---------- | ----------------------- |
| beginIDRef | 필드 시작 아이디 참조값 |
| fieldid    | 필드 객체 아이디        |

### 샘플 89 — fieldEnd 예

```xml
<hp:fieldEnd beginIDRef="1790845288" fieldid="623209829"/>
```

## 10.7.4 bookmark 요소

필드에서 사용되는 책갈피와는 다른 구조를 가지는 책갈피를 표현하기 위한 요소이다. 필드의 책갈피는 지정된 구역에 책갈피 표시를 하지만, `<bookmark>` 요소를 사용한 책갈피는 지정된 구역을 가지지 않는 단순히 지정된 위치에 책갈피 표시를 한다.

### 표 169 — bookmark 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| name      | 책갈피 이름 |

### 샘플 90 — bookmark 예

```xml
<hp:bookmark name="책갈피"/>
  <hp:header id="1" applyPageType="BOTH">
    <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP" linkListIDRef="0"
      linkListNextIDRef="0" textWidth="42520" textHeight="4252" hasTextRef="0" hasNumRef="0">
      <hp:p id="0" paraPrIDRef="21" styleIDRef="13" pageBreak="0" columnBreak="0" merged="0"
        paraTcId="12">
        <hp:run charPrIDRef="1">
          <hp:t>
            <hp:insertBegin Id="36" TcId="11"/>
          </hp:t>
          <hp:ctrl>
            <hp:autoNum num="1" numType="PAGE">
              <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="suffixC har=[] supscript = "0"/>
            </hp:autoNum>
          </hp:ctrl>
          <hp:t/>
        </hp:run>
        <hp:linesegarray>
          <hp:lineseg textpos="0" vertpos="0" vertsize="900" textheight="900" baseline="765" spacing="452"
            horzpos="0" horzsize="42520" flags="393216"/>
        </hp:linesegarray>
      </hp:p>
    </hp:subList>
  </hp:header>
```

## 10.7.5 머리말/꼬리말 요소 형식

머리말 및 꼬리말을 표현하기 위한 요소 형식이다.

### 표 170 — HeaderFooterType 요소

| 속성 이름     | 설명                                                                               |
| ------------- | ---------------------------------------------------------------------------------- |
| id            | 머리말/꼬리말을 식별하기 위한 아이디                                               |
| applyPageType | 머리말/꼬리말이 적용될 페이지 형식<br/>BOTH: 양쪽<br/>EVEN: 짝수쪽<br/>ODD: 홀수쪽 |

### 표 171 — HeaderFooterType 하위 요소

| 하위 요소 이름 | 설명               |
| -------------- | ------------------ |
| subList        | 머리말/꼬리말 내용 |

### 샘플 91 — HeaderFooterType 예

```xml
<hp:footer id="3" applyPageType="BOTH">
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="BOTTOM" linkListIDRef="0"
    linkListNextIDRef="0" textWidth="42520" textHeight="4252" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="22" styleIDRef="13" pageBreak="0" columnBreak="0" merged="0" paraTcId="12">
      <hp:run charPrIDRef="1">
        <hp:t>
          <hp:insertBegin Id="40" TcId="11"/>
        </hp:t>
        <hp:ctrl>
          <hp:autoNum num="1" numType="PAGE">
            <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar="" supscript="0"/>
          </hp:autoNum>
        </hp:ctrl>
        <hp:t/>
        <hp:ctrl>
          <hp:tab width="31188" leader="0" type="2"/>
        </hp:ctrl>
        <hp:ctrl>
          <hp:fieldBegin id="1790879954" type="PATH" name="" editable="0" dirty="0" zorder="-1"
            fieldid="628121972" metaTag="">
            <hp:parameters cnt="3" name="">
              <hp:integerParam name="Prop">8</hp:integerParam>
              <hp:stringParam name="Command">$F</hp:stringParam>
              <hp:stringParam name="Format">$F</hp:stringParam>
            </hp:parameters>
          </hp:fieldBegin>
        </hp:ctrl>
        <hp:t>테스트 문서입니다.owpml</hp:t>
        <hp:ctrl>
          <hp:fieldEnd beginIDRef="1790879954" fieldid="628121972"/>
        </hp:ctrl>
        <hp:t/>
      </hp:run>
      <hp:linesegarray>
        <hp:lineseg textpos="0" vertpos="0" vertsize="900" textheight="900" baseline="765" spacing="452"
          horzpos="0" horzsize="42520" flags="393216"/>
      </hp:linesegarray>
    </hp:p>
  </hp:subList>
</hp:footer>
```

## 10.7.6 각주/미주 요소 형식

각주 및 미주를 표현하기 위한 요소 형식이다.

### 표 172 — NoteType 요소

| 속성 이름 | 설명                             |
| --------- | -------------------------------- |
| id        | 각주/미주를 식별하기 위한 아이디 |

### 표 173 — NoteType 하위 요소

| 하위 요소 이름 | 설명           |
| -------------- | -------------- |
| subList        | 각주/미주 내용 |

### 샘플 92 — NoteType 예

```xml
<hp:footNote instId="1832523497">
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP"
    linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="10" styleIDRef="14" pageBreak="0" columnBreak="0" merged="0">
      <hp:run charPrIDRef="3">
        <hp:ctrl>
          <hp:autoNum num="1" numType="FOOTNOTE">
            <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=")"
              supscript="0"/>
          </hp:autoNum>
        </hp:ctrl>
        <hp:t> </hp:t>
      </hp:run>
    </hp:p>
  </hp:subList>
</hp:footNote>
```

## 10.7.7 자동/새 번호 요소 형식

자동 번호 및 새 번호를 표현하기 위한 요소 형식이다.

### 표 174 — AutoNumNewNumType 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| num       | 번호        |
| numType   | 번호의 종류 |

### 표 175 — AutoNumNewNumType 하위 요소

| 하위 요소 이름 | 설명                        |
| -------------- | --------------------------- |
| autoNumFormat  | 번호 서식<br/>10.6.7.2 참조 |

### 샘플 93 — AutoNumNewNumType 예

```xml
<hp:autoNum num="1" numType="PAGE">
  <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar="" supscript="0"/>
</hp:autoNum>
```

## 10.7.8 pageNumCtrl 요소

쪽 번호를 홀수쪽, 짝수쪽 또는 양쪽 모두에 표시할지를 설정하기 위한 요소이다.

### 표 176 — pageNumCtrl 요소

| 속성 이름    | 설명         |
| ------------ | ------------ |
| pageStartsOn | 홀/짝수 구분 |

## 10.7.9 pageHiding 요소

현재 구역 내에서 감추어야 할 것들을 설정하기 위한 요소이다.

### 표 177 — pageHiding 요소

| 속성 이름      | 설명                |
| -------------- | ------------------- |
| hideHeader     | 머리말 감추기 여부  |
| hideFooter     | 꼬리말 감추기 여부  |
| hideMasterPage | 바탕쪽 감추기 여부  |
| hideBorder     | 테두리 감추기 여부  |
| hideFill       | 배경 감추기 여부    |
| hidePageNum    | 쪽 번호 감추기 여부 |

### 샘플 94 — pageHiding 예

```xml
<hp:pageHiding hideHeader="0" hideFooter="0" hideMasterPage="0" hideBorder="0" hideFill="1"
  hidePageNum="0"/>
```

## 10.7.10 pageNum 요소

쪽 번호의 위치 및 모양을 설정하기 위한 요소이다.

### 표 178 — pageNum 요소

| 속성 이름  | 설명           |
| ---------- | -------------- |
| pos        | 번호 위치      |
| formatType | 번호 모양 종류 |
| sideChar   | 줄표 넣기      |

### 샘플 95 — pageNum 예

```xml
<hp:pageNum pos="BOTTOM_CENTER" formatType="DIGIT" sideChar="-"/>
```

## 10.7.11 indexmark 요소

`<indexmark>`는 찾아보기(Index, 색인)와 관련된 정보를 갖고 있는 요소이다.

### 표 179 — indexmark 요소

| 하위 요소 이름 | 설명                                                                                                          |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| firstKey       | 찾아보기에 사용할 첫 번째 키워드<br/>요소의 값으로 키워드 문자열을 가짐.<br/>해당 요소의 추가적인 설명은 생략 |
| secondKey      | 찾아보기에 사용할 두 번째 키워드<br/>요소의 값으로 키워드 문자열을 가짐.<br/>해당 요소의 추가적인 설명은 생략 |

### 샘플 96 — indexmark 예

```xml
<hp:indexmark>
  <hp:firstKey>aa</hp:firstKey>
  <hp:secondKey>aa</hp:secondKey>
</hp:indexmark>
```

## 10.7.12 hiddenComment 요소

`<hiddenComment>`는 숨은 설명 내용 정보를 갖고 있는 요소이다.

### 표 180 — hiddenComment 요소

| 하위 요소 이름 | 설명                           |
| -------------- | ------------------------------ |
| subList        | 숨은 설명 내용<br/>10.1.1 참조 |

### 샘플 97 — hiddenComment 예

```xml
<hp:hiddenComment>
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP" linkListIDRef="0"
    linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="0" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0">
      <hp:run charPrIDRef="6">
        <hp:t>
          <hp:insertBegin Id="55" TcId="19"/>
          숨은 주석임.
          <hp:insertEnd id="55" TcId="19" paraend="0"/>
        </hp:t>
      </hp:run>
      <hp:linesegarray>
        <hp:lineseg textpos="0" vertpos="0" vertsize="1000" textheight="1000" baseline="850" spacing="600"
          horzpos="0" horzsize="56692" flags="393216"/>
      </hp:linesegarray>
    </hp:p>
  </hp:subList>
</hp:hiddenComment>
```

## 10.8 t 요소

### 10.8.1 t

`<t>` 요소는 문서의 실제 글자들을 담고 있는 요소이다. `<t>` 요소는 요소의 값으로 글자들을 가지게 된다. 단 Tab 글자, 줄바꿈 글자와 같이 특수 글자들은 실제 글자 대신에 하위 요소로서 가지고 있게 된다.

### 표 181 — t 요소

| 속성 이름   | 설명                         |
| ----------- | ---------------------------- |
| charPrIDRef | 글자 모양 설정 아이디 참조값 |

### 표 182 — t 하위 요소

| 하위 요소 이름 | 설명                                                   |
| -------------- | ------------------------------------------------------ |
| {요소 값}      | 글자                                                   |
| markpenBegin   | 형광펜 시작                                            |
| markpenEnd     | 형광펜 끝<br/>해당 요소의 추가적인 설명은 생략         |
| titleMark      | 제목 차례 표시                                         |
| tab            | 탭<br/>하위 속성들은 integer type이지만 단위는 HWPUNIT |
| lineBreak      | 강제 줄나눔<br/>해당 요소의 추가적인 설명은 생략       |
| hyphen         | 하이픈<br/>해당 요소의 추가적인 설명은 생략            |
| nbSpace        | 묶음 빈칸<br/>해당 요소의 추가적인 설명은 생략         |
| fwSpace        | 고정폭 빈칸<br/>해당 요소의 추가적인 설명은 생략       |
| insertBegin    | 변경 추적 삽입 시작지점                                |
| insertEnd      | 변경 추적 삽입 끝지점                                  |
| deleteBegin    | 변경 추적 삭제 시작지점                                |
| deleteEnd      | 변경 추적 삭제 끝지점                                  |

## 10.8.2 markpenBegin 요소

형광펜 색상 정보를 담고 있는 요소이다.

### 표 183 — markpenBegin 요소

| 속성 이름  | 설명        |
| ---------- | ----------- |
| beginColor | 형광펜 색상 |

### 샘플 98 — markpenBegin 예

```xml
<hp:markpenBegin color="#FF0000"/>
sampletext
<hp:markpenEnd/>
```

## 10.8.3 titleMark 요소

제목 차례 표시 여부를 갖고 있는 요소이다.

### 표 184 — titleMark 요소

| 속성 이름 | 설명                                                                     |
| --------- | ------------------------------------------------------------------------ |
| ignore    | 제목 차례 표시 여부<br/>true: 제목 차례 표시<br/>false: 차례 만들기 무시 |

## 10.8.4 tab 요소

### 표 185 — tab 요소

| 속성 이름 | 설명                                                                                                                                                                                                                                                                                         |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| width     | 탭의 간격                                                                                                                                                                                                                                                                                    |
| leader    | 탭의 채움모양: LineType2                                                                                                                                                                                                                                                                     |
| type      | 탭 종류<br/>LEFT: 왼쪽 정렬 탭, 그 위치에 있는 낱말의 왼쪽 시작 부분을 정렬<br/>RIGHT: 오른쪽 정렬 탭, 그 위치에 있는 낱말의 오른쪽 끝부분을 정렬<br/>CENTER: 가운데 정렬 탭, 그 위치에 있는 낱말의 가운데 부분을 정렬<br/>DECIMAL: 소수점 정렬 탭, 그 위치에 있는 숫자의 소수점 부분을 정렬 |

### 샘플 99 — tab 예

```xml
<hp:tab width="31188" leader="0" type="2"/>
```

## 10.8.5 변경 추적 요소 형식

`<insertBegin>`, `<insertEnd>`, `<deleteBegin>`, `<deleteEnd>` 요소는 [TrackChangeTag] 형식을 기본으로 하며 [TrackChangeTag]은 변경 추적 정보를 정의한 형식이다.

### 표 186 — TrackChangeTag 요소

| 속성 이름 | 설명                             |
| --------- | -------------------------------- |
| Id        | 변경 추적을 식별하기 위한 아이디 |
| TcId      | 변경 추적 아이디 참조값          |
| paraend   | 문단 끝 포함 여부                |

### 샘플 100 — TrackChangeTag 예

```xml
<hp:run charPrIDRef="7">
  <hp:t>
    프로그램입니다.
    <hp:insertBegin Id="1" TcId="1"/>
    <hp:insertEnd Id="1" TcId="1" paraend="1"/>
  </hp:t>
</hp:run>
```

## 10.9 기본 도형 객체

### 10.9.1 도형 객체

기본 도형 객체는 표, 그림, 수식, 컨테이너와 같은 문서 내에서 텍스트 이외의 기본적인 객체들을 뜻한다. 기본 도형 객체들은 [AbstractShapeObjectType]을 기본 형식(base-type)으로 가진다.

## 10.9.2 AbstractShapeObjectType

### 10.9.2.1 AbstractShapeObjectType

[AbstractShapeObjectType]은 기본 도형 객체들의 공통된 속성을 정의한 형식이다. 기본 도형 객체들은 [AbstractShapeObjectType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장해서 사용한다. [AbstractShapeObjectType]은 추상 형식이므로 [AbstractShapeObjectType]만으로는 XML 요소를 생성할 수 없다.

### 표 187 — AbstractShapeObjectType 요소

| 속성 이름     | 설명                                                                                                                        |
| ------------- | --------------------------------------------------------------------------------------------------------------------------- |
| id            | 객체를 식별하기 위한 아이디                                                                                                 |
| zOrder        | z-order                                                                                                                     |
| numberingType | 이 객체가 속하는 번호 범위                                                                                                  |
| textWrap      | 오브젝트 주위를 텍스트가 어떻게 흘러갈지 정하는 옵션<br/>하위 요소 pos의 속성 중 "treatAsChar"이 "false"인 때에만 사용.     |
| textFlow      | 오브젝트의 좌우 어느 쪽에 글을 배치할지 정하는 옵션<br/>textWrap 속성이 "SQUARE" 또는 "TIGHT" 또는 "THROUGH"일 때에만 사용. |
| lock          | 객체 선택 가능 여부                                                                                                         |
| dropcapStyle  | 첫글자 장식 스타일<br/>None: 없음<br/>DoubleLine: 2줄<br/>TripleLine: 3줄<br/>Margin: 여백                                  |

### 표 188 — AbstractShapeObjectType 하위 요소

| 하위 요소 이름 | 설명                                                                 |
| -------------- | -------------------------------------------------------------------- |
| sz             | 크기 정보                                                            |
| pos            | 위치 정보                                                            |
| outMargin      | 바깥 여백                                                            |
| caption        | 캡션                                                                 |
| shapeComment   | 요소의 값으로 주석 내용을 가짐.<br/>해당 요소의 추가적인 설명은 생략 |
| metaTag        | 메타태그 관련 정보                                                   |

### 샘플 101 — AbstractShapeObjectType 예

```xml
<hp:rect id="1790879982" zOrder="0" numberingType="PICTURE" textWrap="IN_FRONT_OF_TEXT"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None">
```

### 10.9.2.2 객체 크기 정보

객체들의 크기 정보를 가지고 있는 요소이다.

### 표 189 — sz 요소

| 속성 이름   | 설명                 |
| ----------- | -------------------- |
| width       | 오브젝트 폭          |
| widthRelTo  | 오브젝트 폭의 기준   |
| height      | 오브젝트 높이        |
| heightRelTo | 오브젝트 높이의 기준 |
| protect     | 크기 보호 여부       |

### 10.9.2.3 객체 위치 정보

객체들의 위치 정보 및 객체들이 문서에서 차지하는 영역에 대한 정보를 가지고 있는 요소이다.

### 표 190 — pos 요소

| 속성 이름       | 설명                                                                                                                                              |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| treatAsChar     | 글자처럼 취급 여부                                                                                                                                |
| affectLSpacing  | 줄 간격에 영향을 줄지 여부<br/>treatAsChar 속성이 "true"일 때에만 사용                                                                            |
| flowWithText    | 오브젝트의 세로 위치를 본문 영역으로 제한할지 여부<br/>하위 요소 RelativeTo의 속성 중 "vertical"이 "PARA"일 때에만 사용                           |
| allowOverlap    | 다른 오브젝트와 겹치는 것을 허용할지 여부<br/>treatAsChar 속성이 "false"일 때에만 사용<br/>flowWithText 속성이 "true"이면 무조건 "false"로 간주함 |
| holdAnchorAndSO | 객체와 조판부호를 항상 같은 쪽에 놓을지 여부                                                                                                      |
| vertRelTo       | 세로 위치의 기준<br/>treatAsChar 속성이 "false"일 때에만 사용                                                                                     |
| horzRelTo       | 가로 위치의 기준<br/>treatAsChar 속성이 "false"일 때에만 사용                                                                                     |

vertOffset="10575" horzOffset="9927"/>

````

### 10.9.2.4 객체 바깥 여백

`<outMargin>` 요소는 [MarginAttributeGroup]을 속성으로 포함한다. [MarginAttributeGroup]은 **10.6.6.2**를 참조한다.


### 표 191 — outMargin 요소

| 속성 이름              | 설명          |
| ---------------------- | ------------- |
| [MarginAttributeGroup] | 10.6.6.2 참조 |

### 샘플 103 — outMargin 예

```xml
<hp:outMargin left="10" right="10" top="0" bottom="0"/>
````

### 10.9.2.5 객체 캡션

`<caption>` 요소는 하위 요소로 `<subList>` 요소를 가진다. `<subList>` 요소는 **11.1.2**를 참조한다.

### 표 192 — caption 요소

| 속성 이름 | 설명                           |
| --------- | ------------------------------ |
| side      | 캡션 방향                      |
| fullSize  | 캡션 폭에 마진을 포함할지 여부 |
| width     | 캡션 폭                        |
| gap       | 캡션과 몸 사이의 간격          |
| lastWidth | 텍스트 최대 길이(=객체의 폭)   |

### 표 193 — caption 하위 요소

| 하위 요소 이름 | 설명                       |
| -------------- | -------------------------- |
| subList        | 캡션 내용.<br/>11.1.2 참조 |

### 샘플 104 — caption 예

```xml
<hp:caption side="BOTTOM" fullSz="0" width="8504" gap="850" lastWidth="16800">
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP"
    linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="19" styleIDRef="21" pageBreak="0" columnBreak="0"
      merged="0">
      <hp:run charPrIDRef="0">
        <hp:t>그림 </hp:t>
        <hp:ctrl>
          <hp:autoNum num="1" numType="PICTURE">
            <hp:autoNumFormat type="DIGIT" userChar="" prefixChar="" suffixChar=""
              supscript="0"/>
          </hp:autoNum>
        </hp:ctrl>
        <hp:t> </hp:t>
      </hp:run>
    </hp:p>
  </hp:subList>
</hp:caption>
```

## 10.9.3 tbl 요소

### 10.9.3.1 tbl

`<tbl>` 요소는 표에 관한 정보를 가지고 있는 요소로 [AbstractShapeObjectType]을 상속받는다. [AbstractShapeObjectType]의 자세한 내용은 **10.9.2**를 참조한다.

### 표 194 — tbl 요소

| 속성 이름       | 설명                                                                                                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| pageBreak       | 테이블이 페이지 경계에서 나뉘는 방식<br/>TABLE: 테이블은 나뉘지만 셀은 나뉘지 않음.<br/>CELL: 셀 내의 텍스트도 나뉨.<br/>NONE: 나뉘지 않음. |
| repeatHeader    | 테이블이 나뉘었을 경우, 제목 행을 나뉜<br/>페이지에서도 반복할지 여부                                                                       |
| rowCnt          | 테이블 행 개수                                                                                                                              |
| noAdjust        | 셀 너비/높이 값의 최소 단위(1 pt) 보정 여부                                                                                                 |
| colCnt          | 테이블 열 개수                                                                                                                              |
| cellSpacing     | 셀 간격. 단위는 HWPUNIT.                                                                                                                    |
| borderFillIDRef | 테두리/배경 아이디 참조값                                                                                                                   |

### 표 195 — tbl 하위 요소

| 하위 요소 이름 | 설명      |
| -------------- | --------- |
| inMargin       | 안쪽 여백 |
| cellzoneList   | 셀존 목록 |
| tr             | 행        |
| label          | 라벨      |

### 샘플 105 — tbl 예

```xml
<hp:tbl id="1811647054" zOrder="0" numberingType="TABLE" textWrap="TOP_AND_BOTTOM"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" pageBreak="CELL" repeatHeader="1"
  rowCnt="5" colCnt="5" cellSpacing="0" borderFillIDRef="3" noAdjust="0">
  <hp:sz width="41950" widthRelTo="ABSOLUTE" height="6410" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="0"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="283" right="283" top="283" bottom="283"/>
  <hp:inMargin left="510" right="510" top="141" bottom="141"/>
  <hp:tr>
    <hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0"
      borderFillIDRef="3">
      <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER"
        linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
        <hp:p id="0" paraPrIDRef="0" styleIDRef="0" pageBreak="0" columnBreak="0"
          merged="0">
          <hp:run charPrIDRef="0"/>
        </hp:p>
      </hp:subList>
      <hp:cellAddr colAddr="0" rowAddr="0"/>
      <hp:cellSpan colSpan="1" rowSpan="2"/>
      <hp:cellSz width="8390" height="564"/>
      <hp:cellMargin left="510" right="510" top="141" bottom="141"/>
    </hp:tc>
    ......
  </hp:tr>
</hp:tbl>
```

### 10.9.3.2 inMargin 요소

`<inMargin>` 요소는 안쪽 여백 정보로 [MarginAttributeGroup]을 속성으로 포함한다.

[MarginAttributeGroup]은 **10.6.6.2**를 참조한다.

### 표 196 — inMargin 요소

| 속성 이름              | 설명          |
| ---------------------- | ------------- |
| [MarginAttributeGroup] | 10.6.6.2 참조 |

### 샘플 106 — inMargin 예

```xml
<hp:inMargin left="510" right="510" top="141" bottom="141"/>
```

### 10.9.3.3 cellzoneList 요소

#### 10.9.3.3.1 cellzoneList

표는 표 전체 또는 표 부분적으로 배경색 및 테두리와 같은 속성을 줄 때, 영역을 지정하기 위해서 `<cellzone>` 요소를 사용한다.

### 표 197 — cellzoneList 하위 요소

| 하위 요소 이름 | 설명 |
| -------------- | ---- |
| cellzone       | 셀존 |

#### 10.9.3.3.2 cellzone 요소

Cell zone은 표에서 스타일 및 모양이 적용되는 단위이다. 아래 표와 같이 5x5 테이블 중 가운데 2x3 영역만 다른 테두리를 적용된 경우, cell zone은 아래 예시와 같은 값을 가지게 된다.

### 샘플 107 — cellzone 예

```xml
<cellzone startRowAddr="1" startColAddr="1"
  endRowAddr="2" endColAddr="3" borderFillIDRef="borderXXX" />
```

### 표 198 — cellzone 요소

| 속성 이름       | 설명                                          |
| --------------- | --------------------------------------------- |
| startRowAddr    | 셀존 row의 시작 주소<br/>주소는 0부터 시작    |
| startColAddr    | 셀존 column의 시작 주소<br/>주소는 0부터 시작 |
| endRowAddr      | 셀존 row의 끝 주소<br/>주소는 0부터 시작      |
| endColAddr      | 셀존 column의 끝 주소<br/>주소는 0부터 시작   |
| borderFillIDRef | 테두리/배경 아이디 참조값                     |

### 10.9.3.4 tr 요소

#### 10.9.3.4.1 tr

표에서 하나의 행을 표현하기 위한 요소이다. 하나의 행 안에는 여러 개의 열을 가지게 된다.

### 표 199 — tr 하위 요소

| 하위 요소 이름 | 설명      |
| -------------- | --------- |
| tc             | 테이블 열 |

#### 10.9.3.4.2 tc 요소

`<tc>` 요소는 하위 요소로 표 안의 글 내용을 담고 있는 `<subList>` 요소를 가진다. `<subList>` 요소는 **11.1.2**를 참조한다.

### 표 200 — tc 요소

| 속성 이름       | 설명                                                         |
| --------------- | ------------------------------------------------------------ |
| name            | 셀 필드 이름                                                 |
| header          | 제목 셀인지 여부                                             |
| hasMargin       | 테이블의 기본 셀 여백이 아닌 독자적인 여백을 사용하는지 여부 |
| protect         | 사용자 편집을 막을지 여부                                    |
| editable        | 읽기 전용 상태에서도 수정 가능한지 여부                      |
| dirty           | 마지막 업데이트된 이후 사용자가 내용을 변경했는지 여부       |
| borderFillIDRef | 테두리/배경 아이디 참조값                                    |

### 표 201 — tc 하위 요소

| 하위 요소 이름 | 설명                 |
| -------------- | -------------------- |
| subList        | 셀 내용(11.1.2 참조) |
| cellAddr       | 셀 주소              |
| cellSpan       | 셀 병합 정보         |
| cellSz         | 셀 크기              |
| cellMargin     | 셀 여백              |

### 샘플 108 — tc 예

```xml
<hp:tc name="" header="0" hasMargin="0" protect="0" editable="0" dirty="0" borderFillIDRef="4">
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER"
    linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="0" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0">
      <hp:run charPrIDRef="0"/>
    </hp:p>
  </hp:subList>
  <hp:cellAddr colAddr="4" rowAddr="0"/>
  <hp:cellSpan colSpan="1" rowSpan="1"/>
  <hp:cellSz width="8390" height="282"/>
  <hp:cellMargin left="510" right="510" top="141" bottom="141"/>
</hp:tc>
```

#### • 셀 주소

표에서 하나의 열이 차지하는 영역을 지정하기 위한 요소이다.

### 표 202 — cellAddr 요소

| 속성 이름 | 설명                                                                                 |
| --------- | ------------------------------------------------------------------------------------ |
| colAddr   | 셀의 열 주소<br/>주소는 0부터 시작<br/>표에서 제일 위쪽 셀이 0부터 시작하여 1씩 증가 |
| rowAddr   | 셀의 행 주소<br/>주소는 0부터 시작<br/>표에서 제일 위쪽 셀이 0부터 시작하여 1씩 증가 |

### 샘플 109 — cellAddr 예

```xml
<hp:cellAddr colAddr="3" rowAddr="0"/>
```

| --------- | ------------ |
| colSpan | 열 병합 개수 |
| rowSpan | 행 병합 개수 |

### 샘플 110 — cellSpan 예

```xml
<hp:cellSpan colSpan="1" rowSpan="1"/>
```

#### • 셀 크기

개별 셀의 크기 정보를 가지고 있는 요소이다.

### 표 204 — cellSz 요소

| 하위 요소 이름 | 설명                      |
| -------------- | ------------------------- |
| width          | 셀의 폭. 단위는 HWPUNIT   |
| height         | 셀의 높이. 단위는 HWPUNIT |

### 샘플 111 — cellSz 예

```xml
<hp:cellSz width="8390" height="282"/>
```

#### • 셀 여백

`<cellMargin>` 요소는 [MarginAttributeGroup]을 속성으로 포함한다. [MarginAttributeGroup]은 **10.6.6.2**를 참조한다.

### 표 205 — cellMargin 요소

| 속성 이름              | 설명          |
| ---------------------- | ------------- |
| [MarginAttributeGroup] | 10.6.6.2 참조 |

### 샘플 112 — cellMargin 예

```xml
<hp:cellMargin left="510" right="510" top="141" bottom="141"/>
```

### 10.9.3.5 Label 요소

### 표 206 — label 요소

| 속성 이름    | 설명             |
| ------------ | ---------------- |
| topmargin    | 용지 위쪽 여백   |
| leftmargin   | 용지 왼쪽 여백   |
| boxwidth     | 이름표 폭        |
| boxlength    | 이름표 길이      |
| boxmarginhor | 이름표 좌우 여백 |
| boxmarginver | 이름표 상하 여백 |
| labelcols    | 이름표 행의 개수 |
| labelrows    | 이름표 열의 개수 |
| landscape    | 용지 방향        |
| pagewidth    | 문서의 폭        |
| pageheight   | 문서의 길이      |

### 샘플 113 — label 예

```xml
<hp:label topmargin="1332" leftmargin="1532" boxwidth="56692" boxlength="81936"
  boxmarginhor="0" boxmarginver="0" labelcols="1" labelrows="1" landscape="WIDELY"
  pagewidth="59528" pageheight="84188"/>
```

## 10.9.4 equation 요소

`<equation>` 요소는 [AbstractShapeObjectType]을 상속받는다. [AbstractShapeObjectType]의 자세한 내용은 **10.9.2**를 참조한다.

### 표 207 — equation 요소

| 속성 이름 | 설명                                                   |
| --------- | ------------------------------------------------------ |
| version   | 수식 버전<br/>현재 버전은 "Equation Version>apeObjectT |
| baseLine  | 수식이 그려질 기본 선                                  |
| textColor | 수식 글자 색                                           |
| baseUnit  | 수식의 글자 크기. 단위는 HWPUNIT.                      |
| lineMode  | 수식이 차지하는 범위                                   |
| font      | 수식 폰트<br/>Default font : "HYhwpEQ"                 |

### 표 208 — equation 하위 요소

| 하위 요소 이름 | 설명                                                        |
| -------------- | ----------------------------------------------------------- |
| script         | 수식 내용<br/>자세한 내용은 부속서 I의 수식 스크립트를 참조 |

### 샘플 114 — equation 예

```xml
<hp:equation id="1606912079" zOrder="2" numberingType="EQUATION"
  textWrap="TOP_AND_BOTTOM" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None"
  version="Equation Version 60" baseLine="66" textColor="#000000" baseUnit="1000"
  lineMode="CHAR" font="HancomEQN">
  <hp:sz width="9125" widthRelTo="ABSOLUTE" height="2250" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="0"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="56" right="56" top="0" bottom="0"/>
  <hp:shapeComment>수식입니다.</hp:shapeComment>
  <hp:script>pi = C over d = 3.14159CDOTS</hp:script>
</hp:equation>
```

## 10.9.5 AbstractShapeComponentType

### 10.9.5.1 AbstractShapeComponentType

[AbstractShapeComponentType]은 [AbstractShapeObjectType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장한다. [AbstractShapeObjectType]의 자세한 내용은 **10.9.2**를 참조한다.

[AbstractShapeComponentType]은 추상 형식이므로 [AbstractShapeComponentType]만으로는 XML 요소를 생성할 수 없다.

### 표 209 — AbstractShapeComponentType 요소

| 속성 이름  | 설명            |
| ---------- | --------------- |
| href       | 하이퍼링크 속성 |
| groupLevel | 그룹핑 횟수     |
| instid     | 객체 아이디     |

### 표 210 — AbstractShapeComponentType 하위 요소

| 하위 요소 이름 | 설명                                  |
| -------------- | ------------------------------------- |
| offset         | 객체가 속한 그룹 내에서의 오프셋 정보 |
| orgSz          | 객체 생성 시 최초 크기                |
| curSz          | 객체의 현재 크기                      |
| flip           | 객체가 뒤집어진 상태인지 여부         |
| rotationInfo   | 객체 회전 정보                        |
| renderingInfo  | 객체 렌더링 정보                      |

### 샘플 115 — AbstractShapeComponentType 예

```xml
<hp:rect id="1833429566" zOrder="8" numberingType="PICTURE" textWrap="IN_FRONT_OF_TEXT"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="759687743" ratio="0">
  ......
  <hp:offset x="0" y="0"/>
  <hp:orgSz width="16800" height="12825"/>
  <hp:curSz width="0" height="0"/>
  <hp:flip horizontal="0" vertical="0"/>
  <hp:rotationInfo angle="0" centerX="8400" centerY="6412" rotateimage="1"/>
  <hp:renderingInfo>
    <hp:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:scaMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
  </hp:renderingInfo>
  ......
```

### 10.9.5.2 객체가 속한 그룹 내에서의 오프셋 정보

그룹 객체 내에서 개별 객체들의 그룹 내 상대 위치 정보를 가지고 있는 요소이다.

### 표 211 — offset 요소

| 속성 이름 | 설명                               |
| --------- | ---------------------------------- |
| x         | 객체가 속한 그룹 내에서의 x offset |
| y         | 객체가 속한 그룹 내에서의 y offset |

### 샘플 116 — offset 예

```xml
<hp:offset x="0" y="0"/>
```

### 10.9.5.3 객체 생성 시 최초 크기

객체 생성 시 최초 크기 정보를 가지고 있는 요소이다.

### 표 212 — orgSz 요소

| 속성 이름 | 설명                                   |
| --------- | -------------------------------------- |
| width     | 객체 생성 시 최초 폭. 단위는 HWPUNIT   |
| height    | 객체 생성 시 최초 높이. 단위는 HWPUNIT |

### 샘플 117 — orgSz 예

```xml
<hp:orgSz width="16800" height="12825"/>
```

### 10.9.5.4 객체의 현재 크기

객체의 현재 크기 정보를 가지고 있는 요소이다.

### 표 213 — curSz 요소

| 속성 이름 | 설명                             |
| --------- | -------------------------------- |
| width     | 객체의 현재 폭. 단위는 HWPUNIT   |
| height    | 객체의 현재 높이. 단위는 HWPUNIT |

### 샘플 118 — curSz 예

```xml
<hp:curSz width="12500" height="5000"/>
```

### 10.9.5.5 객체가 뒤집어진 상태인지 여부

객체의 반전 여부 정보를 가지고 있는 요소이다.

### 표 214 — flip 요소

| 속성 이름  | 설명                          |
| ---------- | ----------------------------- |
| horizontal | 좌우로 뒤집어진 상태인지 여부 |
| vertical   | 상하로 뒤집어진 상태인지 여부 |

### 샘플 119 — flip 예

```xml
<hp:flip horizontal="1" vertical="0"/>
```

### 10.9.5.6 객체 회전 정보

객체의 회전 정보를 가지고 있는 요소이다.

### 표 215 — rotationInfo 요소

| 속성 이름   | 설명               |
| ----------- | ------------------ |
| angle       | 회전각             |
| centerX     | 회전 중심의 x 좌표 |
| centerY     | 회전 중심의 y 좌표 |
| rotateimage | 이미지 회전 여부   |

### 샘플 120 — rotationInfo 예

```xml
<hp:rotationInfo angle="0" centerX="6250" centerY="2500" rotateimage="1"/>
```

### 10.9.5.7 객체 렌더링 정보

#### 10.9.5.7.1 객체 렌더링

객체 렌더링 시 필요한, 변환 행렬, 확대/축소 행렬, 회전 행렬을 가지고 있는 요소이다.

### 표 216 — renderingInfo 요소

| 하위 요소 이름 | 설명                                   |
| -------------- | -------------------------------------- |
| transMatrix    | Translation Matrix<br/>10.9.5.7.2 참조 |
| scaMatrix      | Scaling Matrix<br/>10.9.5.7.2 참조     |
| rotMatrix      | Rotation Matrix<br/>10.9.5.7.2 참조    |

#### 10.9.5.7.2 행렬 요소 형식

[MatrixType]은 행렬을 표현하기 위한 요소 형식이다. 9x9 행렬에서 2행의 요소가 저장만 표현을 하고 3행의 요소는 (0, 0, 1)로 일정하기 때문에 표현하지 않는다.

### 표 217 — MatrixType 요소

| 속성 이름 | 설명                            |
| --------- | ------------------------------- |
| e1        | 9x9 행렬의 첫 번째 요소 (0,0)   |
| e2        | 9x9 행렬의 두 번째 요소 (0,1)   |
| e3        | 9x9 행렬의 세 번째 요소 (0,2)   |
| e4        | 9x9 행렬의 네 번째 요소 (1,0)   |
| e5        | 9x9 행렬의 다섯 번째 요소 (1,1) |
| e6        | 9x9 행렬의 여섯 번째 요소 (1,2) |

### 샘플 121 — MatrixType 예

```xml
<hp:renderingInfo>
  <hp:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
  <hp:scaMatrix e1="0.881959" e2="0" e3="0" e4="0" e5="0.352783" e6="0"/>
  <hp:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
</hp:renderingInfo>
```

## 10.9.6 pic 요소

### 10.9.6.1 pic

`<pic>` 요소는 [AbstractShapeComponentType]을 상속받는다. [AbstractShapeComponentType]의 자세한 내용은 **10.9.5**를 참조한다.

### 표 218 — pic 요소

| 속성 이름 | 설명           |
| --------- | -------------- |
| reverse   | 그림 색상 반전 |

### 표 219 — pic 하위 요소

| 하위 요소 이름 | 설명                             |
| -------------- | -------------------------------- |
| lineShape      | 테두리선 모양                    |
| imgRect        | 이미지 좌표 정보                 |
| imgClip        | 이미지 자르기 정보               |
| effects        | 이미지 효과 정보                 |
| inMargin       | 안쪽 여백 정보<br/>10.6.6.2 참조 |
| imgDim         | 이미지 원본 정보                 |
| img            | 그림 정보                        |

### 샘플 122 — pic 예

```xml
<hp:pic id="1790881809" zOrder="2" numberingType="PICTURE" textWrap="SQUARE" textFlow="BOTH_SIDES"
  lock="0" dropcapstyle="None" href="" groupLevel="0" instid="717139986" reverse="0">
  <hp:offset x="0" y="0"/>
  <hp:orgSz width="13800" height="15438"/>
  <hp:curSz width="0" height="0"/>
  <hp:flip horizontal="0" vertical="0"/>
  <hp:rotationInfo angle="0" centerX="6900" centerY="7719" rotateimage="1"/>
  <hp:renderingInfo>
    <hp:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:scaMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
  </hp:renderingInfo>
  <hp:img binaryItemIDRef="image1" bright="0" contrast="0" effect="REAL_PIC" alpha="0"/>
  <hp:lineShape color="#FF0000" width="33" style="DOT" endCap="FLAT" headStyle="NORMAL" tailStyle="NORMAL"
    headfill="0" tailfill="0" headSz="SMALL_SMALL" tailSz="SMALL_SMALL" outlineStyle="OUTER" alpha="0"/>
  <hp:imgRect>
    <hp:pt0 x="0" y="0"/>
    <hp:pt1 x="13800" y="0"/>
    <hp:pt2 x="13800" y="15438"/>
    <hp:pt3 x="0" y="15438"/>
  </hp:imgRect>
  <hp:imgClip left="0" right="45060" top="0" bottom="50400"/>
  <hp:inMargin left="0" right="0" top="0" bottom="0"/>
  <hp:imgDim dimwidth="45060" dimheight="50400"/>
  <hp:effects/>
  <hp:sz width="13800" widthRelTo="ABSOLUTE" height="15438" heightRelTo="ABSOLUTE" protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="1" holdAnchorAndSO="0"
    vertRelTo="PAPER" horzRelTo="PAPER" vertAlign="TOP" horzAlign="LEFT" vertOffset="33960"
    horzOffset="11910"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:shapeComment>그림입니다. 원본 그림의 이름: 01_네오버전.png 원본 그림의 크기: 가로 601pixel, 세로
    672pixel</hp:shapeComment>
</hp:pic>
```

### 10.9.6.2 테두리선 모양

객체의 테두리선 정보를 표현하기 위한 요소이다.

### 표 220 — lineShape 요소

| 속성 이름    | 설명                                  |
| ------------ | ------------------------------------- |
| color        | 선 색상                               |
| width        | 선 굵기. 단위는 HWPUNIT               |
| style        | 선 종류                               |
| endCap       | 선 끝 모양                            |
| headStyle    | 화살표 시작 모양                      |
| tailStyle    | 화살표 끝 모양                        |
| headfill     | 화살표 시작점 선 색상으로 채우기 여부 |
| tailfill     | 화살표 끝점 선 색상으로 채우기 여부   |
| headSz       | 화살표 시작 크기                      |
| tailSz       | 화살표 끝 크기                        |
| outlineStyle | 테두리선의 형태                       |
| alpha        | 투명도                                |

### 샘플 123 — lineShape 예

```xml
<hp:lineShape color="#141313" width="6" style="SOLID" endCap="FLAT" headStyle="NORMAL"
  tailStyle="NORMAL" headfill="1" tailfill="1" headSz="SMALL_SMALL" tailSz="SMALL_SMALL"
  outlineStyle="INNER" alpha="127"/>
```

### 10.9.6.3 이미지 좌표 정보

#### 10.9.6.3.1 이미지 좌표

그림의 좌표 정보를 가지고 있는 요소이다.

### 표 221 — imgRect 요소

| 하위 요소 이름 | 설명                             |
| -------------- | -------------------------------- |
| pt0            | 첫 번째 좌표<br/>10.9.6.3.2 참조 |
| pt1            | 두 번째 좌표<br/>10.9.6.3.2 참조 |
| pt2            | 세 번째 좌표<br/>10.9.6.3.2 참조 |
| pt3            | 네 번째 좌표<br/>10.9.6.3.2 참조 |

#### 10.9.6.3.2 점 요소 형식

좌표 정보를 표현할 때 사용하는 요소로, 2축 좌표계를 사용한다.

### 표 222 — PointType 요소

| 속성 이름 | 설명   |
| --------- | ------ |
| x         | x 좌표 |
| y         | y 좌표 |

### 샘플 124 — PointType 예

```xml
<hp:imgRect>
  <hp:pt0 x="0" y="0"/>
  <hp:pt1 x="14112" y="0"/>
  <hp:pt2 x="14112" y="7938"/>
  <hp:pt3 x="0" y="7938"/>
</hp:imgRect>
```

### 10.9.6.4 이미지 자르기 정보

원본 그림을 기준으로 자를 영역 정보를 가지고 있는 요소이다. 자르기 정보가 설정되면, 그림은 논리적으로 원본 그림에서 해당 영역만큼 잘리게 되고, 화면에서는 남은 영역만 표시된다.

### 표 223 — imgClip 요소

| 속성 이름 | 설명                          |
| --------- | ----------------------------- |
| left      | 왼쪽에서 이미지를 자른 크기   |
| right     | 오른쪽에서 이미지를 자른 크기 |
| top       | 위쪽에서 이미지를 자른 크기   |
| bottom    | 아래쪽에서 이미지를 자른 크기 |

### 샘플 125 — imgClip 예

```xml
<hp:imgClip left="0" right="96000" top="0" bottom="54000"/>
```

### 10.9.6.5 이미지 효과 정보

#### 10.9.6.5.1 이미지 효과

그림에 적용될 효과 정보를 가지고 있는 요소이다.

### 표 224 — effects 요소

| 하위 요소 이름 | 설명                   |
| -------------- | ---------------------- |
| shadow         | 그림자 효과            |
| glow           | 네온 효과              |
| softEdge       | 부드러운 가장자리 효과 |
| reflection     | 반사 효과              |

#### 10.9.6.5.2 그림자 효과

그림 효과 중 그림자 효과에 대한 설정 정보를 가지고 있는 요소이다.

### 표 225 — shadow 요소

| 속성 이름     | 설명                         |
| ------------- | ---------------------------- |
| style         | 그림자 스타일                |
| alpha         | 시작 투명도                  |
| radius        | 흐릿함 정도                  |
| direction     | 방향 각도                    |
| distance      | 대상과 그림자 사이의 거리    |
| alignStyle    | 그림자 정렬                  |
| rotationStyle | 도형과 함께 그림자 회전 여부 |

### 표 226 — shadow 하위 요소

| 하위 요소 이름 | 설명        |
| -------------- | ----------- |
| skew           | 기울기      |
| scale          | 확대 비율   |
| effectsColor   | 그림자 색상 |

### 샘플 126 — shadow 예

```xml
<hp:shadow style="OUTSIDE" alpha="0.5" radius="600" direction="30" distance="600"
  alignStyle="CENTER" rotationStyle="0">
  <hp:skew x="15" y="0"/>
  <hp:scale x="1" y="1"/>
  <hp:effectsColor type="RGB" schemeIdx="-1" systemIdx="-1" presetIdx="-1">
    <hp:rgb r="0" g="0" b="0"/>
  </hp:effectsColor>
</hp:shadow>
```

#### • 기울기 각도

기울기 정보를 가지고 있는 요소이다.

### 표 227 — skew 요소

| 속성 이름 | 설명            |
| --------- | --------------- |
| x         | x축 기울기 각도 |
| y         | y축 기울기 각도 |

### 샘플 127 — skew 예

```xml
<hp:skew x="30" y="0"/>
```

#### • 확대 비율

확대 정보를 가지고 있는 요소이다.

### 표 228 — scale 요소

| 속성 이름 | 설명          |
| --------- | ------------- |
| x         | x축 확대 비율 |
| y         | y축 확대 비율 |

### 샘플 128 — scale 예

```xml
<hp:scale x="1" y="1.2"/>
```

| ----------- | -------------- |
| type | 색상 표현 방법 |
| schemaIndex | Scheme Index |
| systemIndex | System Index |
| presetIndex | Preset Index |

### 표 230 — effectsColor 하위 요소

| 하위 요소 이름 | 설명                                                                                                                          |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| rgb            | RGB 색상 표현<br/>속성으로 r, g, b 세 가지 속성을 가짐. 모두 0이상의 정수 값을 가짐.<br/>해당 요소의 추가적인 설명은 생략     |
| cmyk           | CMYK 색상 표현<br/>속성으로 c, m, y, k 네 가지 속성을 가짐. 모두 0이상의 정수 값을 가짐.<br/>해당 요소의 추가적인 설명은 생략 |
| scheme         | Scheme 색상 표현<br/>속성으로 r, g, b 세 가지 속성을 가짐. 모두 0이상의 정수 값을 가짐.<br/>해당 요소의 추가적인 설명은 생략  |
| system         | System 색상 표현<br/>속성으로 h, s, l 세 가지 속성을 가짐. 모두 0이상의 정수 값을 가짐.<br/>해당 요소의 추가적인 설명은 생략  |
| effect         | 색상 효과                                                                                                                     |

### 샘플 129 — effectsColor 예

```xml
<hp:effectsColor type="RGB" schemeIdx="-1" systemIdx="-1" presetIdx="-1">
  <hp:rgb r="255" g="215" b="0"/>
</hp:effectsColor>
```

#### • effect 요소

색상 효과 정보를 가지고 있는 요소이다.

### 표 231 — effect 요소

| 속성 이름 | 설명                    |
| --------- | ----------------------- |
| type      | 색상 효과 종류          |
| value     | 효과 적용에 필요한 수치 |

### 표 232 — 색상 효과 종류 1

| 색상 효과 구분 | 이름                                                                                    | 값의 범위 | 기본 값 |
| -------------- | --------------------------------------------------------------------------------------- | --------- | ------- |
|                |                                                                                         | 설명      |         |
| ALPHA          | 투명도                                                                                  | 0.0 ~ 1.0 | 1.0     |
|                | "0 ~ 1.0k xo"를 계산한 값으로 투명도를 대체<br/>1.0이면 불투명                          |           |         |
| ALPHA_MOD      | 투명도 조절 값                                                                          | 0.0 ~ 1.0 | 1.0     |
|                | "색상 정보 투명도 \* ALPHA_MOD"를 계산한 값으로 투명도를 대체함.<br/>1.0이면 변화 없음. |           |         |
| ALPHA_OFF      | 투명도 오프셋                                                                           | 정수형    | 0       |
|                | "색상 정보 투명도 + ALPHA_OFF"를 계산한 값으로 투명도를 대체                            |           |         |
| RED            | RGB 값 중 red 값                                                                        | 0.0 ~ 1.0 | 1.0     |
|                | "0 ~ 1.0FF"를 계산한 값으로 R 채널 값을 대체                                            |           |         |
| RED_MOD        | red 조절 값                                                                             | 0.0 ~ 1.0 | 1.0     |
|                | "0채널 \* RED_MODF"를 계산한 값으로 R 채널 값을 대체                                    |           |         |
| RED_OFF        | red 오프셋                                                                              | 정수형    | 0       |
|                | "수형채널 + RED_OFFF"를 계산한 값으로 R 채널 값을 대체                                  |           |         |
| GREEN          | RGB 값 중 green 값                                                                      | 0.0 ~ 1.0 | 1.0     |
|                | "0 ~ 1.0FF"를 계산한 값으로 G 채널 값을 대체                                            |           |         |
| GREEN_MOD      | green 조절 값                                                                           | 0.0 ~ 1.0 | 1.0     |
|                | "0채널 \* GREEN_MOD"를 계산한 값으로 G 채널 값을 대체                                   |           |         |
| GREEN_OFF      | green 오프셋                                                                            | 정수형    | 0       |
|                | "수형채널 + GREEN_OFF"를 계산한 값으로 G 채널 값을 대체                                 |           |         |
| BLUE           | RGB 값 중 blue 값                                                                       | 0.0 ~ 1.0 | 1.0     |
|                | "0 ~ 1.0FF"를 계산한 값으로 B 채널 값을 대체                                            |           |         |
| BLUE_MOD       | blue 조절 값                                                                            | 0.0 ~ 1.0 | 1.0     |
|                | "0채널 \* BLUE_MODF"를 계산한 값으로 B 채널 값을 대체                                   |           |         |

### 표 233 — 색상 효과 종류 2

| 색상 효과 구분 | 이름                                                                                                                       | 값의 범위      | 기본 값 |
| -------------- | -------------------------------------------------------------------------------------------------------------------------- | -------------- | ------- |
|                |                                                                                                                            | 설명           |         |
| BLUE_OFF       | blue 오프셋                                                                                                                | 정수형         | 0       |
|                | "수형채널 + BLUE_OFFF"를 계산한 값으로 B 채널 값을 대체                                                                    |                |         |
| HUE            | 색조                                                                                                                       | 0 ~ 359        |         |
|                | HSI 컬러 모델에서 색조값을 HUE로 설정                                                                                      |                |         |
| HUE_MOD        | 색조 조정값                                                                                                                | 0.0 ~ 1.0      | 1.0     |
|                | HSI 컬러 모델에서 색조값을 HUE_MOD만큼 조정                                                                                |                |         |
| HUE_OFF        | 색조 오프셋                                                                                                                | -16000 ~ 16000 | 0       |
|                | HSI 컬러 모델에서 색조값을 HUE_OFF만큼 조정                                                                                |                |         |
| SAT            | 채도                                                                                                                       | 0.0 ~ 1.0      |         |
|                | HSI 컬러 모델에서 채도값을 SAT로 설정                                                                                      |                |         |
| SAT_MOD        | 채도 조정값                                                                                                                | 0.0 ~ 1.0      | 1.0     |
|                | HSI 컬러 모델에서 채도값을 SAT_MOD만큼 조정                                                                                |                |         |
| SAT_OFF        | 채도 조정값                                                                                                                | 정수형         | 0       |
|                | HSI 컬러 모델에서 채도값을 SAT_OFF만큼 조정                                                                                |                |         |
| LUM            | 명도                                                                                                                       | 0.0 ~ 1.0      |         |
|                | HSI 컬러 모델에서 명도값을 LUM로 설정                                                                                      |                |         |
| LUM_MOD        | 명도 조정값                                                                                                                | 0.0 ~ 1.0      | 1.0     |
|                | HSI 컬러 모델에서 명도값을 LUM_MOD만큼 조정                                                                                |                |         |
| LUM_OFF        | 명도 조정값                                                                                                                | 0.0 ~ 1.0      | 0       |
|                | HSI 컬러 모델에서 명도값을 LUM_OFF만큼 조정                                                                                |                |         |
| SHADE          | 어둡게 조정값                                                                                                              |                | 1       |
|                | Color의 색상에 SHADE 만큼 어둡게 함.<br/>(RGB을 빼서 어둡게 함, 90 % 검정과 SHADE \* 현재 색상을 믹싱)<br/>1이면 변화없음. |                |         |
| TINT           | 밝게 조정값                                                                                                                |                | 1       |
|                | Color의 색상에 TINT만큼 밝게 함.<br/>(RGB 값을 더해서 밝게 함, 90 % 흰색과 TINT \* 현재 색상을 믹싱)<br/>1이면 변화 없음.  |                |         |
| GRAY           | Gray scale 사용                                                                                                            | 0 또는 1       |         |
|                | 색상을 Gray scale로 바꿈.                                                                                                  |                |         |
| COMP           | 보색 사용                                                                                                                  | 0 또는 1       |         |
|                | 색상을 보색으로 바꿈.                                                                                                      |                |         |
| GAMMA          | 감마 적용                                                                                                                  |                |         |
|                | Gamma shift transform을 적용<br/>감마값 = 1/2.2                                                                            |                |         |
| INV_GAMMA      | 역감마 적용                                                                                                                |                |         |
|                | Inverse Gamma shift transform을 적용<br/>감마값 = 2.2                                                                      |                |         |
| INV            | 색상 반전                                                                                                                  |                |         |
|                | 색상을 반전시킴.                                                                                                           |                |         |

### 10.9.6.5.3 네온 효과

네온 효과 정보를 가지고 있는 요소이다.

### 표 234 — glow 요소

| 속성 이름 | 설명                         |
| --------- | ---------------------------- |
| alpha     | 투명도                       |
| radius    | 네온 크기<br/>단위는 HWPUNIT |

### 표 235 — glow 하위 요소

| 하위 요소 이름 | 설명                            |
| -------------- | ------------------------------- |
| effectsColor   | 네온 색상<br/>10.9.6.5.2.3 참조 |

### 샘플 130 — glow 예

```xml
<hp:glow alpha="0.5" radius="1000">
  <hp:effectsColor type="RGB" schemeIdx="-1" systemIdx="-1" presetIdx="-1">
    <hp:rgb r="178" g="178" b="178"/>
    <hp:effect type="SAT_MOD" value="1.75"/>
  </hp:effectsColor>
</hp:glow>
```

### 10.9.6.5.4 부드러운 가장자리 효과

부드러운 가장자리 효과 정보를 가지고 있는 요소이다.

### 표 236 — softEdge 요소

| 속성 이름 | 설명                                      |
| --------- | ----------------------------------------- |
| radius    | 부드러운 가장자리 크기<br/>단위는 HWPUNIT |

### 샘플 131 — softEdge 예

```xml
<hp:softEdge radius="500"/>
```

### 10.9.6.5.5 반사 효과

반사 효과 정보를 가지고 있는 요소이다.

### 표 237 — reflection 요소

| 속성 이름     | 설명                                              |
| ------------- | ------------------------------------------------- |
| alignStyle    | 반사된 그림 위치                                  |
| radius        | 흐릿함 정도<br/>단위는 HWPUNIT                    |
| direction     | 반사된 그림 방향 각도                             |
| distance      | 대상과 반사된 그림 사이의 거리<br/>단위는 HWPUNIT |
| rotationStyle | 도형과 함께 회전할 것인지 여부                    |
| fadeDirection | 오프셋 방향                                       |

### 표 238 — reflection 하위 요소

| 하위 요소 이름 | 설명                            |
| -------------- | ------------------------------- |
| skew           | 기울기<br/>10.9.6.5.2.1 참조    |
| scale          | 확대 비율<br/>10.9.6.5.2.2 참조 |
| alpha          | 투명도                          |
| pos            | 위치                            |

### 샘플 132 — reflection 예

```xml
<hp:reflection alignStyle="BOTTOM_LEFT" radius="50" direction="90" distance="400"
  rotationStyle="0" fadeDirection="90">
  <hp:skew x="0" y="0"/>
  <hp:scale x="1" y="0"/>
  <hp:alpha start="0.5" end="0.997"/>
  <hp:pos start="0" end="0.75"/>
</hp:reflection>
```

#### • 투명도 설정

반사 효과 사용 시 투명도 설정 정보를 가지고 있는 요소이다.

### 표 239 — alpha 요소

| 속성 이름 | 설명             |
| --------- | ---------------- |
| start     | 시작 위치 투명도 |
| end       | 끝 위치 투명도   |

### 샘플 133 — alpha 예

```xml
<hp:alpha start="0.5" end="0.997"/>
```

#### • 반사 효과 위치 설정

반사된 영상이 투영될 위치 정보를 가지고 있는 요소이다.

### 표 240 — pos 요소

| 속성 이름 | 설명      |
| --------- | --------- |
| start     | 시작 위치 |
| end       | 끝 위치   |

### 샘플 134 — pos 예

```xml
<hp:pos start="0" end="0.75"/>
```

### 10.9.6.6 이미지 원본 정보

원본 그림의 크기 정보를 가지고 있는 요소이다.

### 표 241 — imgDim 요소

| 속성 이름 | 설명      |
| --------- | --------- |
| dimwidth  | 원본 너비 |
| dimheight | 원본 높이 |

### 샘플 135 — imgDim 예

```xml
<hp:imgDim dimwidth="96000" dimheight="54000"/>
```

## 10.9.7 ole 요소

### 10.9.7.1 ole

`<ole>` 요소는 [AbstractShapeComponentType]을 상속받는다. [AbstractShapeComponentType]의 자세한 내용은 **10.9.5**를 참조한다.

### 표 242 — ole 요소

| 속성 이름       | 설명                                          |
| --------------- | --------------------------------------------- |
| objectType      | OLE 객체 종류                                 |
| binaryItemIDRef | OLE 객체 바이너리 데이터에 대한 아이디 참조값 |
| hasMoniker      | moniker가 설정되어 있는지 여부                |
| drawAspect      | 화면에 어떤 형태로 표시될지에 대한 설정       |
| eqBaseLine      | 베이스 라인                                   |

### 표 243 — ole 하위 요소

| 하위 요소 이름 | 설명                        |
| -------------- | --------------------------- |
| extent         | 오브젝트 자체의 extent 크기 |
| lineShape      | 테두리선 모양.              |

### 샘플 136 — ole 예

```xml
<hp:ole id="1790881811" zOrder="3" numberingType="PICTURE" textWrap="SQUARE" textFlow="BOTH_SIDES"
  lock="0" dropcapstyle="None" href="" groupLevel="0" instid="717139988" objectType="EMBEDDED"
  binaryItemIDRef="ole2" hasMoniker="0" drawAspect="CONTENT" eqBaseLine="0">
  <hp:offset x="0" y="0"/>
  <hp:orgSz width="14176" height="14176"/>
  <hp:curSz width="0" height="0"/>
  <hp:flip horizontal="0" vertical="0"/>
  <hp:rotationInfo angle="0" centerX="7088" centerY="7088" rotateimage="1"/>
  <hp:renderingInfo>
    <hp:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:scaMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
  </hp:renderingInfo>
  <hp:extent x="14176" y="14176"/>
  <hp:lineShape color="#0000FF" width="1133" style="DASH_DOT" endCap="ROUND" headStyle="NORMAL"
    tailStyle="NORMAL" headfill="0" tailfill="0" headSz="SMALL_SMALL" tailSz="SMALL_SMALL"
    outlineStyle="OUTER" alpha="0"/>
  <hp:sz width="14176" widthRelTo="ABSOLUTE" height="14176" heightRelTo="ABSOLUTE" protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0"
    vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:shapeComment>OLE 객체입니다. 객체 형식은 Bitmap Image입니다.</hp:shapeComment>
</hp:ole>
```

### 10.9.7.2 extent 요소

OLE 객체의 확장 크기 정보를 가지고 있는 요소이다.

### 표 244 — extent 요소

| 속성 이름 | 설명                          |
| --------- | ----------------------------- |
| x         | 오브젝트 자체의 extent x 크기 |
| y         | 오브젝트 자체의 extent y 크기 |

### 샘플 137 — extent 예

```xml
<hp:extent x="14176" y="14176"/>
```

## 10.9.8 container 요소

`<container>` 요소는 [AbstractShapeComponentType]을 상속받는다. [AbstractShapeComponentType]의 자세한 내용은 **10.9.5**를 참조한다.

`<container>` 요소는 다른 도형 객체를 묶기 위해서 사용되는 객체이다. `<container>` 요소로 묶을 수 있는 객체에는 컨테이너 객체 자신과, 선, 사각형, 타원, 호, 다각형, 곡선, 연결선과 같은 그리기 객체, 그림, OLE 객체가 있다.

### 표 245 — container 요소

| 하위 요소 이름 | 설명                 |
| -------------- | -------------------- |
| container      | 컨테이너 객체        |
| line           | 그리기 객체 — 선     |
| rect           | 그리기 객체 — 사각형 |
| ellipse        | 그리기 객체 — 타원   |
| arc            | 그리기 객체 — 호     |
| polygon        | 그리기 객체 — 다각형 |
| curve          | 그리기 객체 — 곡선   |
| connectLine    | 그리기 객체 — 연결선 |
| pic            | 그림                 |
| ole            | OLE 객체             |

### 샘플 138 — container 예

```xml
<hp:container id="1615476006" zOrder="1" numberingType="PICTURE"
  textWrap="IN_FRONT_OF_TEXT" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href=""
  groupLevel="0" instid="541734183">
  <hp:sz width="31160" widthRelTo="ABSOLUTE" height="12660" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="0" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PAPER" horzRelTo="PAPER" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="10540" horzOffset="11734"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:caption side="BOTTOM" fullSz="0" width="8504" gap="850" lastWidth="31160">
    <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="TOP"
      linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
      <hp:p id="0" paraPrIDRef="19" styleIDRef="21" pageBreak="0" columnBreak="0"
        merged="0">
        <hp:run charPrIDRef="7">
          <hp:t>ShapeCompContainer</hp:t>
        </hp:run>
      </hp:p>
    </hp:subList>
  </hp:caption>
  <hp:shapeComment>묶음 개체입니다.</hp:shapeComment>
  ......
  <hp:rect id="2" zOrder="0" numberingType="NONE" textWrap="TOP_AND_BOTTOM"
    textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="1" instid="541734179"
    ratio="20">
    ......
  </hp:rect>
  <hp:ellipse id="7602208" zOrder="0" numberingType="NONE" textWrap="TOP_AND_BOTTOM"
    textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="1" instid="541734181"
    intervalDirty="0" hasArcPr="0" arcType="NORMAL">
    ......
  </hp:ellipse>
</hp:container>
```

## 10.9.9 chart 요소

`<chart>` 요소는 **10.9.2**를 상속받는다. [AbstractShapeObjectType]의 자세한 내용은 **10.9.2**를 참조한다.

`<chartIDRef>`는 차트 데이터에 대한 아이디 참조값으로 차트에 대한 xml 데이터는 OOXML의 형식을 사용하며 Chart/chart.xml (**8.2** 참조)에 기입된다.

### 표 246 — chart 요소

| 속성 이름  | 설명                             |
| ---------- | -------------------------------- |
| chartIDRef | 차트 데이터에 대한 아이디 참조값 |
| version    | 차트 버전                        |

### 샘플 139 — chart 예

```xml
<hp:chart id="1811647071" zOrder="6" numberingType="PICTURE" textWrap="SQUARE"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" chartIDRef="Chart/chart1.xml">
  <hp:sz width="32250" widthRelTo="ABSOLUTE" height="18750" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="0"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
</hp:chart>
```

## 10.10 그리기 객체

### 10.10.1 그리기 객체

그리기 객체는 연결선, 사각형, 원 등과 같은 기본 도형 객체보다 더 구체화된 도형 객체이다. 그리기 객체는 기본 도형 객체의 공통 속성을 모두 상속받으며 그리기 객체만을 위한 속성을 추가적으로 더 정의해서 사용한다.

### 10.10.2 AbstractDrawingObjectType

### 10.10.2.1 AbstractDrawingObjectType

[AbstractDrawingObjectType]은 그리기 객체의 기본 속성을 정의하고 있는 요소 형식이다. [AbstractDrawingObjectType]은 [AbstractShapeComponentType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장한다. [AbstractShapeComponentType]의 자세한 내용은 **10.9.5**를 참조한다.

[AbstractDrawingObjectType]은 추상 형식이므로 [AbstractDrawingObjectType]만으로는 XML 요소를 생성할 수 없다.

### 표 247 — AbstractDrawingObjectType 요소

| 하위 요소 이름 | 설명                                          |
| -------------- | --------------------------------------------- |
| lineShape      | 그리기 객체의 테두리선 정보<br/>10.9.6.2 참조 |
| fillBrush      | 그리기 객체의 채우기 정보                     |
| drawText       | 그리기 객체 글상자용 텍스트                   |
| shadow         | 그리기 객체의 그림자 정보                     |

### 샘플 140 — AbstractDrawingObjectType 예

```xml
<hp:lineShape color="#000000" width="33" style="SOLID" endCap="FLAT" headStyle="NORMAL" tailStyle="NORMAL"
  headfill="1" tailfill="1" headSz="MEDIUM_MEDIUM" tailSz="MEDIUM_MEDIUM" outlineStyle="NORMAL" alpha="0"/>
<hp:fillBrush>
  <hc:winBrush faceColor="#FFFFFF" hatchColor="#000000" alpha="0"/>
</hp:fillBrush>
<hp:shadow type="NONE" color="#B2B2B2" offsetX="0" offsetY="0" alpha="0"/>
<hp:drawText lastWidth="34260" name="" editable="0">
```

### 10.10.2.2 채우기 정보

#### 10.10.2.2.1 채우기

그리기 객체에서 객체의 면 영역에서 사용될 채우기 효과 정보를 가지고 있는 요소이다.

### 표 248 — fillBrush 요소

| 하위 요소 이름 | 설명            |
| -------------- | --------------- |
| winBrush       | 면 채우기       |
| gradation      | 그러데이션 효과 |
| imgBrush       | 그림으로 채우기 |

#### 10.10.2.2.2 면 채우기 정보

채우기 효과 중 단색 또는 무늬가 입혀진 단색으로 채우는 효과 정보를 가지고 있는 요소이다.

### 표 249 — winBrush 요소

| 속성 이름  | 설명      |
| ---------- | --------- |
| faceColor  | 면색      |
| hatchColor | 무늬 색   |
| hatchStyle | 무늬 종류 |
| alpha      | 투명도    |

### 표 250 — gradation 요소

| 속성 이름  | 설명                                 |
| ---------- | ------------------------------------ |
| type       | 그러데이션 유형                      |
| angle      | 그러데이션 기울기(시작각)            |
| centerX    | 그러데이션 가로 중심(중심 X 좌표)    |
| centerY    | 그러데이션 가로 중심(중심 Y 좌표)    |
| step       | 그러데이션 번짐 정도(0 ~ 255)        |
| colorNum   | 그러데이션 색상 수                   |
| stepCenter | 그러데이션 번짐 정도의 중심(0 ~ 100) |
| alpha      | 투명도                               |

### 표 251 — gradation 하위 요소

| 하위 요소 이름 | 설명            |
| -------------- | --------------- |
| color          | 그러데이션 색상 |

### • 그러데이션 색상

그러데이션 색상으로 표현하기 위한 요소로, 점진적으로 또는 단계적으로 변화하는 색상 중 시작 색, 또는 끝 색, 중간 단계 색 등을 표현한다.

### 표 252 — color 요소

| 속성 이름 | 설명   |
| --------- | ------ |
| value     | 색상값 |

## 10.10.2.2.4 그림으로 채우기 정보

그림으로 특정 부분을 채울 때 사용되는 요소로, 지정된 그림을 지정된 효과를 사용해서 채운다. 사용할 수 있는 효과에는 '크기에 맞추어', '위/가운데/아래로', '바둑판식으로' 등이 있다.

### 표 253 — imgBrush 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| mode      | 채우기 유형 |

### 표 254 — imgBrush 하위 요소

| 하위 요소 이름 | 설명                                    |
| -------------- | --------------------------------------- |
| img            | 그림 정보<br/>9.3.3.2.4의 img 요소 참조 |

## 10.10.2.3 그리기 객체 글상자용 텍스트

### 10.10.2.3.1 글상자용 텍스트

그리기 객체 안쪽 또는 특정 영역에 표시되는 글상자 내용을 가지고 있는 요소이다.

### 표 255 — drawText 요소

| 속성 이름 | 설명                                       |
| --------- | ------------------------------------------ |
| lastWidth | 텍스트 문자열의 최대 폭<br/>단위는 HWPUNIT |
| name      | 글상자 이름                                |
| editable  | 편집 가능 여부                             |

### 표 256 — drawText 하위 요소

| 하위 요소 이름 | 설명                          |
| -------------- | ----------------------------- |
| textMargin     | 글상자 텍스트 여백            |
| subList        | 글상자 텍스트<br/>11.1.2 참조 |

### 샘플 141 — drawText 예

```xml
<hp:drawText lastWidth="12540" name="" editable="0">
  <hp:subList id="" textDirection="HORIZONTAL" lineWrap="BREAK" vertAlign="CENTER"
    linkListIDRef="0" linkListNextIDRef="0" textWidth="0" textHeight="0" hasTextRef="0" hasNumRef="0">
    <hp:p id="0" paraPrIDRef="20" styleIDRef="0" pageBreak="0" columnBreak="0" merged="0">
      <hp:run charPrIDRef="8">
        <hp:t>Rectangle</hp:t>
      </hp:run>
    </hp:p>
  </hp:subList>
  <hp:textMargin left="283" right="283" top="283" bottom="283"/>
</hp:drawText>
```

### 10.10.2.3.2 글상자 텍스트 여백

`<textMargin>` 요소는 [MarginAttributeGroup]을 속성으로 포함한다. [MarginAttributeGroup]은 10.6.6.2를 참조한다.

### 표 257 — textMargin 요소

| 속성 이름              | 설명          |
| ---------------------- | ------------- |
| [MarginAttributeGroup] | 10.6.6.2 참조 |

### 샘플 142 — textMargin 예

```xml
<hp:textMargin left="283" right="283" top="283" bottom="283"/>
```

## 10.10.2.4 그리기 객체의 그림자 정보

그리기 객체에 적용될 그림자 효과 정보를 가지고 있는 요소이다.

### 표 258 — shadow 요소

| 속성 이름 | 설명                       |
| --------- | -------------------------- |
| type      | 그림자 종류                |
| color     | 그림자 색                  |
| offsetX   | 그림자 간격 x<br/>단위는 % |
| offsetY   | 그림자 간격 y<br/>단위는 % |
| alpha     | 투명도                     |

### 샘플 143 — shadow 예

```xml
<hp:shadow type="PARELLEL_RIGHTBOTTOM" color="#B2B2B2" offsetX="1000" offsetY="500" alpha="0"/>
```

## 10.10.3 그리기 객체 — 선

`<line>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 259 — line 요소

| 속성 이름   | 설명                                                                                                                                           |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| isReverseHV | 처음 생성 시 수직선 또는 수평선일 때,<br/>선의 방향이 언제나 오른쪽(위쪽)으로<br/>잡힘으로 인한 현상 때문에 방향을<br/>바로 잡아주기 위한 속성 |

### 표 260 — line 하위 요소

| 하위 요소 이름 | 설명                       |
| -------------- | -------------------------- |
| startPt        | 시작점<br/>10.9.6.3.2 참조 |
| endPt          | 끝점<br/>10.9.6.3.2 참조   |

### 샘플 144 — line 예

```xml
<hp:line id="1480891240" zOrder="1" numberingType="PICTURE" textWrap="IN_FRONT_OF_TEXT"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="407149417"
  isReverseHV="0">
  ......
  <hp:startPt x="0" y="0"/>
  <hp:endPt x="4686" y="9102"/>
</hp:line>
```

## 10.10.4 그리기 객체 — 사각형

`<rect>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 261 — rect 요소

| 속성 이름 | 설명                                                                       |
| --------- | -------------------------------------------------------------------------- |
| ratio     | 사각형 모서리 곡률<br/>단위는 %<br/>직각은 0, 둥근 모양은 20, 반원은 50 등 |

### 표 262 — rect 하위 요소

| 하위 요소 이름 | 설명                             |
| -------------- | -------------------------------- |
| pt0            | 첫 번째 좌표<br/>10.9.6.3.2 참조 |
| pt1            | 두 번째 좌표<br/>10.9.6.3.2 참조 |
| pt2            | 세 번째 좌표<br/>10.9.6.3.2 참조 |
| pt3            | 네 번째 좌표<br/>10.9.6.3.2 참조 |

### 샘플 145 — rect 예

```xml
<hp:rect id="1480891242" zOrder="2" numberingType="PICTURE" textWrap="IN_FRONT_OF_TEXT"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="407149419"
  ratio="0">
  ......
  <hp:pt0 x="0" y="0"/>
  <hp:pt1 x="12838" y="0"/>
  <hp:pt2 x="12838" y="9306"/>
  <hp:pt3 x="0" y="9306"/>
</hp:rect>
```

## 10.10.5 그리기 객체 — 타원

`<ellipse>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 263 — ellipse 요소

| 속성 이름      | 설명                                                                                                                       |
| -------------- | -------------------------------------------------------------------------------------------------------------------------- |
| intervalDirty  | 호(arc)로 바뀌었을 때, interval을 다시 계산해야<br/>할 필요가 있는지 여부<br/>interval: 원 위에 존재하는 두 점 사이의 거리 |
| hasArcProperty | 호로 바뀌었는지 여부                                                                                                       |
| arcType        | 호의 종류<br/>NORMAL: 호, PIE: 부채꼴, CHORD: 활                                                                           |

### 표 264 — ellipse 하위 요소

| 하위 요소 이름 | 설명                                 |
| -------------- | ------------------------------------ |
| center         | 중심 좌표<br/>10.9.6.3.2 참조        |
| ax1            | 제1축 좌표<br/>10.9.6.3.2 참조       |
| ax2            | 제2축 좌표<br/>10.9.6.3.2 참조       |
| start1         | 시작 지점 1 좌표<br/>10.9.6.3.2 참조 |
| end1           | 시작 지점 2 좌표<br/>10.9.6.3.2 참조 |
| start2         | 끝 지점 1 좌표<br/>10.9.6.3.2 참조   |
| end2           | 끝 지점 2 좌표<br/>10.9.6.3.2 참조   |

### 샘플 146 — ellipse 예

```xml
<hp:ellipse id="1480891244" zOrder="3" numberingType="PICTURE"
  textWrap="IN_FRONT_OF_TEXT" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href=""
  groupLevel="0" instid="407149421" intervalDirty="0" hasArcPr="0" arcType="NORMAL">
  ......
  <hp:center x="4925" y="3973"/>
  <hp:ax1 x="9850" y="3973"/>
  <hp:ax2 x="4925" y="0"/>
  <hp:start1 x="0" y="1337540795"/>
  <hp:end1 x="1144072527" y="-1432413552"/>
  <hp:start2 x="-1105998402" y="100663296"/>
  <hp:end2 x="393344" y="2"/>
</hp:ellipse>
```

## 10.10.6 그리기 객체 — 호

`<arc>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 265 — arc 요소

| 속성 이름 | 설명                                             |
| --------- | ------------------------------------------------ |
| type      | 호의 종류<br/>NORMAL: 호, PIE: 부채꼴, CHORD: 활 |

### 표 266 — arc 하위 요소

| 하위 요소 이름 | 설명                           |
| -------------- | ------------------------------ |
| center         | 중심 좌표<br/>10.9.6.3.2 참조  |
| ax1            | 제1축 좌표<br/>10.9.6.3.2 참조 |
| ax2            | 제2축 좌표<br/>10.9.6.3.2 참조 |

### 샘플 147 — arc 예

```xml
<hp:arc id="1480891246" zOrder="4" numberingType="PICTURE" textWrap="IN_FRONT_OF_TEXT"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="407149423"
  type="NORMAL">
  ......
  <hp:center x="0" y="0"/>
  <hp:ax1 x="0" y="9645"/>
  <hp:ax2 x="11411" y="0"/>
</hp:arc>
```

## 10.10.7 그리기 객체 — 다각형

`<polygon>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 267 — polygon 요소

| 하위 요소 이름 | 설명                            |
| -------------- | ------------------------------- |
| pt             | 다각형 좌표<br/>10.9.6.3.2 참조 |

### 샘플 148 — polygon 예

```xml
<hp:polygon id="1480891248" zOrder="5" numberingType="PICTURE"
  textWrap="IN_FRONT_OF_TEXT" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href=""
  groupLevel="0" instid="407149425">
  ......
  <hp:pt x="3261" y="0"/>
  <hp:pt x="0" y="3872"/>
  <hp:pt x="3329" y="7744"/>
  <hp:pt x="11547" y="7540"/>
  <hp:pt x="11427" y="204"/>
  <hp:pt x="3261" y="0"/>
</hp:polygon>
```

## 10.10.8 그리기 객체 — 곡선

### 10.10.8.1 곡선

`<curve>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 268 — curve 요소

| 하위 요소 이름 | 설명          |
| -------------- | ------------- |
| seg            | 곡선 세그먼트 |

### 샘플 149 — curve 예

```xml
<hp:curve id="1480891254" zOrder="6" numberingType="PICTURE"
  textWrap="IN_FRONT_OF_TEXT" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href=""
  groupLevel="0" instid="407149431">
  ......
  <hp:seg type="CURVE" x1="274" y1="1485" x2="1429" y2="10859"/>
  <hp:seg type="CURVE" x1="1429" y1="10859" x2="3263" y2="8821"/>
  <hp:seg type="CURVE" x1="3263" y1="8821" x2="5233" y2="11199"/>
  <hp:seg type="CURVE" x1="5233" y1="11199" x2="5980" y2="1010"/>
  <hp:seg type="CURVE" x1="5980" y1="1010" x2="274" y2="1485"/>
</hp:curve>
```

### 10.10.8.2 곡선 세그먼트

그리기 객체 중 곡선을 표현할 때 곡선의 단위 곡선의 시작점 및 끝점을 표현하기 위한 요소이다.

### 표 269 — seg 요소

| 속성 이름 | 설명                                           |
| --------- | ---------------------------------------------- |
| type      | 곡선 세그먼트 형식<br/>curve: 곡선, line: 직선 |
| x1        | 곡선 세그먼트 시작점 x 좌표                    |
| y1        | 곡선 세그먼트 시작점 y 좌표                    |
| x2        | 곡선 세그먼트 끝점 x 좌표                      |
| y2        | 곡선 세그먼트 끝점 y 좌표                      |

### 샘플 150 — seg 예

```xml
<hp:seg type="CURVE" x1="274" y1="1485" x2="1429" y2="10859"/>
```

## 10.10.9 그리기 객체 — 연결선

### 10.10.9.1 연결선

`<connectLine>` 요소는 [AbstractDrawingObjectType]을 상속받는다. [AbstractDrawingObjectType]의 자세한 내용은 10.10.2를 참조한다.

### 표 270 — connectLine 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| type      | 연결선 형식 |

### 표 271 — connectLine 하위 요소

| 하위 요소 이름 | 설명               |
| -------------- | ------------------ |
| startPt        | 연결선 시작점 정보 |
| endPt          | 연결선 끝점 정보   |
| controlPoints  | 연결선 조절점 정보 |

### 샘플 151 — connectLine 예

```xml
<hp:connectLine id="1480891256" zOrder="7" numberingType="PICTURE"
  textWrap="IN_FRONT_OF_TEXT" textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href=""
  groupLevel="0" instid="407149433" type="STRAIGHT_NOARROW">
  ......
  <hp:startPt x="10" y="4154" subjectIDRef="407149431" subjectIdx="0"/>
  <hp:endPt x="0" y="0" subjectIDRef="407149421" subjectIdx="2"/>
  <hp:controlPoints>
    <hp:point x="0" y="4144" type="3"/>
    <hp:point x="0" y="0" type="26"/>
  </hp:controlPoints>
</hp:connectLine>
```

### 10.10.9.2 연결선 연결점 정보

[ConnectPointType]은 [PointType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장한다. [PointType]의 자세한 내용은 10.9.6.3.2를 참조한다.

### 표 272 — ConnectPointType 요소

| 속성 이름    | 설명                                        |
| ------------ | ------------------------------------------- |
| subjectIDRef | 시작/끝부분과 연결되는 대상의 아이디 참조값 |
| subjectIdx   | 시작/끝부분과 연결되는 대상의 연결점 index  |

### 샘플 152 — ConnectPointType 예

```xml
<hp:startPt x="0" y="0" subjectIDRef="0" subjectIdx="0"/>
<hp:endPt x="15402" y="10581" subjectIDRef="0" subjectIdx="0"/>
```

### 10.10.9.3 연결선 연결점 정보

[ConnectControlPointType]은 [PointType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장한다. [PointType]의 자세한 내용은 오류! 참조 원본을 찾을 수 없습니다.을 참조한다.

### 표 273 — ConnectControlPointType 속성

| 속성 이름 | 설명        |
| --------- | ----------- |
| type      | 조절점 종류 |

### 샘플 153 — ConnectControlPointType 예

```xml
<hp:controlPoints>
  <hp:point x="2446" y="0" type="3"/>
  <hp:point x="2446" y="2207" type="2"/>
  <hp:point x="0" y="2207" type="2"/>
  <hp:point x="0" y="7035" type="26"/>
</hp:controlPoints>
```

[ConnectControlPointType]의 하위 속성인 type은 조절점 종류에 대한 값을 가지고 있다.

### 표 274 — type 값

| type 값    | 설명   |
| ---------- | ------ |
| 0x00000001 | 시작점 |
| 0x00000002 | 직선   |
| 0x00000018 | 끝점   |

## 10.11 양식 객체

### 10.11.1 AbstractFormObjectType

[AbstractFormObjectType]은 추상 형식이므로 [AbstractFormObjectType]만으로는 XML 요소를 생성할 수 없다.

### 표 275 — AbstractFormObjectType 요소

| 속성 이름       | 설명                                                                     |
| --------------- | ------------------------------------------------------------------------ |
| name            | 이름                                                                     |
| foreColor       | 전경색                                                                   |
| backColor       | 배경색                                                                   |
| groupName       | 그룹 이름                                                                |
| tabStop         | 탭키로 객체들을 이동할 때 해당 객체에<br/>머물 수 있는지를 결정하는 속성 |
| editable        | 편집 가능 여부                                                           |
| tabOrder        | 탭키 이동 순서                                                           |
| enabled         | 활성화 여부                                                              |
| borderTypeIDRef | 테두리 아이디 참조값                                                     |
| drawFrame       | 프레임 표시 가능 여부                                                    |
| printable       | 출력 가능 여부                                                           |

### 표 276 — AbstractFormObjectType 하위 요소

| 하위 요소 이름 | 설명                  |
| -------------- | --------------------- |
| formCharPr     | 양식 객체의 글자 속성 |

### 샘플 154 — AbstractFormObjectType 요소

```xml
<hp:btn caption="명령 단추1" radioGroupName="" triState="0" name="PushButton1" foreColor="#000000"
  backColor="#F0F0F0" groupName="" tabStop="1" editable="1" tabOrder="1" enabled="1" borderTypeIDRef="4"
  drawFrame="1" printable="1" command="">
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
  <hp:sz width="7087" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE" protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1" holdAnchorAndSO="0"
    vertRelTo="PARA" horzRelTo="PARA" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="1133" right="1133" top="1133" bottom="1133"/>
</hp:btn>
```

### 10.11.1.2 양식 객체의 글자 속성

양식 객체의 글자 속성 설정 정보를 가지고 있는 요소이다.

### 표 277 — formCharPr 요소

| 속성 이름     | 설명                                           |
| ------------- | ---------------------------------------------- |
| charPrIDRef   | 글자 모양 아이디 참조값                        |
| followContext | 양식 객체가 주위의 글자 속성을<br/>따를지 여부 |
| autoSize      | 자동 크기 조절 여부                            |
| wordWrap      | 줄 내림 여부                                   |

### 샘플 155 — formCharPr 예

```xml
<hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
```

## 10.11.2 AbstractButtonObjectType

[AbstractButtonObjectType]은 버튼 양식 객체의 공통 속성을 정의한다. [AbstractButtonObjectType]은 [AbstractFormObjectType]을 기본 형식으로 가지고 추가적으로 필요한 속성이나 요소를 확장한다. [AbstractFormObjectType]의 자세한 내용은 10.11.1를 참조한다.

[AbstractButtonObjectType]은 추상 형식이므로 [AbstractButtonObjectType]만으로는 XML 요소를 생성할 수 없다.

### 표 278 — AbstractButtonObjectType 요소

| 속성 이름      | 설명                  |
| -------------- | --------------------- |
| caption        | 캡션                  |
| value          | 체크 상태 값          |
| radioGroupName | 라디오 버튼 그룹 이름 |
| triState       | 3단 체크 상태 여부    |
| backStyle      | 버튼 배경색 스타일    |

## 10.11.3 양식 객체 — 버튼

`<btn>` 요소는 [AbstractButtonObjectType]을 상속받는다. [AbstractButtonObjectType]의 자세한 내용은 10.11.2를 참조한다.

### 샘플 156 — btn 예

```xml
<hp:btn caption="명령 단추 1" value="UNCHECKED" radioGroupName="" triState="0"
  backStyle="TRANSPARENT" name="PushButton1" foreColor="#000000" backColor="#F0F0F0"
  groupName="" tabStop="1" tabOrder="1" enabled="1" borderTypeIDRef="4" drawFrame="1"
  printable="1">
  <hp:sz width="7087" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
</hp:btn>
```

## 10.11.4 양식 객체 — 라디오 버튼

`<radioBtn>` 요소는 [AbstractButtonObjectType]을 상속받는다. [AbstractButtonObjectType]의 자세한 내용은 10.11.2를 참조한다.

### 샘플 157 — radioBtn 예

```xml
<hp:radioBtn caption="라디오 단추1" value="UNCHECKED" radioGroupName="" triState="0"
  backStyle="OPAQUE" name="RadioButton1" foreColor="#000000" backColor="#FFFFFF"
  groupName="" tabStop="1" tabOrder="4" enabled="1" borderTypeIDRef="0" drawFrame="1"
  printable="1">
  <hp:sz width="8504" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
</hp:radioBtn>
```

## 10.11.5 양식 객체 — 체크 버튼

`<checkBtn>` 요소는 [AbstractButtonObjectType]을 상속받는다. [AbstractButtonObjectType]의 자세한 내용은 10.11.2를 참조한다.

### 샘플 158 — checkBtn 예

```xml
<hp:checkBtn caption="선택 상자1" value="UNCHECKED" radioGroupName="" triState="0"
  backStyle="OPAQUE" name="CheckBox1" foreColor="#000000" backColor="#FFFFFF"
  groupName="" tabStop="1" tabOrder="2" enabled="1" borderTypeIDRef="0" drawFrame="1"
  printable="1">
  <hp:sz width="9921" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
</hp:checkBtn>
```

## 10.11.6 양식 객체 — 콤보 박스

### 10.11.6.1 콤보 박스

`<comboBox>` 요소는 [AbstractFormObjectType]을 상속받는다. [AbstractFormObjectType]의 자세한 내용은 10.11.1을 참조한다.

### 표 279 — comboBox 요소

| 속성 이름     | 설명                                            |
| ------------- | ----------------------------------------------- |
| listBoxRows   | 콤보 박스가 펼쳐졌을 때 최대로<br/>보이는 줄 수 |
| listBoxWidth  | 콤보 박스가 펼쳐졌을 때 최대로<br/>보이는 넓이  |
| editEnable    | 텍스트 수정 가능 여부                           |
| selectedValue | 콤보 박스 아이템 중에서 선택된 값               |

### 표 280 — comboBox 하위 요소

| 하위 요소 이름 | 설명                     |
| -------------- | ------------------------ |
| listItem       | 콤보 박스의 아이템 목록. |

### 샘플 159 — comboBox 예

```xml
<hp:comboBox listBoxRows="10" listBoxWidth="0" editEnable="1" selectedValue=""
  name="ComboBox1" foreColor="#000000" backColor="#F0F0F0" groupName="" tabStop="1"
  tabOrder="3" enabled="1" borderTypeIDRef="5" drawFrame="1" printable="1">
  <hp:sz width="9921" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
  <hp:listItem displayText="" value=""/>
</hp:comboBox>
```

### 10.11.6.2 콤보/리스트 박스의 아이템

양식 객체 중 콤보 박스 및 리스트 박스에서 항목(아이템)을 표현하기 위한 객체이다.

### 표 281 — listItem 요소

| 속성 이름   | 설명                                                    |
| ----------- | ------------------------------------------------------- |
| displayText | 화면에 표시될 아이템 내용                               |
| value       | 아이템이 선택되었을 때 콤보/리스트<br/>박스가 가지는 값 |

## 10.11.7 양식 객체 — 리스트 박스

`<listBox>` 요소는 [AbstractFormObjectType]을 상속받는다. [AbstractFormObjectType]의 자세한 내용은 10.11.1을 참조한다.

### 표 282 — listBox 요소

| 속성 이름     | 설명                                                 |
| ------------- | ---------------------------------------------------- |
| selectedValue | 현재 선택된 아이템의 값                              |
| itemHeight    | 리스트 박스 아이템 높이                              |
| topIdx        | 리스트 박스에서 첫 번째로 보이는<br/>아이템의 인덱스 |

### 표 283 — listBox 하위 요소

| 하위 요소 이름 | 설명                                          |
| -------------- | --------------------------------------------- |
| listItem       | 리스트 박스의 아이템 목록.<br/>10.11.6.2 참조 |

## 10.11.8 양식 객체 — 에디트

### 10.11.8.1 에디트

`<edit>` 요소는 [AbstractFormObjectType]을 상속받는다. [AbstractFormObjectType]의 자세한 내용은 10.11.1을 참조한다.

### 표 284 — edit 요소

| 속성 이름      | 설명                                                                                                            |
| -------------- | --------------------------------------------------------------------------------------------------------------- |
| multiLine      | 다중 줄 허용 여부                                                                                               |
| passwordChar   | 에디트를 패스워드 입력으로 사용할 때,<br/>입력한 글자 대신에 보이게 할 글자                                     |
| maxLength      | 입력 가능한 최대 글자수                                                                                         |
| scrollBars     | 스크롤바 표시 여부                                                                                              |
| tabKeyBehavior | 탭키를 눌렀을 때의 동작 방식<br/>NEXT_OBJECT: 다음 객체로 이동<br/>INSERT_TAB: 에디트 내용에 복수 글자 tab 추가 |
| numOnly        | 숫자만 입력 가능하게 할 것인지 여부                                                                             |
| readOnly       | 읽기 전용 여부                                                                                                  |
| alignText      | 텍스트 좌우 정렬 방식                                                                                           |

### 표 285 — edit 하위 요소

| 하위 요소 이름 | 설명                                                                                     |
| -------------- | ---------------------------------------------------------------------------------------- |
| text           | 에디트의 내용<br/>요소 값으로 텍스트 문자열을 가짐.<br/>해당 요소의 추가적인 설명은 생략 |

### 샘플 160 — edit 예

```xml
<hp:edit multiLine="0" passwordChar="" maxLength="2147483647" scrollBars="NONE"
  tabKeyBehavior="NEXT_OBJECT" numOnly="0" readOnly="0" alignText="LEFT" name="Edit1"
  foreColor="#000000" backColor="#F0F0F0" groupName="" tabStop="1" tabOrder="5" enabled="1"
  borderTypeIDRef="5" drawFrame="1" printable="1">
  <hp:sz width="7087" widthRelTo="ABSOLUTE" height="1984" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="1" affectLSpacing="0" flowWithText="1" allowOverlap="1"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:formCharPr charPrIDRef="7" followContext="0" autoSz="0" wordWrap="0"/>
  <hp:text>입력 상자</hp:text>
</hp:edit>
```

### 10.11.8.2 양식 객체 — 스크롤바

`<scrollBar>` 요소는 [AbstractFormObjectType]을 상속받는다. [AbstractFormObjectType]의 자세한 내용은 10.11.1을 참조한다.

### 표 286 — scrollBar 요소

| 속성 이름   | 설명                                                                   |
| ----------- | ---------------------------------------------------------------------- |
| delay       | 마우스 버튼 다운 후 스크롤이 연속적으<br/>로 일어날 때까지 걸리는 시간 |
| largeChange | Page Up/Down시 변화 값                                                 |
| smallChange | Line Up/Down시 변화 값                                                 |
| min         | 최소값                                                                 |
| max         | 최대값                                                                 |
| page        | 스크롤하는 1페이지의 크기                                              |
| value       | 현재 위치                                                              |
| type        | 스크롤바 형태(수평/수직)                                               |

## 10.12 그 외의 객체들

### 10.12.1 글맵시

#### 10.12.1.1 글맵시

글맵시는 글자를 구부리거나 글자에 외곽선, 면 채우기, 그림자, 회전 등의 효과를 주어 문자를 꾸미는 기능이다.

### 표 287 — textart 요소

| 속성 이름 | 설명        |
| --------- | ----------- |
| text      | 글맵시 내용 |

### 표 288 — textart 하위 요소

| 하위 요소 이름 | 설명                             |
| -------------- | -------------------------------- |
| pt0            | 첫 번째 좌표<br/>10.9.6.3.2 참조 |
| pt1            | 두 번째 좌표<br/>10.9.6.3.2 참조 |
| pt2            | 세 번째 좌표<br/>10.9.6.3.2 참조 |
| pt3            | 네 번째 좌표<br/>10.9.6.3.2 참조 |
| textartPr      | 글맵시 모양 정보                 |
| outline        | 외곽선 정보                      |

### 샘플 161 — textart 예

```xml
<hp:textart id="1790879993" zOrder="1" numberingType="PICTURE" textWrap="SQUARE"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="717138170" text="내용을
입력하세요.">
  <hp:offset x="0" y="0"/>
  <hp:orgSz width="14173" height="14173"/>
  <hp:curSz width="20500" height="5000"/>
  <hp:flip horizontal="0" vertical="0"/>
  <hp:rotationInfo angle="0" centerX="10250" centerY="2500" rotateimage="1"/>
  <hp:renderingInfo>
    <hp:transMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
    <hp:scaMatrix e1="1.446412" e2="0" e3="0" e4="0" e5="0.352783" e6="0"/>
    <hp:rotMatrix e1="1" e2="0" e3="0" e4="0" e5="1" e6="0"/>
  </hp:renderingInfo>
  <hp:lineShape color="#000000" width="0" style="NONE" endCap="ROUND" headStyle="NORMAL"
    tailStyle="NORMAL" headfill="0" tailfill="0" headSz="SMALL_SMALL" tailSz="SMALL_SMALL"
    outlineStyle="INNER" alpha="0"/>
  <hp:fillBrush>
    <hc:winBrush faceColor="#0000FF" hatchColor="#000000" alpha="0"/>
  </hp:fillBrush>
  <hp:shadow type="NONE" color="#B2B2B2" offsetX="0" offsetY="0" alpha="0"/>
  <hc:pt0 x="0" y="0"/>
  <hc:pt1 x="14173" y="0"/>
  <hc:pt2 x="14173" y="14173"/>
  <hc:pt3 x="0" y="14173"/>
  <hp:textartPr fontName="함초롬바탕" fontStyle="보통" fontType="TTF" textShape="WAVE2" lineSpacing="120"
    charSpacing="100" align="LEFT">
    <hp:shadow type="NONE" color="#000000" offsetX="0" offsetY="0" alpha="0"/>
  </hp:textartPr>
  <hp:sz width="20500" widthRelTo="ABSOLUTE" height="5000" heightRelTo="ABSOLUTE" protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="0" holdAnchorAndSO="0"
    vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT" vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="56" right="56" top="0" bottom="0"/>
  <hp:shapeComment>글맵시입니다.</hp:shapeComment>
</hp:textart>
```

#### 10.12.1.2 글맵시 모양 정보

글맵시 내의 글자에 적용될 효과 정보들을 가지고 있는 요소이다.

### 표 289 — textartPr 요소

| 속성 이름   | 설명        |
| ----------- | ----------- |
| fontName    | 글꼴 이름   |
| fontStyle   | 글꼴 스타일 |
| fontType    | 글꼴 형식   |
| textShape   | 글맵시 모양 |
| lineSpacing | 줄 간격     |
| spacing     | 글자 간격   |
| align       | 정렬 방식   |

### 표 290 — 하위 요소

| 하위 요소 이름 | 설명                                                                    |
| -------------- | ----------------------------------------------------------------------- |
| shadow         | 그림자 설정 정보.<br/>10.10.2.4 참조<br/>(단, offsetX, offset 단위는 %) |

### 샘플 162 — textartPr 예

```xml
<hp:textartPr fontName="한컴 소망 B" fontStyle="보통" fontType="TTF"
  textShape="DEFLATE_BOTTOM" lineSpacing="120" charSpacing="100" align="LEFT">
  <hp:shadow type="NONE" color="#000000" offsetX="0" offsetY="0" alpha="0"/>
</hp:textartPr>
```

#### 10.12.1.3 글맵시 외곽선 정보

글맵시의 외곽선에 대한 정보를 가지고 있는 요소이다.

### 표 291 — outline 요소

| 속성 이름 | 설명               |
| --------- | ------------------ |
| cnt       | 외곽선 포인트 개수 |

### 표 292 — outline 하위 요소

| 하위 요소 이름 | 설명                             |
| -------------- | -------------------------------- |
| pt             | 외곽선 좌표.<br/>10.9.6.3.2 참조 |

### 샘플 163 — outline 예

```xml
<hp:outline cnt="1">
  <hp:pt x="500" y="421"/>
</hp:outline>
```

### 10.12.2 글자 겹침

#### 10.12.2.1 글자 겹침

글자 겹침은 일반 글자판으로 입력할 수 없는 원 문자나 사각형 문자를 입력할 수 있게 하는 기능이다.

### 표 293 — compose 요소

| 속성 이름   | 설명                                         |
| ----------- | -------------------------------------------- |
| circleType  | 테두리 형식(상자/원)                         |
| charSz      | 테두리 내부 글자의 크기 비율<br/>단위는 10 % |
| composeType | 겹치기 종류                                  |
| charPrCnt   | 글자 모양 개수                               |

| 하위 요소 이름 | 설명             |
| -------------- | ---------------- |
| charPr         | 겹치기 글자 모양 |

### 샘플 164 — compose 예

```xml
<hp:compose circleType="SHAPE_REVERSAL_CIRCLE" charSz="-3" composeType="SPREAD" charPrCnt="10"
  composeText="111122">
  <hp:charPr prIDRef="8"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
  <hp:charPr prIDRef="4294967295"/>
</hp:compose>
```

#### 10.12.2.2 겹치기 글자 모양

겹쳐진 글자에 적용될 글자 모양에 대한 아이디 참조값을 가지고 있는 요소이다.

### 표 294 — charPr 요소

| 속성 이름 | 설명                    |
| --------- | ----------------------- |
| prIDRef   | 글자 모양 아이디 참조값 |

### 샘플 165 — charPr 예

```xml
<hp:charPr prIDRef="9"/>
```

### 10.12.3 덧말

덧말은 글의 전개로 보아서 본문의 내용 중에 넣기는 어려우나, 본문에서 인용한 자료의 출처를 밝히거나 본문에서 언급한 내용에 대한 간단한 내용의 보충 자료를 제시할 때 본문의 아래나 또는 위에 넣는 말이다. 덧말을 사용하면 일본어의 도시나 중국어의 발음기호 등을 쉽게 넣을 수 있다.

### 표 295 — dutmal 요소

| 속성 이름  | 설명                                                                                                                                                       |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| posType    | 덧말의 위치                                                                                                                                                |
| szRatio    | 덧말의 크기<br/>단위는 %                                                                                                                                   |
| option     | 덧말 글자의 글자 스타일을 지정하기 위한 속성<br/>스타일 지정 시에 4로 고정됨<br/>해당 속성은 속성이 존재하지 않거나, 속성이<br/>존재하면 4로 고정되어야 함 |
| styleIDRef | 글자 스타일 아이디 참조값                                                                                                                                  |
| align      | 정렬 방법                                                                                                                                                  |

### 표 296 — dutmal 하위 요소

| 하위 요소 이름 | 설명                                                                                          |
| -------------- | --------------------------------------------------------------------------------------------- |
| mainText       | 덧말 기능의 본 내용<br/>요소 값으로 내용 문자열을 가짐<br/>해당 요소의 추가적인 설명은 생략   |
| subText        | 덧말 기능의 덧말 내용<br/>요소 값으로 내용 문자열을 가짐<br/>해당 요소의 추가적인 설명은 생략 |

### 샘플 166 — dutmal 예

```xml
<hp:dutmal posType="TOP" szRatio="0" option="0" styleIDRef="0" align="CENTER">
  <hp:mainText>테스트 문서</hp:mainText>
  <hp:subText>테스트</hp:subText>
</hp:dutmal>
```

### 10.12.4 비디오

### 표 297 — video 요소

| 속성 이름  | 설명                                                          |
| ---------- | ------------------------------------------------------------- |
| videotype  | 비디오 종류<br/>Local: 컴퓨터의 동영상<br/>Web: 인터넷 동영상 |
| fileIDRef  | 로컬비디오 바이너리 데이터에 대한 아이디 참조값               |
| imageIDRef | 비디오 홈백의 이미지에 대한 아이디 참조값                     |

### 샘플 167 — video 예

```xml
<hp:video id="1476326878" zOrder="0" numberingType="PICTURE" textWrap="SQUARE"
  textFlow="BOTH_SIDES" lock="0" dropcapstyle="None" href="" groupLevel="0" instid="402585055"
  videotype="Local" fileIDRef="video1" imageIDRef="image2" tag="">
  <hp:sz width="22500" widthRelTo="ABSOLUTE" height="15000" heightRelTo="ABSOLUTE"
    protect="0"/>
  <hp:pos treatAsChar="0" affectLSpacing="0" flowWithText="1" allowOverlap="0"
    holdAnchorAndSO="0" vertRelTo="PARA" horzRelTo="COLUMN" vertAlign="TOP" horzAlign="LEFT"
    vertOffset="0" horzOffset="0"/>
  <hp:outMargin left="0" right="0" top="0" bottom="0"/>
  <hp:shapeComment>동영상입니다.</hp:shapeComment>
  ......
</hp:video>
```
