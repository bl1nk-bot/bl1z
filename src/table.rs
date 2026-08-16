//! Reusable table renderer: define a schema once, render many tables — as
//! a box-drawing terminal frame or a GitHub-style markdown table. No manual
//! formatting either way.
//!
//! The terminal frame is auto-sized to the screen (COLUMNS env → `stty
//! size` → 80 fallback) and columns grow/shrink to keep every line inside
//! it; per-column alignment, per-cell ANSI color, CJK/Thai-aware ellipsis
//! truncation. Markdown cells are never truncated — the renderer aligns
//! them. Works across OSes.

use std::fmt::Write;
use std::sync::OnceLock;

use unicode_width::UnicodeWidthStr;

// Full palette/alignment is the public API for future table kinds in bl1z;
// unused variants stay for reuse.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Bold,
    BoldRed,
    BoldGreen,
    BoldCyan,
    BoldWhite,
}

impl Color {
    pub fn ansi(self) -> &'static str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Gray => "\x1b[90m",
            Color::Bold => "\x1b[1m",
            Color::BoldRed => "\x1b[1;31m",
            Color::BoldGreen => "\x1b[1;32m",
            Color::BoldCyan => "\x1b[1;36m",
            Color::BoldWhite => "\x1b[1;37m",
        }
    }
}

pub struct ColumnSpec {
    pub title: &'static str,
    /// Cap on this column's width; the actual width auto-sizes to fit
    /// both the content and the terminal.
    pub max_width: usize,
    pub align: Align,
    /// Header color for this column; row cells only take their own color.
    pub color: Option<Color>,
}

pub struct Cell {
    pub text: String,
    pub color: Option<Color>,
}

impl Cell {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into(), color: None }
    }

    pub fn colored<T: Into<String>>(text: T, color: Color) -> Self {
        Self { text: text.into(), color: Some(color) }
    }
}

pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }
}

pub struct TableSchema {
    pub left_pad: usize,
    pub right_pad: usize,
    pub columns: Vec<ColumnSpec>,
}

pub struct TableRenderer {
    schema: TableSchema,
}

const RESET: &str = "\x1b[0m";

fn visible_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

fn truncate_ellipsis(s: &str, max_width: usize) -> String {
    if visible_width(s) <= max_width {
        return s.to_string();
    }
    if max_width <= 1 {
        return "…".to_string();
    }

    let mut out = String::new();
    let mut used = 0;

    for ch in s.chars() {
        let mut buf = [0u8; 4];
        let ch_s = ch.encode_utf8(&mut buf);
        let cw = visible_width(ch_s);
        if used + cw + 1 > max_width {
            break;
        }
        out.push(ch);
        used += cw;
    }

    out.push('…');
    out
}

fn pad_to_width(s: &str, width: usize, align: Align) -> String {
    let current = visible_width(s);
    if current >= width {
        return s.to_string();
    }

    let spaces = width - current;
    match align {
        Align::Left => format!("{s}{}", " ".repeat(spaces)),
        Align::Right => format!("{}{}", " ".repeat(spaces), s),
        Align::Center => {
            let left = spaces / 2;
            let right = spaces - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    }
}

/// Escape cell text for GFM markdown tables: backslash, pipe, and newline.
/// Must be used consistently in both width calculation and rendering.
fn md_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('|', "\\|").replace('\r', " ").replace('\n', " ")
}

fn ansi_wrap(text: &str, color: Option<Color>) -> String {
    match color {
        Some(c) => format!("{}{}{}", c.ansi(), text, RESET),
        None => text.to_string(),
    }
}

/// Strip newlines and C0/C1 control sequences from cell text to prevent
/// fake rows and terminal control injection in table output.
fn sanitize_cell_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\n' | '\r' | '\t' => out.push(' '),
            '\x1b' => {
                // Skip ANSI escape: ESC ... (letter or ST)
                // SGR: ESC [ ... m ; CSI: ESC [ ... letter
                // OSC: ESC ] ... BEL or ESC ] ... ST (\x1b\\)
                if let Some(next) = chars.next() {
                    match next {
                        '[' => {
                            // CSI/SGR: consume until final byte (@-~)
                            for c in chars.by_ref() {
                                if ('\x40'..='\x7e').contains(&c) {
                                    break;
                                }
                            }
                        }
                        ']' => {
                            // OSC: consume until BEL (\x07) or ST (\x1b\\)
                            let mut prev = next;
                            for c in chars.by_ref() {
                                if c == '\x07' {
                                    break;
                                }
                                if prev == '\x1b' && c == '\\' {
                                    break;
                                }
                                prev = c;
                            }
                        }
                        '(' | ')' | '#' | '%' => {
                            // Charset/select sequences: consume one more char
                            chars.next();
                        }
                        _ => {} // other ESC seqs: just drop the ESC + next
                    }
                }
            }
            c if c < '\x20' || c == '\x7f' => {
                // Other C0 control chars: drop
            }
            c if ('\u{80}'..='\u{9f}').contains(&c) => {
                // C1 control chars: drop
            }
            _ => out.push(ch),
        }
    }
    out
}

