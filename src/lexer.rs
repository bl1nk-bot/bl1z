use crate::error::{ErrorKind, FormulaError};
use crate::span::{Position, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier, // ชื่อตัวแปร, ฟังก์ชัน
    Number,     // ตัวเลข
    String,     // ข้อความใน "..."
    LParen,     // (
    RParen,     // )
    Comma,      // ,
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Bang,       // !
    AndAnd,     // &&
    OrOr,       // ||
    EqEq,       // ==
    NotEq,      // !=
    Lt,         // <
    Gt,         // >
    LtEq,       // <=
    GtEq,       // >=
    True,       // true
    False,      // false
    Null,       // null
    Dot,        // .
    LBracket,   // [
    RBracket,   // ]
    LBrace,     // {
    RBrace,     // }
    Colon,      // :
    Eof,        // end of file
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

/// Lexer struct เก็บสถานะการอ่าน
struct Lexer<'a> {
    source: &'a str,
    pos: usize,         // ตำแหน่ง byte ใน source
    line: usize,        // บรรทัดปัจจุบัน (เริ่มที่ 1)
    col: usize,         // คอลัมน์ปัจจุบัน (เริ่มที่ 1)
    tokens: Vec<Token>, // token ที่อ่านได้แล้ว
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
        }
    }

    /// ดูตัวอักษรปัจจุบันโดยไม่เลื่อนตำแหน่ง
    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// เลื่อนไปยังตัวถัดไปและคืนค่าตัวปัจจุบัน
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += c.len_utf8();
        }
        ch
    }

    /// สร้าง Span จากตำแหน่งที่บันทึกไว้
    fn make_span(&self, start_line: usize, start_col: usize) -> Span {
        Span {
            start: Position {
                line: start_line,
                column: start_col,
            },
            end: Position {
                line: self.line,
                column: self.col,
            },
        }
    }

    /// เพิ่ม token ลงใน list
    fn push_token(&mut self, kind: TokenKind, lexeme: String, start_line: usize, start_col: usize) {
        let span = self.make_span(start_line, start_col);
        self.tokens.push(Token { kind, span, lexeme });
    }

    /// อ่าน identifier หรือ keyword
    fn scan_identifier(&mut self, first: char) {
        let start_line = self.line;
        let start_col = self.col;
        let mut lexeme = String::new();
        lexeme.push(first);
        self.advance(); // เลื่อนไปหลัง first
                        // เดินหน้าต่อไปตราบใดเป็นตัวอักษร ตัวเลข หรือ '_'
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                lexeme.push(c);
                self.advance();
            } else {
                break;
            }
        }
        // ตรวจคำสงวน
        let kind = match lexeme.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => TokenKind::Identifier,
        };
        self.push_token(kind, lexeme, start_line, start_col);
    }

    /// อ่านตัวเลข
    fn scan_number(&mut self, first: char) {
        let start_line = self.line;
        let start_col = self.col;
        let mut lexeme = String::new();
        lexeme.push(first);
        self.advance(); // เลื่อนไปหลัง first
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                lexeme.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.push_token(TokenKind::Number, lexeme, start_line, start_col);
    }

    /// อ่านข้อความใน "..."
    fn scan_string(&mut self) -> Result<(), FormulaError> {
        let start_line = self.line;
        let start_col = self.col;
        let mut lexeme = String::new();
        // ข้าม " ตัวแรก
        self.advance(); // กิน '"'

        loop {
            match self.peek() {
                None => {
                    return Err(FormulaError::new(
                        ErrorKind::LexError,
                        "E101",
                        "ไม่พบเครื่องหมายปิดข้อความ",
                        Some(self.make_span(start_line, start_col)),
                    ));
                }
                Some('"') => {
                    self.advance(); // กิน '"' ปิด
                    break;
                }
                Some('\\') => {
                    // อนุญาต escape ง่ายๆ เช่น \" \\
                    self.advance(); // กิน '\'
                    if let Some(esc) = self.peek() {
                        lexeme.push(match esc {
                            '"' => '"',
                            '\\' => '\\',
                            'n' => '\n',
                            't' => '\t',
                            _ => esc, // ใส่ตามเดิม
                        });
                        self.advance();
                    }
                }
                Some(c) => {
                    lexeme.push(c);
                    self.advance();
                }
            }
        }
        self.push_token(TokenKind::String, lexeme, start_line, start_col);
        Ok(())
    }

    /// ฟังก์ชันหลักในการ tokenize
    fn tokenize(mut self) -> Result<Vec<Token>, FormulaError> {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '(' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::LParen, "(".to_string(), sl, sc);
                }
                ')' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::RParen, ")".to_string(), sl, sc);
                }
                ',' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Comma, ",".to_string(), sl, sc);
                }
                '+' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Plus, "+".to_string(), sl, sc);
                }
                '-' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Minus, "-".to_string(), sl, sc);
                }
                '*' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Star, "*".to_string(), sl, sc);
                }
                '/' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Slash, "/".to_string(), sl, sc);
                }
                '!' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::NotEq, "!=".to_string(), sl, sc);
                    } else {
                        self.push_token(TokenKind::Bang, "!".to_string(), sl, sc);
                    }
                }
                '=' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::EqEq, "==".to_string(), sl, sc);
                    } else {
                        return Err(FormulaError::new(
                            ErrorKind::LexError,
                            "E101",
                            "พบ '=' โดยไม่มี '=' ตามหลัง (อาจหมายถึง ==?)",
                            Some(self.make_span(sl, sc)),
                        ));
                    }
                }
                '<' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::LtEq, "<=".to_string(), sl, sc);
                    } else {
                        self.push_token(TokenKind::Lt, "<".to_string(), sl, sc);
                    }
                }
                '>' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::GtEq, ">=".to_string(), sl, sc);
                    } else {
                        self.push_token(TokenKind::Gt, ">".to_string(), sl, sc);
                    }
                }
                '&' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('&') {
                        self.advance();
                        self.push_token(TokenKind::AndAnd, "&&".to_string(), sl, sc);
                    } else {
                        return Err(FormulaError::new(
                            ErrorKind::LexError,
                            "E101",
                            "พบ '&' ตัวเดียว ต้องใช้ '&&'",
                            Some(self.make_span(sl, sc)),
                        ));
                    }
                }
                '|' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    if self.peek() == Some('|') {
                        self.advance();
                        self.push_token(TokenKind::OrOr, "||".to_string(), sl, sc);
                    } else {
                        return Err(FormulaError::new(
                            ErrorKind::LexError,
                            "E101",
                            "พบ '|' ตัวเดียว ต้องใช้ '||'",
                            Some(self.make_span(sl, sc)),
                        ));
                    }
                }
                '"' => {
                    self.scan_string()?;
                }
                '.' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Dot, ".".to_string(), sl, sc);
                }
                '[' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::LBracket, "[".to_string(), sl, sc);
                }
                ']' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::RBracket, "]".to_string(), sl, sc);
                }
                '{' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::LBrace, "{".to_string(), sl, sc);
                }
                '}' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::RBrace, "}".to_string(), sl, sc);
                }
                ':' => {
                    let sl = self.line;
                    let sc = self.col;
                    self.advance();
                    self.push_token(TokenKind::Colon, ":".to_string(), sl, sc);
                }
                c if c.is_alphabetic() || c == '_' => {
                    self.scan_identifier(c);
                }
                c if c.is_ascii_digit() => {
                    self.scan_number(c);
                }
                _ => {
                    let sl = self.line;
                    let sc = self.col;
                    return Err(FormulaError::new(
                        ErrorKind::LexError,
                        "E101",
                        &format!("อักขระไม่รู้จัก '{}'", c),
                        Some(self.make_span(sl, sc)),
                    ));
                }
            }
        }
        // เพิ่ม EOF
        self.push_token(TokenKind::Eof, "".to_string(), self.line, self.col);
        Ok(self.tokens)
    }
}

