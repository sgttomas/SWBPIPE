//! Hash-bound deterministic PDF emitter for the full report package
//! (DEC-061 / D-10b Option O-A, completion-plan row E3).
//!
//! This crate consumes the same assembled section model the deterministic
//! HTML renderer emits from (`report_renderer::assemble_report_sections`) and
//! writes a minimal, byte-deterministic PDF: text, tables, and banner boxes
//! only; base-14 standard fonts (Courier / Courier-Bold, WinAnsiEncoding, no
//! font embedding or subsetting); uncompressed content streams; fixed
//! document metadata; no timestamps, no document IDs, no varying producer
//! strings. Identical input yields byte-identical output, so the SHA-256 of
//! the PDF bytes is hash-bound evidence recorded alongside the canonical HTML
//! hash per the existing convention. `assemble_full_report_package` emits
//! both artifacts from one assembled section model and records the per-member
//! hashes.
//!
//! Protected-content gating operates on the shared pre-render text model:
//! the HTML render gates (validation, pre-render section lint, post-render
//! document lint) are evaluated first, and the PDF's own document text model
//! is linted post-emission. Any blocking finding marks the outcome
//! export-blocked.
//!
//! It does not fetch network resources, embed external assets, compress
//! streams, write files, or emit professional approval, certification,
//! sealing, authentication, code-compliance, release-readiness, or lifecycle
//! claims. The app's derived webview print view keeps its ruled role as a
//! non-hash-bound convenience (`DEC-021`); this emitter does not replace it.

use open_pipe_stress_protected_content_linter as linter;
use open_pipe_stress_report_renderer as renderer;
use serde::Serialize;
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Fixed deterministic layout constants (US Letter, points).
// ---------------------------------------------------------------------------

const PAGE_WIDTH: f64 = 612.0;
const PAGE_HEIGHT: f64 = 792.0;
const MARGIN: f64 = 54.0;
const CONTENT_WIDTH: f64 = PAGE_WIDTH - 2.0 * MARGIN;
const CONTENT_TOP: f64 = PAGE_HEIGHT - MARGIN;
const CONTENT_BOTTOM: f64 = MARGIN;
/// Courier glyph advance ratio: every base-14 Courier glyph is 600/1000 em.
const COURIER_WIDTH_RATIO: f64 = 0.6;

const TITLE_SIZE: f64 = 13.0;
const TITLE_LEADING: f64 = 17.0;
const META_SIZE: f64 = 8.0;
const META_LEADING: f64 = 11.0;
const H2_SIZE: f64 = 11.0;
const H2_LEADING: f64 = 15.0;
const H3_SIZE: f64 = 9.0;
const H3_LEADING: f64 = 13.0;
const BODY_SIZE: f64 = 9.0;
const BODY_LEADING: f64 = 12.0;
const TABLE_SIZE: f64 = 8.0;
const TABLE_LEADING: f64 = 10.0;
const CELL_PAD: f64 = 3.0;
const BANNER_SIZE: f64 = 9.0;
const BANNER_LEADING: f64 = 12.0;
const BANNER_PAD: f64 = 6.0;
const BLOCK_SPACE: f64 = 6.0;
const SECTION_SPACE: f64 = 10.0;
/// Minimum sign-off row height so the human review record has writing room.
const SIGNOFF_MIN_ROW_HEIGHT: f64 = 26.0;

/// Fixed, non-varying document metadata (no dates, no IDs).
const PDF_INFO_TITLE: &str = "OpenPipeStress Calculation Report";
const PDF_INFO_PRODUCER: &str = "OpenPipeStress deterministic PDF emitter DEC-061";

/// Closing note carried by the PDF emission. The HTML document keeps its own
/// ruled derived-view note; this text names the PDF's evidence binding.
const PDF_CLOSING_NOTE: &str = "This PDF document is a deterministic emission of the assembled \
report sections; only its exact bytes are bound by the recorded SHA-256, which is recorded \
alongside the canonical HTML hash. Printed or paper copies of this document are derived views \
and are not hash-bound evidence.";

// ---------------------------------------------------------------------------
// Outcome types.
// ---------------------------------------------------------------------------