fn make_border(
    left: char,
    mid: char,
    right: char,
    widths: &[usize],
    left_pad: usize,
    right_pad: usize,
) -> String {
    let mut s = String::new();
    s.push(left);
    for (i, w) in widths.iter().enumerate() {
        s.push_str(&"─".repeat(w + left_pad + right_pad));
        s.push(if i + 1 == widths.len() { right } else { mid });
    }
    s
}

// ponytail: COLUMNS env → `stty size` (cached once per process) → 80.
// No terminal-size crate; a piped/CI run just falls back to 80.
static STTY_WIDTH: OnceLock<usize> = OnceLock::new();

fn terminal_width() -> usize {
    if let Ok(c) = std::env::var("COLUMNS") {
        if let Ok(n) = c.trim().parse::<usize>() {
            if n > 0 {
                return n;
            }
        }
    }
    *STTY_WIDTH.get_or_init(|| {
        std::process::Command::new("stty")
            .arg("size")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.split_whitespace().nth(1).and_then(|c| c.parse().ok()))
            .filter(|n| *n > 0)
            .unwrap_or(80)
    })
}

/// Shrink the widest shrinkable column (down to `floor`) until the widths
/// fit `available` display columns. Greedy, O(width × n) — trivial for CLI.
fn shrink_to_available(widths: &mut [usize], available: usize, floor: usize) {
    let mut total: usize = widths.iter().sum();
    while total > available {
        let mut best: Option<usize> = None;
        let mut best_w = 0usize;
        for (i, w) in widths.iter().enumerate() {
            if *w > floor && (best.is_none() || *w > best_w) {
                best = Some(i);
                best_w = *w;
            }
        }
        let Some(i) = best else { break };
        widths[i] -= 1;
        total -= 1;
    }
}

// ponytail: parses only `max_line_length` for the root / `[*.md]` section of
// the repo's .editorconfig — enough for table generation, not a full INI
// parser. Missing or `off` means no line cap.
fn editorconfig_md_max_line_length() -> Option<usize> {
    static CACHE: OnceLock<Option<usize>> = OnceLock::new();
    *CACHE.get_or_init(|| {
        let text = std::fs::read_to_string(".editorconfig").ok()?;
        parse_md_max_line_length(&text)
    })
}

fn parse_md_max_line_length(text: &str) -> Option<usize> {
    let mut section = "";
    let mut value: Option<usize> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = &line[1..line.len() - 1];
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "max_line_length" && (section == "*" || section == "*.md") {
                value = v.trim().parse().ok(); // "off" fails to parse => no cap
            }
        }
    }
    value
}

impl TableRenderer {
    pub fn new(schema: TableSchema) -> Self {
        Self { schema }
    }

    pub fn render(&self, rows: &[Row]) -> String {
        self.render_at(rows, terminal_width())
    }

    /// Render into a frame of exactly `outer_width` columns. Deterministic —
    /// used by callers that know their width and by tests.
    pub fn render_at(&self, rows: &[Row], outer_width: usize) -> String {
        let widths = self.layout(rows, outer_width);
        let (lp, rp) = (self.schema.left_pad, self.schema.right_pad);
        let top = make_border('┌', '┬', '┐', &widths, lp, rp);
        let sep = make_border('├', '┼', '┤', &widths, lp, rp);
        let bot = make_border('└', '┴', '┘', &widths, lp, rp);

        let mut out = String::new();
        let _ = writeln!(out, "{top}");

        let header: Vec<Cell> = self
            .schema
            .columns
            .iter()
            .map(|c| Cell { text: c.title.to_string(), color: c.color })
            .collect();

        let _ = writeln!(out, "{}", self.render_row(&widths, &header));
        let _ = writeln!(out, "{sep}");

        for row in rows {
            let _ = writeln!(out, "{}", self.render_row(&widths, &row.cells));
        }

        let _ = write!(out, "{bot}");
        out
    }

