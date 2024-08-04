use super::{BorderShape, IdMappingCount};
use crate::{u32, DocInfoTag, RecordIter};

#[derive(Debug)]
pub struct TabDefinition {
    pub left_tab: bool,
    pub right_tab: bool,
    pub tabs: Vec<TabInformation>,
}

#[derive(Debug)]
pub struct TabInformation {
    pub position: u32,
    pub kind: TabKind,
    pub fill_kind: BorderShape,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TabKind {
    Left,
    Right,
    Center,
    Decimal,
    Unknown(u8),
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn tab_definitions(&mut self, id_mappings: &IdMappingCount) -> Vec<TabDefinition> {
        let mut tab_defs = Vec::with_capacity(id_mappings.tab_def as usize);

        for record in self
            .take(id_mappings.tab_def as usize)
            .take_while(|record| record.tag_id == DocInfoTag::HWPTAG_TAB_DEF as u16)
        {
            tab_defs.push(TabDefinition::from_buf(record.payload));
        }

        tab_defs
    }
}

impl TabDefinition {
    pub fn from_buf(buf: &[u8]) -> TabDefinition {
        let attribute = u32(buf, 0);
        let count = u32(buf, 4);
        let (_, mut buf) = buf.split_at(8);
        let mut tabs = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (tab, rest) = buf.split_at(8);
            buf = rest;

            let position = u32(tab, 0);
            let kind = match tab[4] {
                0 => TabKind::Left,
                1 => TabKind::Right,
                2 => TabKind::Center,
                3 => TabKind::Decimal,
                kind => TabKind::Unknown(kind),
            };
            let fill_kind = BorderShape::from_buf(&[tab[5]]);

            tabs.push(TabInformation {
                position,
                kind,
                fill_kind,
            });
        }

        TabDefinition {
            left_tab: attribute & 0x01 != 0,
            right_tab: attribute & 0x02 != 0,
            tabs,
        }
    }
}
