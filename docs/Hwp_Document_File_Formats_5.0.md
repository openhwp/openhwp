> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

본 문서는 HWP 5.0 형식과 관련한 작업을 진행할 이후의 다른 분에게 도움이 주고자 하는 의도로 한컴의 [HWP 5.0 형식 PDF](https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_5.0_revision1.3.pdf)를 Markdown 형식으로 옮겼으며, 아래와 같은 이유 등으로 원본 PDF와 다른 부분을 다소 포함하고 있습니다.

- PDF 형식 대비 Markdown 형식의 표현 한계
- 실제 HWP 파일이 원본 PDF 서술 내용과 다른 부분
- 원본 PDF에서 누락된 내용
- 서술 내용을 이해하기 위해 필요한 메타지식

# 한/글 문서 파일 구조 5.0

## 저작권

(주)한글과컴퓨터(이하 '한컴')는 문서 형식의 개방성과 표준화에 대하여 적극 찬성합니다. 한컴은 한/글 97의 문서 형식을 무상으로 지원한 바 있으며, 한/글 2002~2010 문서의 XML 형식은 HWPML에 대해서 도 문서 형식을 공개한 바 있습니다. 개방형 문서 표준화 및 코드 관련 위원회에도 적극적으로 참여하여 파일 형식의 표준화와 개방성을 위해 노력해 왔습니다. 이러한 결과로 HWPML 스펙이 OWPML란 이름으로 한국산업표준(KS X 6101:2011)으로 제정되었습니다. 또한, 한컴오피스에서 기록물 장기보존 표준 포맷인 PDF/A-1의 지원과 ISO 국제 문서 형식인 ODF와 OOXML 파일 형식의 불러오기와 저장하 기를 적극적으로 지원하였습니다.

본 문서를 열람하고자 하는 자라면 누구에게나 제공되는 것이며, 본 문서를 열람하는 것 외에 복사, 배포, 게재 및 본 문서에 기재되어 있는 내용을 사용하고자 하는 자는 한글과컴퓨터의 본 저작권을 충분히 인식하고 동의하여야 합니다.

본 문서를 누구나 열람, 복사, 배포, 게재 및 사용을 자유롭게 할 수 있습니다. 다만, 배포는 원 내용이 일체 수정되지 않은 원본 또는 복사본으로 제한됩니다. 원본 및 복사본은 한컴에서 제공하는 스펙의 최신 버전을 포함하고 있어야 합니다.

한컴은 한컴오피스 한/글 문서 파일(.hwp) 공개 문서에 따라 얻은 결과물을 기초로 또 다른 독점적, 배타적 권리를 취득하고 이를 (주)한글과컴퓨터를 상대로 행사하고자 하는 자를 상대로는 적극적으로 권리행사를 할 수도 있습니다.

그리고, 본 문서 및 본 문서에 기재된 내용을 참고하여 개발한 결과물에 대한 모든 저작권은 결과물을 개발한 개인 또는 단체에 있을 것입니다. 그러나 반드시 개발 결과물에 "본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다."라고 제품 내 사용자 인터페이스, 매뉴얼, 도움말 및 소스에 모두 기재하여야 하며 제품이 이러한 구성물이 없을 시에는 존재하는 구성물에만 기재합니다. 한컴은 본 문서 및 본 문서에 기재된 내용을 참고하여 개발한 결과물에 대해서 어떠한 정확성, 진실성도 보증하지 아니합니다.

## 본 문서에 대하여...

본 문서는 한/글 워드 프로세서의 파일 저장 형식 중, 한/글 2002 이후 제품에서 사용되는 한/글 문서 파일 형식 5.0 에 관하여 설명한다.
본 문서는 한/글 문서 파일 형식 5.0의 주요한 자료 형식 및 파일 구조, 레코드 구조에 대해서 설명한다.
한/글 문서 수식, 차트, 배포용 문서, 한/글 문서 파일 형식 3.0, HWPML에 관해서는 별도의 문서에서 설명한다.

## I. 한/글 5.0 파일 구조 (Hwp Document File Format 5.0)

### 1. 개요

한/글의 문서 파일은 사용자가 따로 지정하지 않는 한 .HWP를 기본 확장자로 가진다. 문서 파일에 저장되는 내용은, 실제 사용자가 입력한 문서의 내용과 문자 장식 정보뿐만 아니라 문서를 편집할 당시의 글꼴에 대한 정보, 조판에 영향을 주는 설정 사항(용지 종류, 여백 정보 등)도 포함된다.