/// ฟังก์ชัน public สำหรับ tokenize
pub fn tokenize(source: &str) -> Result<Vec<Token>, FormulaError> {
    Lexer::new(source).tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_identifier() {
        let tokens = tokenize("score").unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "score");
    }

    #[test]
    fn test_arithmetic_expression() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Number,
                TokenKind::Plus,
                TokenKind::Number,
                TokenKind::Star,
                TokenKind::Number,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("if(score > 50, \"pass\", \"fail\")").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(kinds[0], TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "if");
        assert!(kinds.contains(&TokenKind::String));
    }

    #[test]
    fn test_unterminated_string() {
        let result = tokenize("\"hello world");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::LexError);
        assert_eq!(err.code, "E101");
    }

    #[test]
    fn test_invalid_single_ampersand() {
        let result = tokenize("a & b");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::LexError);
        assert_eq!(err.code, "E101");
    }

    #[test]
    fn test_invalid_single_pipe() {
        let result = tokenize("a | b");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::LexError);
        assert_eq!(err.code, "E101");
    }

    #[test]
    fn test_invalid_single_equals() {
        let result = tokenize("a = b");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::LexError);
        assert_eq!(err.code, "E101");
    }

    // -- Tests for LBracket/RBracket (added in Phase 6.1) --

    #[test]
    fn test_lbracket_token() {
        let tokens = tokenize("[").unwrap();
        // [ + EOF
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::LBracket);
        assert_eq!(tokens[0].lexeme, "[");
    }

    #[test]
    fn test_rbracket_token() {
        let tokens = tokenize("]").unwrap();
        // ] + EOF
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::RBracket);
        assert_eq!(tokens[0].lexeme, "]");
    }

    #[test]
    fn test_empty_array_brackets() {
        let tokens = tokenize("[]").unwrap();
        // [ ] EOF
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::LBracket);
        assert_eq!(tokens[1].kind, TokenKind::RBracket);
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_array_with_numbers_tokens() {
        let tokens = tokenize("[1, 2, 3]").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LBracket,
                TokenKind::Number,
                TokenKind::Comma,
                TokenKind::Number,
                TokenKind::Comma,
                TokenKind::Number,
                TokenKind::RBracket,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_bracket_lexemes() {
        let tokens = tokenize("[42]").unwrap();
        assert_eq!(tokens[0].lexeme, "[");
        assert_eq!(tokens[1].lexeme, "42");
        assert_eq!(tokens[2].lexeme, "]");
    }

    #[test]
    fn test_nested_array_brackets() {
        let tokens = tokenize("[[1]]").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LBracket,
                TokenKind::LBracket,
                TokenKind::Number,
                TokenKind::RBracket,
                TokenKind::RBracket,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_bracket_inside_function_call_tokens() {
        let tokens = tokenize("sum([1])").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Identifier,
                TokenKind::LParen,
                TokenKind::LBracket,
                TokenKind::Number,
                TokenKind::RBracket,
                TokenKind::RParen,
                TokenKind::Eof,
            ]
        );
    }

    // -- Tests for Map literal tokens (added in Phase 6.2) --

    #[test]
    fn test_lbrace_token() {
        let tokens = tokenize("{").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::LBrace);
        assert_eq!(tokens[0].lexeme, "{");
    }

    #[test]
    fn test_rbrace_token() {
        let tokens = tokenize("}").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::RBrace);
        assert_eq!(tokens[0].lexeme, "}");
    }

    #[test]
    fn test_colon_token() {
        let tokens = tokenize(":").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Colon);
        assert_eq!(tokens[0].lexeme, ":");
    }

    #[test]
    fn test_empty_map_braces() {
        let tokens = tokenize("{}").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![TokenKind::LBrace, TokenKind::RBrace, TokenKind::Eof]
        );
    }

    #[test]
    fn test_map_with_key_value_tokens() {
        let tokens = tokenize("{key: \"value\"}").unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LBrace,
                TokenKind::Identifier,
                TokenKind::Colon,
                TokenKind::String,
                TokenKind::RBrace,
                TokenKind::Eof,
            ]
        );
    }
}