/// Result of one deterministic PDF emission. `pdf_bytes` is always produced
/// (a blocked document renders with a visible blocked banner for diagnosis);
/// `export_blocked` is the gate the caller must honor before saving or
/// exporting the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PdfEmissionOutcome {
    pub pdf_bytes: Vec<u8>,
    pub sha256_hex: String,
    pub page_count: usize,
    pub export_blocked: bool,
    pub blocking_reasons: Vec<String>,
    /// The canonical HTML hash this PDF hash is recorded alongside.
    pub canonical_html_sha256_hex: String,
    /// Findings from the post-emission lint of the PDF document text model.
    pub text_model_findings: Vec<renderer::OutcomeFinding>,
}

/// One hash-recorded member of the assembled full report package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PackageMember {
    pub role: String,
    pub file_name: String,
    pub media_type: String,
    pub sha256_hex: String,
    pub byte_length: usize,
}

/// The assembled full report package: the canonical HTML and PDF emissions
/// of one assembled section model, each hash-recorded per the existing
/// convention. Assembly makes no release-readiness or lifecycle claim.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FullReportPackageOutcome {
    pub render: renderer::RenderOutcome,
    pub pdf: PdfEmissionOutcome,
    pub export_blocked: bool,
    pub blocking_reasons: Vec<String>,
    pub members: Vec<PackageMember>,
}

// ---------------------------------------------------------------------------
// Public entry points.
// ---------------------------------------------------------------------------

/// Emit the hash-bound deterministic PDF for a renderable report input. All
/// HTML render gates are evaluated first (the PDF is never less gated than
/// the HTML emission of the same input).
pub fn emit_calculation_report_pdf(input: &renderer::RenderableReportInput) -> PdfEmissionOutcome {
    let render = renderer::render_calculation_report(input);
    pdf_from_render(input, &render)
}

/// Emit the deterministic PDF alongside an existing HTML render outcome of
/// the SAME input. The blocked banner mirrors the render's blocking reasons;
/// the PDF document text model is then linted post-emission (the analogue of
/// the renderer's post-render gate).
pub fn pdf_from_render(
    input: &renderer::RenderableReportInput,
    render: &renderer::RenderOutcome,
) -> PdfEmissionOutcome {
    let (pdf_bytes, page_count, text_model) = emit_pdf_document(input, &render.blocking_reasons);

    let lint_run = linter::lint_targets(
        "pdf-emitter-text-model",
        linter::LintConfiguration::public_surfaces_only("pdf-emitter-text-model"),
        vec![linter::LintTarget {
            target_id: "pdf-report-text-model".to_string(),
            path: "pdf_emitter://document".to_string(),
            surface: linter::SurfaceKind::PublicReportExample,
            text: text_model.join("\n"),
            provenance: renderer::lint_provenance(&input.calculation_report.provenance),
        }],
    );

    let mut blocking_reasons = render.blocking_reasons.clone();
    for finding in &lint_run.findings {
        if finding.is_blocking() {
            blocking_reasons.push(format!("pdf text-model lint: {:?}", finding.code));
        }
    }

    let sha256 = sha256_hex(&pdf_bytes);
    PdfEmissionOutcome {
        sha256_hex: sha256,
        page_count,
        export_blocked: !blocking_reasons.is_empty(),
        blocking_reasons,
        canonical_html_sha256_hex: render.sha256_hex.clone(),
        text_model_findings: lint_run.findings.iter().map(outcome_finding).collect(),
        pdf_bytes,
    }
}

/// Assemble the full report package: one deterministic HTML emission and one
/// deterministic PDF emission of the same assembled section model, each
/// SHA-256-recorded as a package member. No release-readiness, lifecycle,
/// professional, certification, or code-compliance claim is created.
pub fn assemble_full_report_package(
    input: &renderer::RenderableReportInput,
) -> FullReportPackageOutcome {
    let render = renderer::render_calculation_report(input);
    let pdf = pdf_from_render(input, &render);

    let members = vec![
        PackageMember {
            role: "canonical_rendered_report_html".to_string(),
            file_name: "calculation_report.html".to_string(),
            media_type: "text/html".to_string(),
            sha256_hex: render.sha256_hex.clone(),
            byte_length: render.html.len(),
        },
        PackageMember {
            role: "canonical_rendered_report_pdf".to_string(),
            file_name: "calculation_report.pdf".to_string(),
            media_type: "application/pdf".to_string(),
            sha256_hex: pdf.sha256_hex.clone(),
            byte_length: pdf.pdf_bytes.len(),
        },
    ];

    // The PDF blocking reasons are a superset of the render's (they start
    // from the render's reasons and add PDF text-model lint blockers).
    let blocking_reasons = pdf.blocking_reasons.clone();
    FullReportPackageOutcome {
        export_blocked: render.export_blocked || pdf.export_blocked,
        blocking_reasons,
        members,
        render,
        pdf,
    }
}

