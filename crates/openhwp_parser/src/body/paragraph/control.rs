use crate::{BodyIter, HwpDocumentError, HwpTag};

#[derive(Debug)]
pub enum Control {
    //
}

impl<'hwp> BodyIter<'hwp> {
    pub fn controls(&mut self, count: usize) -> Result<Vec<Control>, HwpDocumentError> {
        let mut controls = Vec::with_capacity(count as usize);

        for _ in 0..count {
            controls.push(self.control()?);
        }

        Ok(controls)
    }

    pub fn control(&mut self) -> Result<Control, HwpDocumentError> {
        let record = self.expect(HwpTag::HWPTAG_CTRL_HEADER)?;
        let control = Control::from_buf(record.payload);

        Ok(control)
    }
}

impl Control {
    pub const fn from_buf(buf: &[u8]) -> Self {
        let (id, buf) = buf.split_at(4);
        match &[id[3], id[2], id[1], id[0]] {
            b"tbl " => std::todo!(),
            b"$lin" => std::todo!(),
            b"$rec" => std::todo!(),
            b"$ell" => std::todo!(),
            b"$arc" => std::todo!(),
            b"$pol" => std::todo!(),
            b"$cur" => std::todo!(),
            b"eqed" => std::todo!(),
            b"$pic" => std::todo!(),
            b"$ole" => std::todo!(),
            b"$con" => std::todo!(),

            // @hahnlee 님의 hwp-rs 레포지토리에서 참고함.
            // <https://github.com/hahnlee/hwp-rs/blob/e41e7b427e131bea298c708d8a1c5e71dbf625b7/crates/hwp/src/hwp/paragraph/control/mod.rs#L177>
            b"tcps" => std::todo!(),

            b"%unk" => std::todo!(),
            b"%sig" => std::todo!(),
            b"%%*d" => std::todo!(),
            b"%%*a" => std::todo!(),
            b"%%*C" => std::todo!(),
            b"%%*S" => std::todo!(),
            b"%%*T" => std::todo!(),
            b"%%*P" => std::todo!(),
            b"%%*L" => std::todo!(),
            b"%%*c" => std::todo!(),
            b"%%*h" => std::todo!(),
            b"%%*A" => std::todo!(),
            b"%%*i" => std::todo!(),
            b"%%*t" => std::todo!(),
            b"%%*r" => std::todo!(),
            b"%%*l" => std::todo!(),
            b"%%*n" => std::todo!(),
            b"%%*e" => std::todo!(),
            b"%spl" => std::todo!(),
            b"%%mr" => std::todo!(),
            b"%%me" => std::todo!(),
            b"%cpr" => std::todo!(),
            b"%toc" => std::todo!(),

            // 이 아래부터는 HWP 파일 형식 5.0 문서에 없음.
            // <https://www.hancom.com/board/devmanualList.do?artcl_seq=3909>
            b"cold" => std::todo!(),
            b"secd" => std::todo!(),
            b"fn  " => std::todo!(),
            b"en  " => std::todo!(),
            b"gso " => std::todo!(),
            b"atno" => std::todo!(),
            b"nwno" => std::todo!(),
            b"pgct" => std::todo!(),
            b"pghd" => std::todo!(),
            b"pgnp" => std::todo!(),
            b"head" => std::todo!(),
            b"foot" => std::todo!(),
            b"%dte" => std::todo!(),
            b"%ddt" => std::todo!(),
            b"%pat" => std::todo!(),
            b"%bmk" => std::todo!(),
            b"%mmg" => std::todo!(),
            b"%xrf" => std::todo!(),
            b"%fmu" => std::todo!(),
            b"%clk" => std::todo!(),
            b"%smr" => std::todo!(),
            b"%usr" => std::todo!(),
            b"%hlk" => std::todo!(),
            b"bokm" => std::todo!(),
            b"idxm" => std::todo!(),
            b"tdut" => std::todo!(),
            b"tcmt" => std::todo!(),
            _ => std::todo!(),
        }
    }
}
