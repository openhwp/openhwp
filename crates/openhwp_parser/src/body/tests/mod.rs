use super::super::section::{MemoListHeader, SectionMemoShape};
use super::super::{Body, MemoRecord, MemoStore, Section};
use crate::doc_info::memo_shape::MemoShape as DocMemoShape;
use crate::{HwpDocumentError, hash_code, pseudo, u32};

#[test]
fn officially_distributed_hwp_5_0_format() {
    static PAYLOAD: [u8; 256] = [
        190, 100, 247, 91, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 159, 159, 168, 159, 219,
        159, 168, 159, 170, 159, 217, 236, 213, 236, 169, 236, 218, 236, 175, 236, 213, 236, 221,
        236, 222, 236, 157, 175, 195, 134, 194, 134, 199, 134, 195, 134, 194, 76, 122, 76, 117, 76,
        123, 76, 146, 165, 227, 165, 231, 165, 156, 165, 150, 165, 149, 165, 227, 165, 203, 249,
        204, 249, 207, 249, 193, 249, 201, 249, 193, 22, 83, 22, 46, 22, 37, 19, 38, 19, 36, 19,
        82, 19, 16, 147, 19, 19, 19, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47,
        190, 190, 190, 190, 190, 190, 190, 190, 98, 98, 31, 31, 31, 31, 18, 18, 18, 18, 18, 18, 18,
        18, 18, 204, 204, 204, 204, 204, 204, 204, 204, 204, 204, 204, 204, 204, 164, 164, 164,
        164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 218, 218, 218, 218, 218, 218, 20, 20, 20, 20,
        20, 20, 20, 20, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 170, 170, 170, 170,
        170, 170, 170, 152, 152, 152, 152, 44, 44, 44, 44, 44, 44, 44, 204, 204, 204, 204, 204,
        204, 204, 204, 204, 245, 245, 245, 245, 245, 245, 245, 245, 245, 245, 245, 245, 245, 245,
        245, 245, 97,
    ];
    let seed = u32(&PAYLOAD, 0);
    let pseudo = pseudo(seed);
    let hash_code = hash_code(seed, pseudo, PAYLOAD);
    insta::assert_debug_snapshot!((seed, pseudo, hash_code));
}

#[test]
fn validate_sections_allows_matching_lengths() {
    let sections = vec![Section {
        paragraphs: vec![],
        memos: vec![],
    }];
    let result = Body::validate_sections(sections, Some(1), "BodyText");

    assert!(result.is_ok());
}

#[test]
fn validate_sections_rejects_mismatch() {
    let sections = vec![
        Section {
            paragraphs: vec![],
            memos: vec![],
        },
        Section {
            paragraphs: vec![],
            memos: vec![],
        },
    ];
    let error =
        Body::validate_sections(sections, Some(1), "ViewText").expect_err("should detect mismatch");

    match error {
        HwpDocumentError::SectionCountMismatch {
            stream,
            expected,
            actual,
        } => {
            assert_eq!(stream, "ViewText");
            assert_eq!(expected, 1);
            assert_eq!(actual, 2);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn memo_store_collects_memos() {
    let doc_shapes = vec![DocMemoShape { raw: vec![1] }, DocMemoShape { raw: vec![2] }];
    let sections = vec![Section {
        paragraphs: vec![],
        memos: vec![
            MemoRecord::Shape(SectionMemoShape { raw: vec![10] }),
            MemoRecord::List(MemoListHeader { property: 0x10001 }),
        ],
    }];
    let store = MemoStore::from_sections(&doc_shapes, &sections, &[]);

    assert_eq!(store.doc_shapes.len(), 2);
    assert_eq!(store.section_shapes.len(), 1);
    assert_eq!(store.lists.len(), 1);
}