한/글 문서 파일 형식 5.0은 2000년 10월 이후에 출시된 한/글 제품군(한/글 워디안, 한/글 2002, 한/글 2005, 한/글 2007, 한/글 2010, 한/글 2014, 한/글 2018 등)에서 생성되며, 문서 버전에 따라 큰 골격은 유지되나, 추가적인 정보들에 의해 약간의 차이가 있다.

한/글 문서 파일 형식 5.0은 파일의 크기를 최소화하기 위하여 압축 기능을 이용한다. 압축된 문서 파일도 기본적인 정보를 저장하는 부분은 압축되지 않으며, 실제 압축되는 부분은 사용자가 입력한 본문과 그림 관련 데이터 부분이다.

한/글은 문서 파일의 압축에 zlib.org의 zlib을 사용했다. zlib은 웹상에 소스가 공개되어 있는 공개 소프트웨어이다. zlib은 zlib License를 따르며, 이는 소스의 자유로운 사용이 가능하며 해당 소스를 사용한 2차 산출물에 대한 소스 공개 의무가 없다. 자세한 사항은 zlib에 포함되어 있는 라이센스 문서 파일을 참조하기 바란다.

한/글 문서 파일 형식 5.0의 구조는 윈도우즈의 복합 파일(Compound File)에 기초를 두며, 문자 코드는 ISO-10646 표준을 기반으로 한다. 대부분의 문자 정보는 유니코드(UTF-16LE) 형식으로 전달되고, 저장된다.

> Compound File에 대한 접근 방법은 OLE관련 자료 또는 MSDN을 참고 StgOpenStorage(), IStorage::Open(), ...

### 2. 자료형 설명

앞으로 계속되는 설명에서 한/글의 문서 파일에 저장되는 정보는 아래 표에 설명하는 자료형을 이용해 표현한다.

자료형에서 한 바이트는 8 비트로 표현되며, 두 바이트 이상의 길이를 가지는 자료형은 최하위 바이트가 가장 먼저 저장되고, 최상위 바이트가 가장 나중에 저장되는 리틀 엔디언(Little-endian) 형태이다.

파일에 저장되는 자료가 배열(array)일 때는 '자료형 array[개수]'와 같이 표현한다. 예를 들어 10개의 원소를 갖는 word 배열이면 'word array[10]'과 같이 표현한다

<i id='table-1'></i>

| 자료형                                       |    길이 | 부호 | 설명                                                                                        |
| -------------------------------------------- | ------: | :--: | ------------------------------------------------------------------------------------------- |
| <i id='datatype-BYTE'>BYTE</i>               | 1 Bytes |      | 부호 없는 한 바이트(0~255)                                                                  |
| <i id='datatype-WORD'>WORD</i>               | 2 Bytes |      | 16비트 컴파일러에서 'unsigned int'에 해당                                                   |
| <i id='datatype-DWORD'>DWORD</i>             | 4 Bytes |      | 16비트 컴파일러에서 'unsigned long'에 해당                                                  |
| <i id='datatype-WCHAR'>WCHAR</i>             | 2 Bytes |      | 한/글의 기본 코드로 유니코드 기반 문자                                                      |
| <i id='datatype-HWPUNIT'>HWPUNIT</i>         | 4 Bytes |      | 1/7200인치로 표현된 한/글 내부 단위                                                         |
| <i id='datatype-SHWPUNIT'>SHWPUNIT</i>       | 4 Bytes |  v   | 1/7200인치로 표현된 한/글 내부 단위                                                         |
| <i id='datatype-UINT8'>UINT8</i>             | 1 Bytes |      | 'unsigned \_\_int8' 에 해당                                                                 |
| <i id='datatype-UINT16'>UINT16</i>           | 2 Bytes |      | 'unsigned \_\_int16' 에 해당                                                                |
| <i id='datatype-UINT32'>UINT32</i>(=UINT)    | 4 Bytes |      | 'unsigned \_\_int32' 에 해당                                                                |
| <i id='datatype-INT8'>INT8</i>               | 1 Bytes |  v   | 'signed \_\_int8' 에 해당                                                                   |
| <i id='datatype-INT16'>INT16</i>             | 2 Bytes |  v   | 'signed \_\_int16' 에 해당                                                                  |
| <i id='datatype-INT32'>INT32</i>             | 4 Bytes |  v   | 'signed \_\_int32' 에 해당                                                                  |
| <i id='datatype-HWPUNIT16'>HWPUNIT16</i>     | 2 Bytes |  v   | INT16 과 같다.                                                                              |
| <i id='datatype-COLORREF'>COLORREF</i>       | 4 Bytes |      | RGB값(0x00bbggrr)을 십진수로 표시<br>(rr : red 1 byte, gg : green 1 byte, bb : blue 1 byte) |
| <i id='datatype-BYTE-stream'>BYTE stream</i> |         |      | 일련의 BYTE로 구성됨.<br>본문 내에서 다른 구조를 참조할 경우에 사용됨.                      |

