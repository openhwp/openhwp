pub mod bin_data;
pub mod border_fill;
pub mod bullet;
pub mod char_shape;
pub mod distribute_doc_data;
pub mod doc_data;
pub mod face_name;
pub mod forbidden_char;
pub mod id_mapping_count;
pub mod numbering;
pub mod paragraph_shape;
pub mod style;
pub mod tab_definition;

use super::{DocInfoError, RecordIter};
use crate::Version;

pub use bin_data::*;
pub use border_fill::*;
pub use bullet::*;
pub use char_shape::*;
pub use distribute_doc_data::*;
pub use doc_data::*;
pub use face_name::*;
pub use forbidden_char::*;
pub use id_mapping_count::*;
pub use numbering::*;
pub use paragraph_shape::*;
pub use style::*;
pub use tab_definition::*;

#[derive(Debug)]
pub struct IdMappings {
    pub id_mapping_count: IdMappingCount,
    pub bin_data: Vec<BinData>,
    pub face_names: Vec<FaceName>,
    pub border_fills: Vec<BorderFill>,
    pub char_shapes: Vec<CharShape>,
    pub tab_definitions: Vec<TabDefinition>,
    pub numberings: Vec<Numbering>,
    pub bullets: Vec<Bullet>,
    pub paragraph_shapes: Vec<ParagraphShape>,
    pub styles: Vec<Style>,
    pub doc_data: Vec<DocData>,
    pub distribute_doc_data: Vec<DistributeDocData>,
    pub forbidden_chars: Vec<ForbiddenChar>,
}

impl<'doc_info> RecordIter<'doc_info> {
    pub fn id_mappings(&mut self, version: &Version) -> Result<IdMappings, DocInfoError> {
        let id_mapping_count = self.id_mapping_count()?;

        let bin_data = self.bin_data(&id_mapping_count);
        let face_names = self.face_names(&id_mapping_count);
        let border_fills = self.border_fills(&id_mapping_count);
        let char_shapes = self.char_shapes(&id_mapping_count, version);
        let tab_definitions = self.tab_definitions(&id_mapping_count);
        let numberings = self.numberings(&id_mapping_count, version);
        let bullets = self.bullets(&id_mapping_count);
        let paragraph_shapes = self.paragraph_shapes(&id_mapping_count, version);
        let styles = self.styles(&id_mapping_count);
        let doc_data = self.doc_data();
        let distribute_doc_data = self.distribute_doc_data();
        let forbidden_chars = self.forbidden_chars();

        Ok(IdMappings {
            id_mapping_count,
            bin_data,
            face_names,
            border_fills,
            char_shapes,
            tab_definitions,
            numberings,
            bullets,
            paragraph_shapes,
            styles,
            doc_data,
            distribute_doc_data,
            forbidden_chars,
        })
    }
}
