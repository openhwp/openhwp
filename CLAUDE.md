# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Test specific crate
cargo test -p hwp
cargo test -p hwpx

# Check code without building
cargo check

# Generate documentation
cargo doc --open
```

## Project Overview

OpenHWP is a Rust library for reading and writing HWP (한글 워드프로세서) documents. It supports two Korean document formats:

- **HWP 5.0** (`.hwp`): Binary format using OLE/CFB containers (한글 2002~2022)
- **HWPX** (`.hwpx`): XML-based format following KS X 6101:2024 standard

## Architecture

### Workspace Structure

```
crates/
├── hwp/   - HWP 5.0 binary parser (cfb + miniz_oxide)
└── hwpx/  - HWPX XML parser (serde + quick-xml)
```

### hwp Crate

Entry point: `HwpDocument::from_bytes()` / `HwpDocument::from_bytes_with_password()`

Key modules:
- `document.rs` - Main `HwpDocument` type that orchestrates CFB parsing
- `header/` - File header parsing (version, encryption flags)
- `doc_info/` - Document metadata (fonts, styles, paragraph shapes)
- `body/` - Content parsing (sections, paragraphs, controls like tables/pictures)
- `crypto/` - Password encryption and distribution document decryption
- `util/` - Stream decompression (deflate) and binary reading helpers

HWP 5.0 files are OLE compound files containing streams:
- `/FileHeader` - Document identification and properties
- `/DocInfo` - Document-level info (fonts, styles, numbering)
- `/BodyText/Section{N}` - Body content per section
- `/BinData/` - Binary data (images, OLE objects)
- `/PrvText`, `/PrvImage` - Preview content

### hwpx Crate

Entry points: `hwpx::from_str()` for deserialization, `hwpx::to_string()` for serialization

Key modules:
- `core/` - Shared attributes, enums, types
- `header/` - Document header types (fonts, styles, character/paragraph shapes)
- `paragraph/` - Body content (sections, paragraphs, tables, pictures, controls)
- `master_page.rs` - Master page definitions
- `history.rs` - Change history
- `version.rs` - Version information

HWPX file-to-type mapping:
- `version.xml` → `hwpx::version::HcfVersion`
- `Contents/header.xml` → `hwpx::header::Head`
- `Contents/section*.xml` → `hwpx::paragraph::Section`

## Specification Documents

The `docs/` directory contains Korean standard specifications:
- `docs/hwp/` - HWP 5.0 binary format specs
- `docs/hwpx/` - KS X 6101:2024 (HWPX) XML schema documentation