<i id='table-1-label'>표 1 자료형</i>

[WCHAR](#datatype-WCHAR)는 한/글의 내부 코드로 표현된 문자 한 글자를 표현하는 자료형이다. 한/글의 내부 코드는 한글, 영문, 한자를 비롯해 모든 문자가 2 바이트의 일정한 길이를 가진다.

[HWPUNIT](#datatype-HWPUNIT)과 [SHWPUNIT](#datatype-SHWPUNIT)는 문자의 크기, 그림의 크기, 용지 여백 등, 문서를 구성하는 요소들의 크기를 표현하기 위한 자료형이다. 문서 출력 장치의 해상도는 가변적이기 때문에, 크기 정보를 점(도트)의 수로 표현할 수는 없다. 따라서 일정한 단위를 기준으로 해야 하는데, 한/글에서는 1/7200인치를 기본 단위로 사용한다. 예를 들어 [가로 2 인치 x 세로 1 인치]짜리 그림의 크기를 [HWPUNIT](#datatype-HWPUNIT) 형으로 표현하면 각각 14400 x 7200이 된다.

### 3. 한/글 파일 구조

#### 3.1. 한/글 파일 구조 요약

한/글의 문서 파일은 개괄적으로 다음 표와 같은 구조를 가진다. 복합 파일(Compound File) 구조를 가지기 때문에, 내부적으로 스토리지(Storage)와 스트림(Stream)을 구별하기 위한 이름을 가진다.

하나의 스트림에는 일반적인 바이너리나 레코드 구조로 데이터가 저장되고, 스트림에 따라서 압축/암호화되기도 한다.

<i id='table-2'></i>

📁 Storage 📖 Stream

| 설명            | 구별 이름                                                            | 길이 | 레코드 구조 | 압축/암호화 |
| --------------- | -------------------------------------------------------------------- | :--: | :---------: | :---------: |
| 파일 인식 정보  | 📖 FileHeader                                                        | 고정 |             |             |
| 문서 정보       | 📖 DocInfo                                                           | 고정 |      v      |      v      |
| 본문            | 📁 BodyText<br>↳ 📖 Section0<br>↳ 📖 Section1<br>↳ 📖 ...            | 가변 |      v      |      v      |
| 문서 요약       | 📖 \005HwpSummaryInformation                                         | 고정 |             |             |
| 바이너리 데이터 | 📁 BinData<br>↳ 📖 BinaryData0<br>↳ 📖 BinaryData1<br>↳ 📖 ...       | 가변 |             |      v      |
| 미리보기 텍스트 | 📖 PrvText                                                           | 고정 |             |             |
| 미리보기 이미지 | 📖 PrvImage                                                          | 가변 |             |             |
| 문서 옵션       | 📁 DocOptions<br>↳ 📖 \_LinkDoc<br>↳ 📖 DrmLicense<br>↳ 📖 ...       | 가변 |             |             |
| 스크립트        | 📁 Scripts<br>↳ 📖 DefaultJScript<br>↳ 📖 JScriptVersion<br>↳ 📖 ... | 가변 |             |             |
| XML 템플릿      | 📁 XMLTemplate<br>↳ 📖 Schema<br>↳ 📖 Instance<br>↳ 📖 ...           | 가변 |             |             |
| 문서 이력 관리  | 📁 DocHistory<br>↳ 📖 VersionLog0<br>↳ 📖 VersionLog1<br>↳ 📖 ...    | 가변 |      v      |      v      |

<i id='table-2-label'>표 2 전체 구조</i>

압축된 문서 파일의 경우 문서 파일을 읽는 쪽에서는 '파일 인식 정보' 항목의 '압축' 플래그를 살펴보고, 압축된 파일이면 압축을 풀어서 처리해야 한다. 이후의 설명에서는 압축이 풀린 상태의 파일을 기준으로 한다. '문서정보'와 '본문' '문서 이력 관리'에 사용되는 '레코드 구조'는 이후 '데이터 레코드'란에서 구조 설명과 사용되는 레코드들에 대한 상세한 설명을 한다.

#### 3.2. 스토리지 별 저장 정보

##### 3.2.1. 파일 인식 정보

한/글의 문서 파일이라는 것을 나타내기 위해 '파일 인식 정보'가 저장된다.

FileHeader 스트림에 저장되는 데이터는 다음과 같다.

<i id='table-3'></i>

<table>
  <thead>
    <tr>
      <th>자료형</th>
      <th>길이</th>
      <th colspan="2">설명</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>BYTE array[32]</td>
      <td>32 Bytes</td>
      <td colspan="2">signature. 문서 파일은 "HWP Document File"</td>
    </tr>
    <tr>
      <td>DWORD</td>
      <td>4 Bytes</td>
      <td colspan="2">
        파일 버전. 0xMMnnPPrr의 형태(예 5.0.3.0)
        <br>- MM: 문서 형식의 구조가 완전히 바뀌는 것을 나타냄. 숫자가 다르면 구 버전과 호환 불가능.
        <br>- nn: 큰 구조는 동일하나, 큰 변화가 있는 것을 나타냄. 숫자가 다르면 구 버전과 호환 불가능.
        <br>- PP: 구조는 동일, Record가 추가되었거나, 하위 버전에서 호환되지 않는 정보가 추가된 것을 나타냄. 숫자가 달라도 구 버전과 호환 가능.
        <br>- rr: Record에 정보들이 추가된 것을 나타냄. 숫자가 달라도 구 버전과 호환 가능.
      </td>
    </tr>
    <tr>
      <td rowspan="21">DWORD</td>
      <td rowspan="21">4 Bytes</td>
      <td colspan="2">
        속성
      </td>
    </tr>
    <tr>
      <th>범위</th>
      <th>설명</th>
    </tr>
    <tr>
      <td>bit 0</td>
      <td>압축 여부</td>
    </tr>
    <tr>
      <td>bit 1</td>
      <td>암호 설정 여부</td>
    </tr>
    <tr>
      <td>bit 2</td>
      <td>배포용 문서 여부</td>
    </tr>
    <tr>
      <td>bit 3</td>
      <td>스크립트 저장 여부</td>
    </tr>
    <tr>
      <td>bit 4</td>
      <td>DRM 보안 문서 여부</td>
    </tr>
    <tr>
      <td>bit 5</td>
      <td>XMLTemplate 스토리지 존재 여부</td>
    </tr>
    <tr>
      <td>bit 6</td>
      <td>문서 이력 관리 존재 여부</td>
    </tr>
    <tr>
      <td>bit 7</td>
      <td>전자 서명 정보 존재 여부</td>
    </tr>
    <tr>
      <td>bit 8</td>
      <td>공인 인증서 암호화 여부</td>
    </tr>
    <tr>
      <td>bit 9</td>
      <td>전자 서명 예비 저장 여부</td>
    </tr>
    <tr>
      <td>bit 10</td>
      <td>공인 인증서 DRM 보안 문서 여부</td>
    </tr>
    <tr>
      <td>bit 11</td>
      <td>CCL 문서 여부</td>
    </tr>
    <tr>
      <td>bit 12</td>
      <td>모바일 최적화 여부</td>
    </tr>
    <tr>
      <td>bit 13</td>
      <td>개인 정보 보안 문서 여부</td>
    </tr>
    <tr>
      <td>bit 14</td>
      <td>변경 추적 문서 여부</td>
    </tr>
    <tr>
      <td>bit 15</td>
      <td>공공누리(KOGL) 저작권 문서</td>
    </tr>
    <tr>
      <td>bit 16</td>
      <td>비디오 컨트롤 포함 여부</td>
    </tr>
    <tr>
      <td>bit 17</td>
      <td>차례 필드 컨트롤 포함 여부</td>
    </tr>
    <tr>
      <td>bit 18 ~ 31</td>
      <td>예약</td>
    </tr>
    <tr>
      <td rowspan="6">DWORD</td>
      <td rowspan="6">4 Bytes</td>
      <td colspan="2">
        속성
      </td>
    </tr>
    <tr>
      <th>범위</th>
      <th>설명</th>
    </tr>
    <tr>
      <td>bit 0</td>
      <td>CCL, 공공누리 라이선스 정보</td>
    </tr>
    <tr>
      <td>bit 1</td>
      <td>복제 제한 여부</td>
    </tr>
    <tr>
      <td>bit 2</td>
      <td>동일 조건 하에 복제 허가 여부 (복제 제한인 경우 무시)</td>
    </tr>
    <tr>
      <td>bit 3~31</td>
      <td>예약</td>
    </tr>
    <tr>
      <td>DWORD</td>
      <td>4 Bytes</td>
      <td colspan="2">
        EncryptVersion
        <br>- 0 : None
        <br>- 1 : (한/글 2.5 버전 이하)
        <br>- 2 : (한/글 3.0 버전 Enhanced)
        <br>- 3 : (한/글 3.0 버전 Old)
        <br>- 4 : (한/글 7.0 버전 이후)
      </td>
    </tr>
    <tr>
      <td>BYTE</td>
      <td>1 Bytes</td>
      <td colspan="2">
        공공누리(KOGL) 라이선스 지원 국가
        <br>- 6 : KOR
        <br>- 15 : US
      </td>
    </tr>
    <tr>
      <td>BYTE array[207]</td>
      <td>207 Bytes</td>
      <td colspan="2">예약</td>
    </tr>
  </tbody>
</table>

<i id='table-3-label'>표 3 파일 인식 정보</i>

##### 3.2.2. 문서 정보

본문에 사용 중인 글꼴, 글자 속성, 문단 속성, 탭, 스타일 등에 문서 내 공통으로 사용되는 세부 정보를 담고 있다.

DocInfo 스트림에 저장되는 데이터는 다음과 같다.

<i id='table-4'></i>

| Tag ID                      |       길이 | 레벨 | 설명                           |
| --------------------------- | ---------: | ---: | ------------------------------ |
| HWPTAG_DOCUMENT_PROPERTIES  |   30 Bytes |    0 | 문서 속성(표 14 참조)          |
| HWPTAG_ID_MAPPINGS          |   32 Bytes |    0 | 아이디 매핑 헤더(표 15 참조)   |
| HWPTAG_BIN_DATA             |       가변 |    1 | 바이너리 데이터(표 17 참조)    |
| HWPTAG_FACE_NAME            |       가변 |    1 | 글꼴(표 19 참조)               |
| HWPTAG_BORDER_FILL          |       가변 |    1 | 테두리/배경(표 23 참조)        |
| HWPTAG_CHAR_SHAPE           |   72 Bytes |    1 | 글자 모양(표 33 참조)          |
| HWPTAG_TAB_DEF              |   14 Bytes |    1 | 탭 정의(표 36 참조)            |
| HWPTAG_NUMBERING            |       가변 |    1 | 문단 번호(표 38 참조)          |
| HWPTAG_BULLET               |   10 Bytes |    1 | 글머리표(표 42 참조)           |
| HWPTAG_PARA_SHAPE           |   54 Bytes |    1 | 문단 모양(표 43 참조)          |
| HWPTAG_STYLE                |       가변 |    1 | 스타일(표 47 참조)             |
| HWPTAG_MEMO_SHAPE           |   22 Bytes |    1 | 메모 모양                      |
| HWPTAG_TRACK_CHANGE_AUTHOR  |       가변 |    1 | 변경 추적 작성자               |
| HWPTAG_TRACK_CHANGE         |       가변 |    1 | 변경 추적 내용 및 모양         |
| HWPTAG_DOC_DATA             |       가변 |    0 | 문서 임의의 데이터(표 49 참조) |
| HWPTAG_FORBIDDEN_CHAR       |       가변 |    0 | 금칙처리 문자                  |
| HWPTAG_COMPATIBLE_DOCUMENT  |    4 Bytes |    0 | 호환 문서(표 54 참조)          |
| HWPTAG_LAYOUT_COMPATIBILITY |   20 Bytes |    1 | 레이아웃 호환성(표 56 참조)    |
| HWPTAG_DISTRIBUTE_DOC_DATA  |  256 Bytes |    0 | 배포용 문서                    |
| HWPTAG_TRACKCHANGE          | 1032 Bytes |    1 | 변경 추적 정보                 |
| 전체 길이                   |       가변 |      |                                |

<i id='table-4-label'>표 4 문서 정보</i>

각각의 세부 정보는 <'문서 정보'의 데이터 레코드>란에서 추가로 다룬다.

#### 3.2.3. 본문

문서의 본문에 해당되는 문단, 표, 그리기 개체 등의 내용이 저장된다.

BodyText 스토리지는 본문의 구역에 따라 `Section%d` 스트림(%d는 구역의 번호)으로 구분된다. 구역 의 개수는 문서 정보의 문서 속성에 저장된다.

각 구역의 첫 문단에는 구역 정의 레코드가 저장되고, 각 단 설정의 첫 문단에는 단 정의 레코드가 저장된다.

각 구역의 가장 끝 위치에는 확장 바탕쪽(마지막 쪽, 임의 쪽) 관련 정보가 저장되고, 마지막 구역의 가장 끝 위치에는 메모 관련 정보가 저장된다.

Section 스트림에 저장되는 데이터는 문단들(문단 리스트)이며, 다음과 같은 문단 정보들이 반복 된다.

<i id='table-5'></i>

| Tag ID                           |     길이 | 레벨 | 설명                         |
| -------------------------------- | -------: | ---: | ---------------------------- |
| HWPTAG_PARA_HEADER               | 22 Bytes |    0 | 문단 헤더(표 58 참조)        |
| HWPTAG_PARA_TEXT                 |     가변 |    1 | 문단의 텍스트(표 60 참조)    |
| HWPTAG_PARA_CHAR_SHAPE           |     가변 |    1 | 문단의 글자 모양(표 61 참조) |
| HWPTAG_PARA_LINE_SEG             |     가변 |    1 | 문단의 레이아웃              |
| HWPTAG_PARA_RANGE_TAG            |     가변 |    1 | 문단의 영역 태그(표 63 참조) |
| HWPTAG_CTRL_HEADER               |  4 Bytes |    1 | 컨트롤 헤더(표 64 참조)      |
| HWPTAG_LIST_HEADER               |  6 Bytes |    2 | 문단 리스트 헤더(표 65 참조) |
| HWPTAG_PAGE_DEF                  | 40 Bytes |    2 | 용지 설정                    |
| HWPTAG_FOOTNOTE_SHAPE            | 30 Bytes |    2 | 각주/미주 모양               |
| HWPTAG_PAGE_BORDER_FILL          | 14 Bytes |    2 | 쪽 테두리/배경               |
| HWPTAG_SHAPE_COMPONENT           |  4 Bytes |    2 | 개체                         |
| HWPTAG_TABLE                     |     가변 |    2 | 표 개체                      |
| HWPTAG_SHAPE_COMPONENT_LINE      | 20 Bytes |    3 | 직선 개체                    |
| HWPTAG_SHAPE_COMPONENT_RECTANGLE |  9 Bytes |    3 | 사각형 개체                  |
| HWPTAG_SHAPE_COMPONENT_ELLIPSE   | 60 Bytes |    3 | 타원 개체                    |
| HWPTAG_SHAPE_COMPONENT_ARC       | 25 Bytes |    3 | 호 개체                      |
| HWPTAG_SHAPE_COMPONENT_POLYGON   |     가변 |    3 | 다각형 개체                  |
| HWPTAG_SHAPE_COMPONENT_CURVE     |     가변 |    3 | 곡선 개체                    |
| HWPTAG_SHAPE_COMPONENT_OLE       | 26 Bytes |    3 | OLE 개체                     |
| HWPTAG_SHAPE_COMPONENT_PICTURE   |     가변 |    3 | 그림 개체                    |
| HWPTAG_CTRL_DATA                 |     가변 |    2 | 컨트롤 임의의 데이터         |
| HWPTAG_EQEDIT                    |     가변 |    2 | 수식 개체                    |
| HWPTAG_SHAPE_COMPONENT_TEXTART   |     가변 |    3 | 글맵시                       |
| HWPTAG_FORM_OBJECT               |     가변 |    2 | 양식 개체                    |
| HWPTAG_MEMO_SHAPE                | 22 Bytes |    1 | 메모 모양                    |
| HWPTAG_MEMO_LIST                 |  4 Bytes |    1 | 메모 리스트 헤더             |
| HWPTAG_CHART_DATA                |  2 Bytes |    2 | 차트 데이터                  |
| HWPTAG_VIDEO_DATA                |     가변 |    3 | 비디오 데이터                |
| HWPTAG_SHAPE_COMPONENT_UNKNOWN   | 36 Bytes |    3 | Unknown                      |
| 전체 길이                        |     가변 |      |                              |

<i id='table-5-label'>표 5 본문</i>

문단에 컨트롤이 포함되는 경우 컨트롤 헤더 이후로 문단 리스트 헤더와 같은 컨트롤의 레코드 데이터가 저장된다.

##### 제어 문자 (컨트롤)

표, 그림 등 일반 문자로 표현할 수 없는 요소를 표현하기 위해서 문자 코드 중 일부분을 특수 용도로 사용하고 있다.

문단 내용 중에 문자 코드가 0-31인 문자들은 특수 용도로 사용된다. 이미 13번 문자는 문단 내용의 끝 식별 기호로 사용된다는 것은 설명한 바 있다. 이외의 특수 문자들은 표나 그림 등, 일반 문자로 표현할 수 없는 문서 장식 요소를 표현하기 위해서 제어문자(컨트롤)로 사용된다.

제어 문자는 다음 세 가지 형식이 존재한다.

- 문자 컨트롤 [char] = 하나의 문자로 취급되는 문자 컨트롤 / size = 1
- 인라인 컨트롤 [inline] = 별도의 오브젝트 포인터를 가리키지 않는 단순한 인라인 컨트롤 / size = 8
- 확장 컨트롤 [extended] = 별도의 오브젝트가 데이터를 표현하는 확장 컨트롤 / size = 8

<i id='table-6'></i>

|  코드 | 컨트롤 형식 | 설명                                                                                                                                                                         |
| ----: | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     0 | char        | unusable                                                                                                                                                                     |
|     1 | extended    | 예약                                                                                                                                                                         |
|     2 | extended    | 구역 정의/단 정의                                                                                                                                                            |
|     3 | extended    | 필드 시작<br>(누름틀, 하이퍼링크, 블록 책갈피, 표 계산식, 문서 요약, 사용자 정보, 현재 날짜/시간, 문서 날짜/시간, 파일 경로, 상호 참조, 메일 머지, 메모, 교정부호, 개인정보) |
|     4 | inline      | 필드 끝                                                                                                                                                                      |
|   5-7 | inline      | 예약                                                                                                                                                                         |
|     8 | inline      | title mark                                                                                                                                                                   |
|     9 | inline      | 탭                                                                                                                                                                           |
|    10 | char        | 한 줄 끝(line break)                                                                                                                                                         |
|    11 | extended    | 그리기 개체/표                                                                                                                                                               |
|    12 | extended    | 예약                                                                                                                                                                         |
|    13 | char        | 문단 끝(para break)                                                                                                                                                          |
|    14 | extended    | 예약                                                                                                                                                                         |
|    15 | extended    | 숨은 설명                                                                                                                                                                    |
|    16 | extended    | 머리말/꼬리말                                                                                                                                                                |
|    17 | extended    | 각주/미주                                                                                                                                                                    |
|    18 | extended    | 자동번호(각주, 표 등)                                                                                                                                                        |
| 19-20 | inline      | 예약                                                                                                                                                                         |
|    21 | extended    | 페이지 컨트롤(감추기, 새 번호로 시작 등)                                                                                                                                     |
|    22 | extended    | 책갈피/찾아보기 표식                                                                                                                                                         |
|    23 | extended    | 덧말/글자 겹침                                                                                                                                                               |
|    24 | char        | 하이픈                                                                                                                                                                       |
| 25-29 | char        | 예약                                                                                                                                                                         |
|    30 | char        | 묶음 빈칸                                                                                                                                                                    |
|    31 | char        | 고정폭 빈칸                                                                                                                                                                  |

<i id='table-6-label'>표 6 제어 문자</i>

문서 파일에서 문단 내용을 읽다가 제어 문자를 발견하면, 문서를 읽는 쪽에서는 제어 문자 종류에 따라 읽어 들이거나 건너 뛰어 다음 데이터의 시작 위치까지 파일 포인터를 옮기기 위한 적절한 처리를 수행해야 한다. 제어 문자 가운데는 또 다른 문단 리스트를 포함하는 경우도 있기 때문에, 제어 문자를 일반 문자처럼 처리하면 문서 파일을 정상적으로 읽을 수 없다.

표, 각주 등과 같은 문단 리스트를 포함하는 컨트롤 문자들은 독자적인 문단 리스트를 가진다. 해당 리스트들은 아래와 같은 리스트 헤더 정보를 포함한다. 실제 문단들은 그 다음에 serialize된다.

문단 내에서 컨트롤은 세 가지 형식에 따라 다음과 같은 차이가 있다.

###### 문자 컨트롤

부가정보 없이 문자 하나로 표현되는 제어 문자이다. (3번째 ch)

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | 10  | 11  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 'A' | 'B' | 'C' | ch  | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 13  |

###### 인라인 컨트롤

부가정보가 12바이트(6 WCHAR) 이내에서 표현될 수 있는 제어 문자이다. info에 부가정보를 다 넣지
못하는 경우는 확장 컨트롤로 대체된다.(3~9까지 8개의 ch)

<table>
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>10</th>
      <th>11</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>'A'</td>
      <td>'B'</td>
      <td>ch</td>
      <td colspan="6" style="text-align: center;">info</td>
      <td>ch</td>
      <td>'C'</td>
      <td>13</td>
    </tr>
  </tbody>
</table>

###### 확장 컨트롤

제어 문자는 포인터를 가지고 있고, 포인터가 가리키는 곳에 실제 오브젝트가 존재하는 제어 문자이다.(3~9까지 8개의 ch)

<table>
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>10</th>
      <th>11</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>'A'</td>
      <td>'B'</td>
      <td>ch</td>
      <td colspan="6" style="text-align: center;">pointer(-> Control Object Instance)</td>
      <td>ch</td>
      <td>'C'</td>
      <td>13</td>
    </tr>
  </tbody>
</table>

본 문서에 부가 설명 없이 '컨트롤' 또는 '제어 문자'이라고 하면 바로 이 확장 컨트롤을 지칭하는 것이다.

#### 3.2.4. 문서 요약

`\005HwpSummaryInfomation` 스트림에는 한/글 메뉴의 "파일-문서 정보-문서 요약"에서 입력한 내용이 저장된다.

> Summary Information에 대한 자세한 설명은 MSDN을 참고
> The Summary Information Property Set
> The DocumentSummaryInformation and UserDefined Property Set

<i id='table-7'></i>

| Name                        | Property ID string | Property ID | VT type           |
| --------------------------- | ------------------ | ----------- | ----------------- |
| Title                       | PIDSI_TITLE        | 0x00000002  | VT_LPSTR          |
| Subject                     | PIDSI_SUBJECT      | 0x00000003  | VT_LPSTR          |
| Author                      | PIDSI_AUTHOR       | 0x00000004  | VT_LPSTR          |
| Keywords                    | PIDSI_KEYWORDS     | 0x00000005  | VT_LPSTR          |
| Comments                    | PIDSI_COMMENTS     | 0x00000006  | VT_LPSTR          |
| Last Saved By               | PIDSI_LASTAUTHOR   | 0x00000008  | VT_LPSTR          |
| Revision Number             | PIDSI_REVNUMBER    | 0x00000009  | VT_LPSTR          |
| Last Printed                | PIDSI_LASTPRINTED  | 0x0000000B  | VT_FILETIME (UTC) |
| Create Time/Date( (\*))     | PIDSI_CREATE_DTM   | 0x0000000C  | VT_FILETIME (UTC) |
| Last saved Time/Date( (\*)) | PIDSI_LASTSAVE_DTM | 0x0000000D  | VT_FILETIME (UTC) |
| Number of Pages             | PIDSI_PAGECOUNT    | 0x0000000E  | VT_I4             |
| Date String(User define)    | HWPPIDSI_DATE_STR  | 0x00000014  | VT_LPSTR          |
| Para Count(User define)     | HWPPIDSI_PARACOUNT | 0x00000015  | VT_I4             |

<i id='table-7-label'>표 7 문서 요약</i>