    /// GitHub-style markdown table honoring the repo's .editorconfig
    /// `max_line_length` (root/`[*.md]`). Pipe-delimited rows, alignment
    /// markers (`:---` / `---:` / `:---:`), `|` escaped as `\|`, newlines
    /// collapsed to spaces. Columns are capped at `max_width`; when the
    /// line cap demands it, columns shrink and cells get `…` so no line
    /// exceeds the cap.
    pub fn render_markdown(&self, rows: &[Row]) -> String {
        self.render_markdown_capped(rows, editorconfig_md_max_line_length())
    }

    /// Markdown table with an explicit hard line cap (`None` = no cap).
    pub fn render_markdown_capped(&self, rows: &[Row], cap: Option<usize>) -> String {
        let cols = &self.schema.columns;
        let n = cols.len();

        let mut col_w: Vec<usize> = Vec::with_capacity(n);
        for (i, col) in cols.iter().enumerate() {
            let mut m = visible_width(col.title);
            for row in rows {
                if let Some(cell) = row.cells.get(i) {
                    let md = md_escape(&cell.text);
                    m = m.max(visible_width(&md));
                }
            }
            col_w.push(m.min(col.max_width));
        }

        if let Some(cap) = cap {
            // GFM separator cells need >= 3 dashes, so a column can't be
            // narrower than 4 (5 when centered) even under a hard line cap.
            for (i, col) in cols.iter().enumerate() {
                let min_sep = match col.align {
                    Align::Center => 5,
                    _ => 4,
                };
                col_w[i] = col_w[i].max(min_sep);
            }
            let bars = n + 1;
            let pads = (self.schema.left_pad + self.schema.right_pad) * n;
            shrink_to_available(&mut col_w, cap.saturating_sub(bars + pads), 4);
        }

        let header: Vec<Cell> =
            cols.iter().map(|c| Cell { text: c.title.to_string(), color: None }).collect();

        let mut out = String::new();
        let _ = writeln!(out, "{}", self.md_line(&col_w, &header));

        out.push('|');
        for (i, col) in cols.iter().enumerate() {
            // separator cell is exactly `col_w[i]` wide (colons included),
            // so its line length matches the data rows
            let sep = match col.align {
                Align::Left => format!(":{}", "-".repeat(col_w[i].saturating_sub(1))),
                Align::Right => format!("{}:", "-".repeat(col_w[i].saturating_sub(1))),
                Align::Center => format!(":{}:", "-".repeat(col_w[i].saturating_sub(2))),
            };
            let _ = write!(out, " {sep} |");
        }
        out.push('\n');

        for row in rows {
            let _ = writeln!(out, "{}", self.md_line(&col_w, &row.cells));
        }
        out
    }

    fn md_line(&self, col_w: &[usize], cells: &[Cell]) -> String {
        let mut out = String::new();
        out.push('|');
        for (i, col) in self.schema.columns.iter().enumerate() {
            let cell = cells.get(i).map(|c| c.text.as_str()).unwrap_or("");
            let md = md_escape(cell);
            let truncated = truncate_ellipsis(&md, col_w[i]);
            let padded = pad_to_width(&truncated, col_w[i], col.align);
            let _ = write!(out, " {padded} |");
        }
        out
    }

    /// Column widths: content-based, expanded up to `max_width` to fill a
    /// wide terminal, shrunk (down to 1) to fit a narrow one. Truncation
    /// with `…` handles whatever is left.
    fn layout(&self, rows: &[Row], outer_width: usize) -> Vec<usize> {
        let cols = &self.schema.columns;
        let n = cols.len();
        let bars = n + 1;
        let pads = (self.schema.left_pad + self.schema.right_pad) * n;
        let available = outer_width.saturating_sub(bars + pads);

        let mut content: Vec<usize> = Vec::with_capacity(n);
        for (i, col) in cols.iter().enumerate() {
            let mut m = visible_width(col.title);
            for row in rows {
                if let Some(cell) = row.cells.get(i) {
                    let clean = sanitize_cell_text(&cell.text);
                    m = m.max(visible_width(&clean));
                }
            }
            content.push(m.min(col.max_width));
        }

        let mut widths = content.clone();
        let mut total: usize = widths.iter().sum();
        let total_max: usize = cols.iter().map(|c| c.max_width).sum();

        // Grow round-robin up to max_width while the frame has room.
        let target = available.min(total_max);
        let mut cursor = 0usize;
        while total < target {
            let mut grew = false;
            for _ in 0..n {
                let i = cursor % n;
                cursor += 1;
                if widths[i] < cols[i].max_width {
                    widths[i] += 1;
                    total += 1;
                    grew = true;
                    break;
                }
            }
            if !grew {
                break;
            }
        }

        // Shrink the widest shrinkable column until everything fits.
        shrink_to_available(&mut widths, available, 1);

        widths
    }

