use crate::ast::*;
use crate::error::{ErrorKind, FormulaError};
use crate::lexer::{Token, TokenKind};
use crate::span::Span;

/// สร้าง Span จาก token list โดยอ้างอิง index
fn token_span(tokens: &[Token], idx: usize) -> Span {
    tokens
        .get(idx)
        .map(|t| t.span)
        .expect("BUG: token index out of bounds")
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> TokenKind {
        self.tokens
            .get(self.pos)
            .map(|t| t.kind.clone())
            .unwrap_or(TokenKind::Eof)
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: TokenKind, err_msg: &str) -> Result<(), FormulaError> {
        if self.peek() != expected {
            return Err(FormulaError::new(
                ErrorKind::ParseError,
                "E201",
                err_msg,
                Some(token_span(self.tokens, self.pos)),
            ));
        }
        self.advance();
        Ok(())
    }

    /// Helper สำหรับ parse binary operators แบบ left associative
    fn parse_left_associative_binary<F>(
        &mut self,
        mut next_parser: F,
        token_ops: &[(TokenKind, BinaryOp)],
    ) -> Result<SpannedExpr, FormulaError>
    where
        F: FnMut(&mut Self) -> Result<SpannedExpr, FormulaError>,
    {
        let mut left = next_parser(self)?;
        while let Some((_, op)) = token_ops.iter().find(|(token, _)| *token == self.peek()) {
            self.advance();
            let right = next_parser(self)?;
            let span = Span {
                start: left.meta.span.start,
                end: right.meta.span.end,
            };
            left = SpannedExpr::new(
                Expr::BinaryExpr {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                },
                span,
            );
        }
        Ok(left)
    }

    /// expression = logical_or
    pub fn parse_expression(&mut self) -> Result<SpannedExpr, FormulaError> {
        // Phase 10: Check for fn keyword
        if self.peek() == TokenKind::Fn {
            return self.parse_function_def();
        }
        self.parse_logical_or()
    }

    /// Phase 10: Parse function definition: fn name(params) = body
    fn parse_function_def(&mut self) -> Result<SpannedExpr, FormulaError> {
        let fn_span = token_span(self.tokens, self.pos);
        self.advance(); // consume 'fn'

        // expect function name
        if self.peek() != TokenKind::Identifier {
            return Err(FormulaError::new(
                ErrorKind::ParseError,
                "E210",
                "ต้องการชื่อฟังก์ชัน (identifier) หลัง 'fn'",
                Some(token_span(self.tokens, self.pos)),
            ));
        }
        let name = self.advance().lexeme.clone();

        // expect '('
        self.expect(TokenKind::LParen, "ต้องการ '(' หลังชื่อฟังก์ชัน")?;

        // parse params
        let mut params = Vec::new();
        if self.peek() != TokenKind::RParen {
            loop {
                if self.peek() != TokenKind::Identifier {
                    return Err(FormulaError::new(
                        ErrorKind::ParseError,
                        "E210",
                        "พารามิเตอร์ต้องเป็น identifier",
                        Some(token_span(self.tokens, self.pos)),
                    ));
                }
                params.push(self.advance().lexeme.clone());
                if self.peek() == TokenKind::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Check for duplicate params
        let mut unique_params = std::collections::HashSet::new();
        for param in &params {
            if !unique_params.insert(param.clone()) {
                return Err(FormulaError::new(
                    ErrorKind::ParseError,
                    "E209",
                    &format!("พารามิเตอร์ซ้ำกันในฟังก์ชัน: '{}'", param),
                    Some(fn_span),
                ));
            }
        }

        self.expect(TokenKind::RParen, "ต้องการ ')' หลังพารามิเตอร์")?;
        self.expect(TokenKind::Eq, "ต้องการ '=' หลังพารามิเตอร์")?;

        let body = self.parse_expression()?;
        let span = Span {
            start: fn_span.start,
            end: body.meta.span.end,
        };
        Ok(SpannedExpr::new(
            Expr::FunctionDef {
                name,
                params,
                body: Box::new(body),
            },
            span,
        ))
    }

    /// logical_or = logical_and ('||' logical_and)*
    fn parse_logical_or(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_logical_and,
            &[(TokenKind::OrOr, BinaryOp::Or)],
        )
    }

    /// logical_and = equality ('&&' equality)*
    fn parse_logical_and(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_equality,
            &[(TokenKind::AndAnd, BinaryOp::And)],
        )
    }

    /// equality = comparison (('==' | '!=') comparison)*
    fn parse_equality(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_comparison,
            &[
                (TokenKind::EqEq, BinaryOp::Eq),
                (TokenKind::NotEq, BinaryOp::NotEq),
            ],
        )
    }

    /// comparison = term (('<' | '>' | '<=' | '>=') term)*
    fn parse_comparison(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_term,
            &[
                (TokenKind::Lt, BinaryOp::Lt),
                (TokenKind::Gt, BinaryOp::Gt),
                (TokenKind::LtEq, BinaryOp::LtEq),
                (TokenKind::GtEq, BinaryOp::GtEq),
            ],
        )
    }

    /// term = factor (('+' | '-') factor)*
    fn parse_term(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_factor,
            &[
                (TokenKind::Plus, BinaryOp::Add),
                (TokenKind::Minus, BinaryOp::Sub),
            ],
        )
    }

    /// factor = unary (('*' | '/') unary)*
    fn parse_factor(&mut self) -> Result<SpannedExpr, FormulaError> {
        self.parse_left_associative_binary(
            Self::parse_unary,
            &[
                (TokenKind::Star, BinaryOp::Mul),
                (TokenKind::Slash, BinaryOp::Div),
            ],
        )
    }

    /// unary = ('-' | '+' | '!')? primary
    fn parse_unary(&mut self) -> Result<SpannedExpr, FormulaError> {
        if self.peek() == TokenKind::Minus || self.peek() == TokenKind::Plus || self.peek() == TokenKind::Bang {
            let op = match self.peek() {
                TokenKind::Minus => UnaryOp::Neg,
                TokenKind::Plus => UnaryOp::Pos,
                TokenKind::Bang => UnaryOp::Not,
                _ => unreachable!(),
            };
            let op_span = token_span(self.tokens, self.pos);
            self.advance();
            let expr = self.parse_unary()?;
            let span = Span {
                start: op_span.start,
                end: expr.meta.span.end,
            };
            return Ok(SpannedExpr::new(
                Expr::UnaryExpr {
                    op,
                    expr: Box::new(expr),
                },
                span,
            ));
        }
        self.parse_postfix()
    }

    /// postfix = primary (('.' IDENT) | ('[' expression ']'))*
    fn parse_postfix(&mut self) -> Result<SpannedExpr, FormulaError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                TokenKind::Dot => {
                    self.advance();
                    let field_tok = self.advance();
                    if field_tok.kind != TokenKind::Identifier {
                        return Err(FormulaError::new(
                            ErrorKind::ParseError,
                            "E201",
                            "ต้องการชื่อคุณสมบัติ (identifier) หลังเครื่องหมายจุด",
                            Some(field_tok.span),
                        ));
                    }
                    let span = Span {
                        start: expr.meta.span.start,
                        end: field_tok.span.end,
                    };
                    expr = SpannedExpr::new(
                        Expr::PropertyAccess {
                            object: Box::new(expr),
                            property: field_tok.lexeme.clone(),
                        },
                        span,
                    );
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenKind::RBracket, "ต้องการ ']' ปิดท้าย index")?;
                    let span = Span {
                        start: expr.meta.span.start,
                        end: index.meta.span.end,
                    };
                    expr = SpannedExpr::new(
                        Expr::IndexAccess {
                            object: Box::new(expr),
                            index: Box::new(index),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// primary = NUMBER | STRING | TRUE | FALSE | IDENTIFIER | '(' expression ')' | function_call
    fn parse_primary(&mut self) -> Result<SpannedExpr, FormulaError> {
        let tok = self.advance();
        let span = tok.span;
        match &tok.kind {
            TokenKind::Number => {
                let n: f64 = tok.lexeme.parse().map_err(|_| {
                    FormulaError::new(
                        ErrorKind::ParseError,
                        "E202",
                        "ไม่สามารถแปลงตัวเลขได้",
                        Some(span),
                    )
                })?;
                Ok(SpannedExpr::new(
                    Expr::Literal(crate::value::Value::Number(n)),
                    span,
                ))
            }
            TokenKind::String => Ok(SpannedExpr::new(
                Expr::Literal(crate::value::Value::String(tok.lexeme.clone())),
                span,
            )),
            TokenKind::True => Ok(SpannedExpr::new(
                Expr::Literal(crate::value::Value::Bool(true)),
                span,
            )),
            TokenKind::False => Ok(SpannedExpr::new(
                Expr::Literal(crate::value::Value::Bool(false)),
                span,
            )),
            TokenKind::Null => Ok(SpannedExpr::new(
                Expr::Literal(crate::value::Value::Null),
                span,
            )),
            TokenKind::Identifier => {
                let name = tok.lexeme.clone();
                // Phase 9: Check for single-identifier lambda: x => x + 1
                if self.peek() == TokenKind::Arrow {
                    self.advance(); // consume '=>'
                    let body = self.parse_expression()?;
                    let span = Span {
                        start: span.start,
                        end: body.meta.span.end,
                    };
                    return Ok(SpannedExpr::new(
                        Expr::Lambda {
                            params: vec![name],
                            body: Box::new(body),
                        },
                        span,
                    ));
                }
                // ถ้าเจอ '(' ข้างหน้า คือ function call
                if self.peek() == TokenKind::LParen {
                    self.advance(); // กิน '('
                    let mut args = Vec::new();
                    if self.peek() != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek() == TokenKind::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen, "ต้องการ ')'")?;
                    Ok(SpannedExpr::new(Expr::FunctionCall { name, args }, span))
                } else {
                    // ตัวแปร (อาจเป็น dot notation แล้ว)
                    Ok(SpannedExpr::new(Expr::Variable(name), span))
                }
            }
            TokenKind::LParen => {
                // Phase 9: Check for parenthesized lambda: (x, y) => body
                let lparen_span = span;
                let mut params = Vec::new();
                let start_pos = self.pos;

                // Check if immediate ')' (empty grouping) or identifier (lambda params)
                if self.peek() == TokenKind::RParen {
                    // Empty () - not a lambda yet, check for =>
                    self.advance(); // consume ')'
                    if self.peek() == TokenKind::Arrow {
                        self.advance(); // consume '=>'
                        let body = self.parse_expression()?;
                        let span = Span {
                            start: lparen_span.start,
                            end: body.meta.span.end,
                        };
                        return Ok(SpannedExpr::new(
                            Expr::Lambda {
                                params: vec![],
                                body: Box::new(body),
                            },
                            span,
                        ));
                    }
                    // Rollback and let normal grouping handle it
                    self.pos = start_pos;
                } else if self.peek() == TokenKind::Identifier {
                    // Try to parse as lambda params: id (',' id)* ')'
                    let first_param = self.advance().lexeme.clone();
                    params.push(first_param);

                    let mut valid_params = true;
                    while self.peek() == TokenKind::Comma {
                        self.advance();
                        if self.peek() == TokenKind::Identifier {
                            params.push(self.advance().lexeme.clone());
                        } else {
                            valid_params = false;
                            break;
                        }
                    }

                    if valid_params && self.peek() == TokenKind::RParen {
                        self.advance(); // consume ')'
                        if self.peek() == TokenKind::Arrow {
                            self.advance(); // consume '=>'
                            let mut unique_params = std::collections::HashSet::new();
                            for param in &params {
                                if !unique_params.insert(param.clone()) {
                                    return Err(FormulaError::new(
                                        ErrorKind::ParseError,
                                        "E209",
                                        &format!("พารามิเตอร์ซ้ำกันใน lambda: '{}'", param),
                                        Some(lparen_span),
                                    ));
                                }
                            }
                            let body = self.parse_expression()?;
                            let span = Span {
                                start: lparen_span.start,
                                end: body.meta.span.end,
                            };
                            return Ok(SpannedExpr::new(
                                Expr::Lambda {
                                    params,
                                    body: Box::new(body),
                                },
                                span,
                            ));
                        }
                    }
                    // Rollback and let normal grouping handle it
                    self.pos = start_pos;
                }

                // Not a lambda - parse as grouping expression
                let inner = self.parse_expression()?;
                self.expect(TokenKind::RParen, "ต้องการ ')'")?;
                let span = Span {
                    start: lparen_span.start,
                    end: inner.meta.span.end,
                };
                Ok(SpannedExpr::new(Expr::Grouping(Box::new(inner)), span))
            }
            TokenKind::LBracket => {
                // parse array literal: '[' (expression (',' expression)*)? ']'
                let mut elements = Vec::new();
                if self.peek() != TokenKind::RBracket {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.peek() == TokenKind::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RBracket, "ต้องการ ']' ปิดท้าย array")?;
                Ok(SpannedExpr::new(Expr::ArrayLiteral(elements), span))
            }
            TokenKind::LBrace => {
                // parse map literal: '{' (identifier ':' expression (',' identifier ':' expression)*)? '}'
                let mut pairs = Vec::new();
                if self.peek() != TokenKind::RBrace {
                    loop {
                        // expect identifier as key
                        let key_tok = self.advance();
                        if key_tok.kind != TokenKind::Identifier {
                            return Err(FormulaError::new(
                                ErrorKind::ParseError,
                                "E201",
                                "key ใน map ต้องเป็น identifier",
                                Some(token_span(self.tokens, self.pos - 1)),
                            ));
                        }
                        let key = key_tok.lexeme.clone();
                        self.expect(TokenKind::Colon, "ต้องการ ':' หลัง key")?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        if self.peek() == TokenKind::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RBrace, "ต้องการ '}' ปิดท้าย map")?;
                Ok(SpannedExpr::new(Expr::MapLiteral(pairs), span))
            }
            _ => Err(FormulaError::new(
                ErrorKind::ParseError,
                "E203",
                &format!("ไม่คาดคิด token: {:?}", tok.kind),
                Some(span),
            )),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<SpannedExpr, FormulaError> {
    let mut parser = Parser::new(tokens);
    let first_expr = parser.parse_expression()?;

    // Phase 10: Check for sequence (semicolons)
    if parser.peek() == TokenKind::Semicolon {
        let mut exprs = vec![first_expr];
        while parser.peek() == TokenKind::Semicolon {
            parser.advance(); // consume ';'
            // Allow trailing semicolons
            if parser.peek() == TokenKind::Eof {
                break;
            }
            exprs.push(parser.parse_expression()?);
        }
        if parser.pos < tokens.len() - 1 {
            return Err(FormulaError::new(
                ErrorKind::ParseError,
                "E201",
                "มี tokens เหลือหลัง parse expression",
                Some(token_span(tokens, parser.pos)),
            ));
        }
        let span = Span {
            start: exprs.first().unwrap().meta.span.start,
            end: exprs.last().unwrap().meta.span.end,
        };
        return Ok(SpannedExpr::new(Expr::Sequence(exprs), span));
    }

    if parser.pos < tokens.len() - 1 {
        // -1 เพราะมี EOF
        return Err(FormulaError::new(
            ErrorKind::ParseError,
            "E201",
            "มี tokens เหลือหลัง parse expression",
            Some(token_span(tokens, parser.pos)),
        ));
    }
    Ok(first_expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_arithmetic() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(&tokens).unwrap();
        if let Expr::BinaryExpr { left: _, op, right } = &ast.expr {
            assert_eq!(*op, BinaryOp::Add);
            if let Expr::BinaryExpr {
                left: _,
                op: op2,
                right: _,
            } = &right.expr
            {
                assert_eq!(*op2, BinaryOp::Mul);
            } else {
                panic!("expected multiplication");
            }
        } else {
            panic!("expected binary add");
        }
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("len(\"abc\")").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "len");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("expected function call"),
        }
    }

    #[test]
    fn test_unclosed_parenthesis() {
        let tokens = tokenize("1 + (2 * 3").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201");
    }

    #[test]
    fn test_missing_comma_in_function() {
        let tokens = tokenize("if(true \"pass\" \"fail\")").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
    }

    #[test]
    fn test_invalid_number() {
        let tokens = tokenize("123abc").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201"); // มี tokens เหลือหลัง parse expression
    }

    // -- Tests for ArrayLiteral parsing (added in Phase 6.1) --

    #[test]
    fn test_empty_array_literal() {
        let tokens = tokenize("[]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => assert_eq!(elems.len(), 0),
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_single_element_array() {
        let tokens = tokenize("[42]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => {
                assert_eq!(elems.len(), 1);
                match &elems[0].expr {
                    Expr::Literal(crate::value::Value::Number(n)) => assert_eq!(*n, 42.0),
                    _ => panic!("expected number literal"),
                }
            }
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_multi_element_array() {
        let tokens = tokenize("[1, 2, 3]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => assert_eq!(elems.len(), 3),
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_array_with_string_elements() {
        let tokens = tokenize("[\"a\", \"b\"]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => {
                assert_eq!(elems.len(), 2);
                match &elems[0].expr {
                    Expr::Literal(crate::value::Value::String(s)) => assert_eq!(s, "a"),
                    _ => panic!("expected string literal"),
                }
            }
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_nested_array_literal() {
        let tokens = tokenize("[[1, 2], [3, 4]]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => {
                assert_eq!(elems.len(), 2);
                match &elems[0].expr {
                    Expr::ArrayLiteral(inner) => assert_eq!(inner.len(), 2),
                    _ => panic!("expected nested ArrayLiteral"),
                }
            }
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_unclosed_array_literal() {
        let tokens = tokenize("[1, 2").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201");
    }

    #[test]
    fn test_array_with_expression_elements() {
        let tokens = tokenize("[1 + 2, 3 * 4]").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::ArrayLiteral(elems) => {
                assert_eq!(elems.len(), 2);
                // First element should be a BinaryExpr (Add)
                match &elems[0].expr {
                    Expr::BinaryExpr { op, .. } => assert_eq!(*op, BinaryOp::Add),
                    _ => panic!("expected BinaryExpr"),
                }
            }
            _ => panic!("expected ArrayLiteral"),
        }
    }

    #[test]
    fn test_array_as_function_arg() {
        let tokens = tokenize("sum([1, 2, 3])").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "sum");
                assert_eq!(args.len(), 1);
                match &args[0].expr {
                    Expr::ArrayLiteral(elems) => assert_eq!(elems.len(), 3),
                    _ => panic!("expected ArrayLiteral as arg"),
                }
            }
            _ => panic!("expected FunctionCall"),
        }
    }

    // -- Tests for MapLiteral parsing (added in Phase 6.2) --

    #[test]
    fn test_empty_map_literal() {
        let tokens = tokenize("{}").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::MapLiteral(pairs) => assert_eq!(pairs.len(), 0),
            _ => panic!("expected MapLiteral"),
        }
    }

    #[test]
    fn test_single_key_value_map() {
        let tokens = tokenize("{key: 42}").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::MapLiteral(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "key");
                match &pairs[0].1.expr {
                    Expr::Literal(crate::value::Value::Number(n)) => assert_eq!(*n, 42.0),
                    _ => panic!("expected number literal"),
                }
            }
            _ => panic!("expected MapLiteral"),
        }
    }

    #[test]
    fn test_multi_key_value_map() {
        let tokens = tokenize("{a: 1, b: \"hello\", c: true}").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::MapLiteral(pairs) => {
                assert_eq!(pairs.len(), 3);
                assert_eq!(pairs[0].0, "a");
                assert_eq!(pairs[1].0, "b");
                assert_eq!(pairs[2].0, "c");
            }
            _ => panic!("expected MapLiteral"),
        }
    }

    #[test]
    fn test_map_with_expression_values() {
        let tokens = tokenize("{sum: 1 + 2}").unwrap();
        let ast = parse(&tokens).unwrap();
        match ast.expr {
            Expr::MapLiteral(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "sum");
                match &pairs[0].1.expr {
                    Expr::BinaryExpr { op, .. } => assert_eq!(*op, BinaryOp::Add),
                    _ => panic!("expected BinaryExpr"),
                }
            }
            _ => panic!("expected MapLiteral"),
        }
    }

    #[test]
    fn test_unclosed_map_literal() {
        let tokens = tokenize("{key: value").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201");
    }

    #[test]
    fn test_map_with_non_identifier_key() {
        let tokens = tokenize("{123: value}").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201");
    }

    #[test]
    fn test_map_missing_colon() {
        let tokens = tokenize("{key value}").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert_eq!(err.code, "E201");
    }
}