// ---------------------------------------------------------------------------
// Document emission: assembled section model -> page content streams.
// ---------------------------------------------------------------------------

fn emit_pdf_document(
    input: &renderer::RenderableReportInput,
    blocking_reasons: &[String],
) -> (Vec<u8>, usize, Vec<String>) {
    let assembled = renderer::assemble_report_sections(input);
    let mut emitter = Emitter::new();

    emitter.note(&input.report_title);
    emitter.text_block(&input.report_title, TITLE_SIZE, TITLE_LEADING, true);
    let meta_line = renderer::report_meta_text(input);
    emitter.note(&meta_line);
    emitter.text_block(&meta_line, META_SIZE, META_LEADING, false);

    let status_text = renderer::status_banner_text(&input.report_sections);
    emitter.note(&status_text);
    emitter.banner(&status_text, true);

    if !blocking_reasons.is_empty() {
        let blocked_text = renderer::export_blocked_banner_text(blocking_reasons);
        emitter.note(&blocked_text);
        emitter.banner(&blocked_text, true);
    }

    for section in &assembled {
        emitter.note(section.title);
        emitter.section_heading(section.title);
        for block in &section.blocks {
            match block {
                renderer::SectionBlock::Subheading { text } => {
                    emitter.note(text);
                    emitter.subheading(text);
                }
                renderer::SectionBlock::Paragraph { text, meta } => {
                    emitter.note(text);
                    if *meta {
                        emitter.text_block(text, META_SIZE, META_LEADING, false);
                    } else {
                        emitter.text_block(text, BODY_SIZE, BODY_LEADING, false);
                    }
                }
                renderer::SectionBlock::Table {
                    header,
                    rows,
                    signoff,
                } => {
                    emitter.note(&header.join(" | "));
                    for row in rows {
                        emitter.note(&row.join(" | "));
                    }
                    emitter.table_block(header, rows, *signoff);
                }
                renderer::SectionBlock::BoundaryBox { text } => {
                    emitter.note(text);
                    emitter.banner(text, false);
                }
            }
        }
    }

    emitter.note(PDF_CLOSING_NOTE);
    emitter.text_block(PDF_CLOSING_NOTE, META_SIZE, META_LEADING, false);

    let (pages, text_model) = emitter.finish();
    let page_count = pages.len();
    (build_pdf(&pages), page_count, text_model)
}

// ---------------------------------------------------------------------------
// Deterministic page layout.
// ---------------------------------------------------------------------------

struct Emitter {
    pages: Vec<Vec<u8>>,
    ops: Vec<u8>,
    y: f64,
    text_model: Vec<String>,
}

impl Emitter {
    fn new() -> Self {
        Emitter {
            pages: Vec::new(),
            ops: Vec::new(),
            y: CONTENT_TOP,
            text_model: Vec::new(),
        }
    }

    fn finish(mut self) -> (Vec<Vec<u8>>, Vec<String>) {
        let ops = std::mem::take(&mut self.ops);
        self.pages.push(ops);
        (self.pages, self.text_model)
    }

    /// Record one raw text entry of the document text model (lint surface).
    fn note(&mut self, text: &str) {
        self.text_model.push(text.to_string());
    }

    fn new_page(&mut self) {
        let ops = std::mem::take(&mut self.ops);
        self.pages.push(ops);
        self.y = CONTENT_TOP;
    }

    fn ensure(&mut self, needed: f64) {
        if self.y - needed < CONTENT_BOTTOM {
            self.new_page();
        }
    }

    fn draw_text(&mut self, x: f64, baseline: f64, size: f64, bold: bool, text: &str) {
        let font = if bold { "F2" } else { "F1" };
        self.ops.extend(
            format!(
                "BT /{font} {size} Tf 1 0 0 1 {x} {y} Tm ",
                size = fmt(size),
                x = fmt(x),
                y = fmt(baseline),
            )
            .bytes(),
        );
        self.ops.push(b'(');
        self.ops.extend(encode_pdf_text(text));
        self.ops.extend_from_slice(b") Tj ET\n");
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.ops.extend(
            format!(
                "{} {} {} {} re S\n",
                fmt(x),
                fmt(y),
                fmt(width),
                fmt(height)
            )
            .bytes(),
        );
    }