    fn render_row(&self, widths: &[usize], cells: &[Cell]) -> String {
        let mut out = String::new();
        out.push('│');

        for (i, col) in self.schema.columns.iter().enumerate() {
            let cell = cells.get(i);
            let raw = cell.map(|c| c.text.as_str()).unwrap_or("");
            let clean = sanitize_cell_text(raw);
            let truncated = truncate_ellipsis(&clean, widths[i]);
            let padded = pad_to_width(&truncated, widths[i], col.align);

            let color = cell.and_then(|c| c.color);
            let colored = ansi_wrap(&padded, color);

            out.push_str(&" ".repeat(self.schema.left_pad));
            out.push_str(&colored);
            out.push_str(&" ".repeat(self.schema.right_pad));
            out.push('│');
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> TableSchema {
        TableSchema {
            left_pad: 1,
            right_pad: 1,
            columns: vec![
                ColumnSpec { title: "ID", max_width: 10, align: Align::Left, color: None },
                ColumnSpec { title: "STATUS", max_width: 8, align: Align::Right, color: None },
            ],
        }
    }

    #[test]
    fn one_schema_renders_many_tables_with_same_frame() {
        let r = TableRenderer::new(schema());
        let t1 = r.render_at(&[Row::new(vec![Cell::new("aaa"), Cell::new("ok")])], 40);
        let t2 = r.render_at(&[Row::new(vec![Cell::new("bbb"), Cell::new("no")])], 40);
        assert_eq!(t1.lines().count(), 5); // top + header + sep + 1 row + bottom
        assert_eq!(t1.lines().next(), t2.lines().next(), "top border identical");
        let top_len = t1.lines().next().unwrap().chars().count();
        assert!(
            t1.lines().all(|l| l.chars().count() == top_len),
            "all lines must share one frame width"
        );
        assert!(top_len <= 40, "frame must not exceed the terminal width");
    }

    #[test]
    fn thai_and_cjk_truncate_with_ellipsis_inside_width() {
        let r = TableRenderer::new(schema());
        let out = r.render_at(
            &[Row::new(vec![
                Cell::new("กกกกกกกกกกกกกกกกกก"), // 34 halfwidth columns
                Cell::new("x"),
            ])],
            40,
        );
        let row = out.lines().nth(3).unwrap();
        // cell text region: strip "│ " prefix by chars, cut at first " │"
        let body: String = row.chars().skip(2).collect();
        let cell = body.split(" │").next().unwrap();
        assert!(cell.ends_with('…'));
        assert!(visible_width(cell) <= 10, "cell width {} > 10", visible_width(cell));
    }

    #[test]
    fn narrow_terminal_shrinks_columns_to_fit() {
        let r = TableRenderer::new(schema());
        let out = r.render_at(
            &[Row::new(vec![Cell::new("0123456789abcdef"), Cell::new("enabled")])],
            24, // narrow: 24 - 3 bars - 4 pads = 17 usable
        );
        for line in out.lines() {
            assert!(
                line.chars().count() <= 24,
                "line overflows: {line:?} ({} chars)",
                line.chars().count()
            );
        }
        assert!(out.lines().any(|l| l.contains('…')));
    }

    #[test]
    fn colors_are_emitted() {
        let r = TableRenderer::new(schema());
        let out = r.render_at(
            &[Row::new(vec![
                Cell::colored("aaa", Color::Cyan),
                Cell::colored("no", Color::BoldRed),
            ])],
            40,
        );
        assert!(out.contains("\x1b[36m"), "cyan missing: {out:?}");
        assert!(out.contains("\x1b[1;31m"), "bold-red missing: {out:?}");
        assert!(out.contains(RESET));
    }

    #[test]
    fn align_right_pads_on_the_left() {
        let out = truncate_ellipsis("abc", 10);
        let padded = pad_to_width(&out, 10, Align::Right);
        assert_eq!(padded, "       abc");
        assert_eq!(pad_to_width("ab", 4, Align::Center), " ab ");
    }

    #[test]
    fn markdown_table_is_well_formed() {
        let r = TableRenderer::new(schema());
        let out = r.render_markdown(&[
            Row::new(vec![Cell::new("a|b"), Cell::new("ok\nline2")]),
            Row::new(vec![Cell::new("x"), Cell::new("no")]),
        ]);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 4); // header + separator + 2 rows
        assert!(lines[0].starts_with('|') && lines[0].ends_with('|'));
        assert!(lines[1].contains("---"), "separator missing: {}", lines[1]);
        assert_eq!(lines[0].matches('|').count(), 3, "2 cols => 3 pipes");
        assert!(lines[2].contains(r"a\|b"), "pipe not escaped: {}", lines[2]);
        assert!(out.contains("ok line2"), "newline must collapse to space");
        assert!(out.contains("line2"), "markdown must never truncate");
    }

    #[test]
    fn pipes_align_across_mixed_width_lines() {
        let r = TableRenderer::new(schema());
        let out = r.render_at(
            &[
                Row::new(vec![Cell::new("กก"), Cell::new("ok")]), // Thai, width 4
                Row::new(vec![Cell::new("中文"), Cell::new("x")]), // CJK, width 4
                Row::new(vec![Cell::new("short"), Cell::new("y")]),
                Row::new(vec![Cell::new(""), Cell::new("enabled")]), // shorter than header
            ],
            40,
        );
        let lines: Vec<String> = out.lines().map(strip_ansi).collect();
        // compare display columns of the pipes (CJK chars are 2 wide)
        let pipes = |s: &str| {
            let mut col = 0usize;
            let mut pos = Vec::new();
            for c in s.chars() {
                if c == '│' {
                    pos.push(col);
                }
                col += visible_width(&c.to_string());
            }
            pos
        };
        let header_pos = pipes(&lines[1]); // header row, not the top border
        for l in lines.iter().filter(|l| l.contains('│')) {
            assert_eq!(pipes(l), header_pos, "pipes misaligned in line: {l:?}");
        }
    }

    #[test]
    fn border_matches_cell_region_when_pads_differ() {
        let s = TableSchema {
            left_pad: 1,
            right_pad: 2,
            columns: vec![
                ColumnSpec { title: "ID", max_width: 8, align: Align::Left, color: None },
                ColumnSpec { title: "S", max_width: 6, align: Align::Left, color: None },
            ],
        };
        let r = TableRenderer::new(s);
        let out = r.render_at(&[Row::new(vec![Cell::new("ab"), Cell::new("cd")])], 30);
        let lines: Vec<String> = out.lines().map(strip_ansi).collect();
        let top = lines[0].chars().count();
        assert!(
            lines.iter().all(|l| l.chars().count() == top),
            "border and rows must share one width"
        );
    }

    #[test]
    fn markdown_caps_columns_and_respects_line_cap() {
        let r = TableRenderer::new(schema()); // max_width 10/8
        let out = r.render_markdown_capped(
            &[Row::new(vec![Cell::new("0123456789abcdef"), Cell::new("0123456789")])],
            Some(22), // bars 3 + pads 4 => 15 usable
        );
        for l in out.lines() {
            assert!(l.chars().count() <= 22, "line over cap: {l:?} ({} chars)", l.chars().count());
        }
        assert!(out.contains('…'), "wide cells must truncate");
    }

    #[test]
    fn editorconfig_parser_reads_md_max_line_length() {
        assert_eq!(
            parse_md_max_line_length("root = true\n[*]\nmax_line_length = 120\n"),
            Some(120)
        );
        assert_eq!(parse_md_max_line_length("[*.md]\nmax_line_length = off\n"), None);
        assert_eq!(parse_md_max_line_length("[Makefile]\nmax_line_length = 10\n"), None);
        // [*.md] inherits the root value when it doesn't set its own
        assert_eq!(parse_md_max_line_length("[*]\nmax_line_length = 80\n[*.md]\n"), Some(80));
    }

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' && chars.peek() == Some(&'[') {
                chars.next();
                for c2 in chars.by_ref() {
                    if c2.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}