    fn stroke_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.ops
            .extend(format!("{} {} m {} {} l S\n", fmt(x1), fmt(y1), fmt(x2), fmt(y2)).bytes());
    }

    /// Flow one wrapped text block at the left margin.
    fn text_block(&mut self, text: &str, size: f64, leading: f64, bold: bool) {
        let lines = wrap_text(text, max_chars(CONTENT_WIDTH, size));
        for line in &lines {
            self.ensure(leading);
            self.y -= leading;
            self.draw_text(MARGIN, self.y + (leading - size) * 0.5, size, bold, line);
        }
        self.y -= BLOCK_SPACE;
    }

    /// Section heading with an underline rule.
    fn section_heading(&mut self, text: &str) {
        self.ensure(SECTION_SPACE + H2_LEADING + BODY_LEADING);
        self.y -= SECTION_SPACE;
        let lines = wrap_text(text, max_chars(CONTENT_WIDTH, H2_SIZE));
        for line in &lines {
            self.ensure(H2_LEADING);
            self.y -= H2_LEADING;
            self.draw_text(
                MARGIN,
                self.y + (H2_LEADING - H2_SIZE) * 0.5,
                H2_SIZE,
                true,
                line,
            );
        }
        self.y -= 2.0;
        self.stroke_line(MARGIN, self.y, MARGIN + CONTENT_WIDTH, self.y);
        self.y -= BLOCK_SPACE;
    }

    fn subheading(&mut self, text: &str) {
        let lines = wrap_text(text, max_chars(CONTENT_WIDTH, H3_SIZE));
        for line in &lines {
            self.ensure(H3_LEADING);
            self.y -= H3_LEADING;
            self.draw_text(
                MARGIN,
                self.y + (H3_LEADING - H3_SIZE) * 0.5,
                H3_SIZE,
                true,
                line,
            );
        }
        self.y -= 2.0;
    }

    /// Banner box: bordered rectangle with wrapped text (status banner,
    /// export-blocked banner, professional-boundary box). A banner taller
    /// than one page area falls back to a plain flowed block.
    fn banner(&mut self, text: &str, bold: bool) {
        let width = CONTENT_WIDTH - 2.0 * BANNER_PAD;
        let lines = wrap_text(text, max_chars(width, BANNER_SIZE));
        let box_height = lines.len() as f64 * BANNER_LEADING + 2.0 * BANNER_PAD;
        if box_height > CONTENT_TOP - CONTENT_BOTTOM {
            self.text_block(text, BANNER_SIZE, BANNER_LEADING, bold);
            return;
        }
        self.ensure(box_height);
        let top = self.y;
        self.stroke_rect(MARGIN, top - box_height, CONTENT_WIDTH, box_height);
        for (index, line) in lines.iter().enumerate() {
            let baseline = top - BANNER_PAD - (index as f64 + 1.0) * BANNER_LEADING
                + (BANNER_LEADING - BANNER_SIZE) * 0.5;
            self.draw_text(MARGIN + BANNER_PAD, baseline, BANNER_SIZE, bold, line);
        }
        self.y = top - box_height - BLOCK_SPACE;
    }

    /// Table: equal-width columns, one repeated header row, per-cell word
    /// wrap, grid strokes. Rows split deterministically across pages when
    /// taller than the remaining page area.
    fn table_block(&mut self, header: &[String], rows: &[Vec<String>], signoff: bool) {
        let column_count = header.len().max(1);
        let column_width = CONTENT_WIDTH / column_count as f64;
        let cell_chars = max_chars(column_width - 2.0 * CELL_PAD, TABLE_SIZE);

        let header_cells: Vec<Vec<String>> = (0..column_count)
            .map(|index| {
                wrap_text(
                    header.get(index).map(String::as_str).unwrap_or(""),
                    cell_chars,
                )
            })
            .collect();
        let header_lines = row_line_count(&header_cells);
        let header_height = header_lines as f64 * TABLE_LEADING + 2.0 * CELL_PAD;

        self.ensure(header_height + TABLE_LEADING + 2.0 * CELL_PAD);
        self.draw_table_row(
            &header_cells,
            0,
            header_lines,
            header_height,
            column_width,
            true,
        );

        for row in rows {
            let cells: Vec<Vec<String>> = (0..column_count)
                .map(|index| {
                    wrap_text(row.get(index).map(String::as_str).unwrap_or(""), cell_chars)
                })
                .collect();
            let total_lines = row_line_count(&cells);
            let mut line_offset = 0usize;
            loop {
                let remaining = self.y - CONTENT_BOTTOM;
                let available_lines = ((remaining - 2.0 * CELL_PAD) / TABLE_LEADING).floor() as i64;
                if available_lines < 1 {
                    self.new_page();
                    self.draw_table_row(
                        &header_cells,
                        0,
                        header_lines,
                        header_height,
                        column_width,
                        true,
                    );
                    continue;
                }
                let take = (total_lines - line_offset).min(available_lines as usize);
                let mut chunk_height = take as f64 * TABLE_LEADING + 2.0 * CELL_PAD;
                if signoff && line_offset == 0 && take == total_lines {
                    chunk_height = chunk_height.max(SIGNOFF_MIN_ROW_HEIGHT);
                }
                self.draw_table_row(&cells, line_offset, take, chunk_height, column_width, false);
                line_offset += take;
                if line_offset >= total_lines {
                    break;
                }
                self.new_page();
                self.draw_table_row(
                    &header_cells,
                    0,
                    header_lines,
                    header_height,
                    column_width,
                    true,
                );
            }
        }
        self.y -= BLOCK_SPACE;
    }

    /// Draw one table row chunk: `line_count` wrapped lines per cell starting
    /// at `line_offset`, inside a full-width grid of `row_height`.
    fn draw_table_row(
        &mut self,
        cells: &[Vec<String>],
        line_offset: usize,
        line_count: usize,
        row_height: f64,
        column_width: f64,
        bold: bool,
    ) {
        let top = self.y;
        for (column, cell_lines) in cells.iter().enumerate() {
            let x = MARGIN + column as f64 * column_width;
            self.stroke_rect(x, top - row_height, column_width, row_height);
            for line_index in 0..line_count {
                if let Some(line) = cell_lines.get(line_offset + line_index) {
                    let baseline = top - CELL_PAD - (line_index as f64 + 1.0) * TABLE_LEADING
                        + (TABLE_LEADING - TABLE_SIZE) * 0.5;
                    self.draw_text(x + CELL_PAD, baseline, TABLE_SIZE, bold, line);
                }
            }
        }
        self.y = top - row_height;
    }
}

fn row_line_count(cells: &[Vec<String>]) -> usize {
    cells
        .iter()
        .map(|lines| lines.len())
        .max()
        .unwrap_or(1)
        .max(1)
}

fn max_chars(width: f64, size: f64) -> usize {
    let count = (width / (size * COURIER_WIDTH_RATIO)).floor() as i64;
    count.max(1) as usize
}

/// Greedy deterministic word wrap on Unicode scalar counts (Courier is
/// monospace, so character count is exact width). Words longer than one line
/// are hard-split.
fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;
    for word in text.split_whitespace() {
        let mut remainder: Vec<char> = word.chars().collect();
        while !remainder.is_empty() {
            let space_needed = if current_len == 0 { 0 } else { 1 };
            let available = max_chars.saturating_sub(current_len + space_needed);
            if available == 0 {
                lines.push(std::mem::take(&mut current));
                current_len = 0;
                continue;
            }
            if remainder.len() <= available {
                if current_len > 0 {
                    current.push(' ');
                    current_len += 1;
                }
                current.extend(remainder.iter());
                current_len += remainder.len();
                remainder.clear();
            } else if current_len == 0 {
                let taken: String = remainder.drain(..max_chars.min(remainder.len())).collect();
                lines.push(taken);
            } else {
                lines.push(std::mem::take(&mut current));
                current_len = 0;
            }
        }
    }
    if current_len > 0 {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

// ---------------------------------------------------------------------------
// Deterministic PDF byte assembly.
// ---------------------------------------------------------------------------

/// Deterministic minimal-fraction number formatting for content streams.
fn fmt(value: f64) -> String {
    let mut text = format!("{value:.2}");
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    if text == "-0" {
        text = "0".to_string();
    }
    text
}

/// Map one char to its WinAnsiEncoding (CP1252) byte; unmappable characters
/// become '?' deterministically.
fn winansi_byte(ch: char) -> u8 {
    let code_point = ch as u32;
    match code_point {
        0x20..=0x7E => code_point as u8,
        0xA0..=0xFF => code_point as u8,
        0x20AC => 0x80,
        0x201A => 0x82,
        0x0192 => 0x83,
        0x201E => 0x84,
        0x2026 => 0x85,
        0x2020 => 0x86,
        0x2021 => 0x87,
        0x02C6 => 0x88,
        0x2030 => 0x89,
        0x0160 => 0x8A,
        0x2039 => 0x8B,
        0x0152 => 0x8C,
        0x017D => 0x8E,
        0x2018 => 0x91,
        0x2019 => 0x92,
        0x201C => 0x93,
        0x201D => 0x94,
        0x2022 => 0x95,
        0x2013 => 0x96,
        0x2014 => 0x97,
        0x02DC => 0x98,
        0x2122 => 0x99,
        0x0161 => 0x9A,
        0x203A => 0x9B,
        0x0153 => 0x9C,
        0x017E => 0x9E,
        0x0178 => 0x9F,
        _ => b'?',
    }
}

/// Encode text as a PDF literal-string body: WinAnsi bytes with `(`, `)`,
/// `\` escaped and residual control bytes emitted as octal escapes.
fn encode_pdf_text(text: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(text.len());
    for ch in text.chars() {
        let byte = winansi_byte(ch);
        match byte {
            b'(' | b')' | b'\\' => {
                bytes.push(b'\\');
                bytes.push(byte);
            }
            0x00..=0x1F => {
                bytes.extend(format!("\\{byte:03o}").bytes());
            }
            _ => bytes.push(byte),
        }
    }
    bytes
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

/// Assemble the final PDF bytes: fixed object numbering (1 Catalog, 2 Pages,
/// 3 Courier, 4 Courier-Bold, 5 Info, then Page/Contents pairs), uncompressed
/// content streams, a correct xref table, and a trailer with no /ID.
fn build_pdf(pages: &[Vec<u8>]) -> Vec<u8> {
    let object_count = 5 + 2 * pages.len();
    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<usize> = vec![0; object_count + 1];
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    let kids: Vec<String> = (0..pages.len())
        .map(|index| format!("{} 0 R", 6 + 2 * index))
        .collect();

    begin_object(&mut out, &mut offsets, 1);
    out.extend_from_slice(b"<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    begin_object(&mut out, &mut offsets, 2);
    out.extend(
        format!(
            "<< /Type /Pages /Kids [{}] /Count {} >>\nendobj\n",
            kids.join(" "),
            pages.len()
        )
        .bytes(),
    );

    begin_object(&mut out, &mut offsets, 3);
    out.extend_from_slice(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier /Encoding /WinAnsiEncoding >>\nendobj\n",
    );

    begin_object(&mut out, &mut offsets, 4);
    out.extend_from_slice(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier-Bold /Encoding /WinAnsiEncoding >>\nendobj\n",
    );

    begin_object(&mut out, &mut offsets, 5);
    out.extend(
        format!("<< /Title ({PDF_INFO_TITLE}) /Producer ({PDF_INFO_PRODUCER}) >>\nendobj\n")
            .bytes(),
    );

    for (index, content) in pages.iter().enumerate() {
        let page_object = 6 + 2 * index;
        let content_object = page_object + 1;
        begin_object(&mut out, &mut offsets, page_object);
        out.extend(
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font \
<< /F1 3 0 R /F2 4 0 R >> >> /Contents {} 0 R >>\nendobj\n",
                fmt(PAGE_WIDTH),
                fmt(PAGE_HEIGHT),
                content_object
            )
            .bytes(),
        );
        begin_object(&mut out, &mut offsets, content_object);
        out.extend(format!("<< /Length {} >>\nstream\n", content.len()).bytes());
        out.extend_from_slice(content);
        out.extend_from_slice(b"endstream\nendobj\n");
    }

    let xref_offset = out.len();
    out.extend(format!("xref\n0 {}\n", object_count + 1).bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        out.extend(format!("{offset:010} 00000 n \n").bytes());
    }
    out.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R /Info 5 0 R >>\nstartxref\n{}\n%%EOF\n",
            object_count + 1,
            xref_offset
        )
        .bytes(),
    );
    out
}

fn begin_object(out: &mut Vec<u8>, offsets: &mut [usize], number: usize) {
    offsets[number] = out.len();
    out.extend(format!("{number} 0 obj\n").bytes());
}

fn outcome_finding(finding: &linter::LintFinding) -> renderer::OutcomeFinding {
    renderer::OutcomeFinding {
        finding_id: finding.finding_id.clone(),
        code: format!("{:?}", finding.code),
        severity: format!("{:?}", finding.severity).to_ascii_lowercase(),
        path: finding.source_location.path.clone(),
        line: finding.source_location.line,
        column: finding.source_location.column,
        excerpt: finding.excerpt.clone(),
        message: finding.message.clone(),
        blocking: finding.is_blocking(),
    }
}
