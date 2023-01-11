use crate::{
    ast,
    ast::Expr,
    lexer::Lexer,
    token::{Kind, Token},
};
use alc_command_option::CommandOptions;
use alc_diagnostic::{Diagnostic, FileId, Files, Label, Result, Span, Spanned};
use log::debug;
use std::{env, iter::FusedIterator, mem};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Restriction {
    None,
    NoStructLiteral,
}

#[derive(Debug)]
pub struct Parser<'a> {
    file_id: FileId,
    current: Option<Spanned<Token>>,
    last_span: Span,
    tokens: Lexer<'a>,
    #[allow(unused)]
    command_options: &'a CommandOptions,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Spanned<ast::Item>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            None
        } else {
            Some(self.next_item())
        }
    }
}

impl<'a> FusedIterator for Parser<'a> {}

macro_rules! register_unops {
    ( $( ( $kind:ident , $op:ident ) ),* ) => {
        fn next_is_unop(&self) -> bool {
            $(
                self.next_is(Kind::$kind) ||
            )*
            false
        }

        fn next_unop(&mut self) -> Result<Spanned<ast::UnopKind>> {
            $(
                if self.next_is(Kind::$kind) {
                    let span = self.eat(Kind::$kind)?.span();
                    return Ok(span.span(ast::UnopKind::$op))
                }
            )*
            Err(Box::from(self.multi_expectation_diagnostic(vec![
                $( Kind::$kind, )*
            ])))
        }
    }
}

macro_rules! register_binops {
    ( $( ( $kind:ident , $op:ident, $prec:expr ) ),* ) => {
        fn next_binop(&mut self) -> Result<Spanned<ast::BinopKind>> {
            $(
                if self.next_is(Kind::$kind) {
                    let span = self.eat(Kind::$kind)?.span();
                    return Ok(span.span(ast::BinopKind::$op))
                }
            )*
            Err(Box::from(self.multi_expectation_diagnostic(vec![
                $( Kind::$kind, )*
            ])))
        }

        pub fn _precedence_of(kind: ast::BinopKind) -> u32 {
            match kind {
                $( ast::BinopKind::$op => $prec, )*
            }
        }

        fn next_binop_precendence(&self) -> Option<u32> {
            $(
                if self.next_is(Kind::$kind) {
                    return Some($prec);
                }
            )*
            None
        }
    }
}

impl<'a> Parser<'a> {
    register_unops![(Not, Not)];

    register_binops![
        (Mul, Mul, 70),
        (Div, Div, 70),
        (Plus, Plus, 60),
        (Minus, Minus, 60),
        (LShift, LShift, 50),
        (RShift, RShift, 50),
        (And, And, 40),
        (Xor, Xor, 30),
        (Or, Or, 20),
        (EqEq, Eq, 10),
        (Neq, Neq, 10),
        (LAngle, Less, 10),
        (RAngle, Greater, 10),
        (Leq, Leq, 10),
        (Geq, Geq, 10)
    ];

    pub fn new(command_options: &'a CommandOptions, files: &'a Files, file_id: FileId) -> Result<Parser<'a>> {
        let mut lexer = Lexer::new(files, file_id);
        debug!("{:#?}", lexer);
        if let Some(first) = lexer.next() {
            Ok(Parser {
                file_id,
                current: Some(first?),
                last_span: Span::dummy(),
                tokens: lexer,
                command_options,
            })
        } else {
            Err(Box::from(Diagnostic::new_bug(
                "cannot construct parser from empty token stream",
                Label::new(file_id, Span::dummy(), "this file appears to be empty"),
            )))
        }
    }

    fn multi_expectation_diagnostic(&self, expected: Vec<Kind>) -> Diagnostic {
        let mut label = String::from("expected ");
        for (i, kind) in expected.iter().enumerate() {
            if i + 1 < expected.len() {
                label.push_str(&format!("'{:?}', ", kind))
            } else {
                label.push_str(&format!("or '{:?}'", kind))
            }
        }
        let (span, msg) = if let Some(ref current) = self.current {
            label.push_str(&format!(", but got '{:?}'", current.kind()));
            (current.span(), "token type mismatch")
        } else {
            (self.last_span.clip(), "unexpected end of input")
        };
        Diagnostic::new_error(msg, Label::new(self.file_id, span, label.as_str()))
    }

    fn eat(&mut self, kind: Kind) -> Result<Spanned<Token>> {
        match &self.current {
            Some(ref current) if current.is(kind) => {
                let mut token = self.tokens.next().map_or(Ok(None), |token| token.map(Some))?;
                self.last_span = current.span();
                mem::swap(&mut self.current, &mut token);
                Ok(token.unwrap())
            }
            Some(ref current) => Err(Box::from(Diagnostic::new_error(
                "token type mismatch",
                Label::new(
                    self.file_id,
                    current.span(),
                    &format!("expected '{:?}', but got '{:?}'", kind, current.kind()),
                ),
            ))),
            _ => Err(Box::from(Diagnostic::new_error(
                "unexpected end of input",
                Label::new(
                    self.file_id,
                    self.last_span.clip(),
                    &format!("expected '{:?}', but reached end of input", kind),
                ),
            ))),
        }
    }

    #[inline]
    fn next_is(&self, kind: Kind) -> bool {
        self.current.as_ref().map_or(false, |current| current.is(kind))
    }

    fn next_comma_group<T>(
        &mut self,
        head: Kind,
        tail: Kind,
        elem: impl Fn(&mut Parser) -> Result<T>,
    ) -> Result<Spanned<Vec<T>>> {
        let span = self.eat(head)?.span();
        let mut elems = vec![];
        while !self.next_is(tail) {
            elems.push(elem(self)?);
            if self.next_is(Kind::Comma) {
                self.eat(Kind::Comma)?;
            } else {
                break;
            }
        }
        let span = span.merge(self.eat(tail)?.span());
        Ok(span.span(elems))
    }

    fn next_array_elements(&mut self) -> Result<Spanned<Vec<Spanned<Expr>>>> {
        let span = self.eat(Kind::LSquare)?.span();
        let mut elems = vec![];

        if self.next_is(Kind::RSquare) {
            self.eat(Kind::RSquare)?;
            return Ok(span.span(elems));
        }
        let first_val = self.next_expr()?;
        elems.push(first_val.clone());
        if self.next_is(Kind::Semi) {
            self.eat(Kind::Semi)?;
            let count = self.next_number_literal()?;
            for _ in 1..count.into_raw() {
                elems.push(first_val.clone());
            }
            return Ok(span.merge(self.eat(Kind::RSquare)?.span()).span(elems));
        }
        self.eat(Kind::Comma)?;
        while !self.next_is(Kind::RSquare) {
            elems.push(self.next_expr()?);
            if self.next_is(Kind::Comma) {
                self.eat(Kind::Comma)?;
            } else {
                break;
            }
        }
        let span = span.merge(self.eat(Kind::RSquare)?.span());
        Ok(span.span(elems))
    }

    fn next_ident(&mut self) -> Result<Spanned<ast::Ident>> {
        let token = self.eat(Kind::Ident)?;
        Ok(token.span().span(token.value().unwrap().to_string()))
    }

    fn unpack_number_literal(&self, data: &str, span: Span) -> Result<i64> {
        data.parse::<i64>().map_err(|err| {
            Box::from(Diagnostic::new_bug(
                "failed to parse u64 literal",
                Label::new(self.file_id, span, err.to_string()),
            ))
        })
    }

    fn next_number_literal(&mut self) -> Result<Spanned<i64>> {
        let token = self.eat(Kind::NumberLiteral)?;
        Ok(token
            .span()
            .span(self.unpack_number_literal(token.value().unwrap(), token.span())?))
    }

    fn next_string_literal(&mut self) -> Result<Spanned<String>> {
        let token = self.eat(Kind::StringLiteral)?;
        Ok(token.span().span(token.value().unwrap().to_string()))
    }

    fn next_ty(&mut self) -> Result<Spanned<ast::Ty>> {
        if self.next_is(Kind::I8Ty) {
            Ok(self.eat(Kind::I8Ty)?.span().span(ast::Ty::I8))
        } else if self.next_is(Kind::I16Ty) {
            Ok(self.eat(Kind::I16Ty)?.span().span(ast::Ty::I16))
        } else if self.next_is(Kind::I32Ty) {
            Ok(self.eat(Kind::I32Ty)?.span().span(ast::Ty::I32))
        } else if self.next_is(Kind::I64Ty) {
            Ok(self.eat(Kind::I64Ty)?.span().span(ast::Ty::I64))
        } else if self.next_is(Kind::StringTy) {
            Ok(self.eat(Kind::StringTy)?.span().span(ast::Ty::String))
        } else if self.next_is(Kind::LSquare) {
            let span = self.eat(Kind::LSquare)?.span();
            let ty = self.next_ty()?;
            self.eat(Kind::Semi)?;
            let size = self.next_number_literal()?;
            let span = span.merge(self.eat(Kind::RSquare)?.span());
            Ok(span.span(ast::Ty::Array(Box::from(ty.into_raw()), size.into_raw() as i32)))
        } else {
            let ident = self.next_ident()?;
            Ok(ident.span().span(ast::Ty::TyName(ident.into_raw())))
        }
    }

    fn next_ident_expr(&mut self, res: Restriction) -> Result<Spanned<ast::Expr>> {
        let ident = self.next_ident()?;
        if self.next_is(Kind::LParen) {
            let args = self.next_comma_group(Kind::LParen, Kind::RParen, |this| this.next_expr())?;
            let span = ident.span().merge(args.span());
            Ok(span.span(ast::Expr::Call {
                target: ident,
                args: args.into_raw(),
            }))
        } else if self.next_is(Kind::Separator) {
            self.eat(Kind::Separator)?;
            let discriminant = self.next_ident()?;
            self.eat(Kind::LParen)?;
            let body = self.next_expr()?;
            let span = ident.span().merge(self.eat(Kind::RParen)?.span());
            Ok(span.span(ast::Expr::Variant {
                enum_name: ident,
                discriminant,
                body: body.boxed(),
            }))
        } else if self.next_is(Kind::LCurl) && res != Restriction::NoStructLiteral {
            let fields = self.next_comma_group(Kind::LCurl, Kind::RCurl, |this| {
                let field_name = this.next_ident()?;
                this.eat(Kind::Colon)?;
                Ok((field_name, this.next_expr()?))
            })?;
            let span = ident.span().merge(fields.span());
            Ok(span.span(ast::Expr::Record {
                struct_name: ident,
                fields: fields.into_raw(),
            }))
        } else if self.next_is(Kind::Dot) {
            let mut vars = vec![ident];
            while self.next_is(Kind::Dot) {
                self.eat(Kind::Dot)?;
                let field = self.next_ident()?;
                vars.push(field);
            }
            let span = vars.first().unwrap().span().merge(vars.last().unwrap().span());
            Ok(span.span(ast::Expr::Var(vars)))
        } else {
            Ok(ident
                .span()
                .span(ast::Expr::Var(vec![ident.span().span(ident.into_raw())])))
        }
    }

    fn next_primary(&mut self, res: Restriction) -> Result<Spanned<ast::Expr>> {
        if self.next_is(Kind::Env) {
            let span = self.eat(Kind::Env)?.span();
            self.eat(Kind::LParen)?;
            let env = self.eat(Kind::StringLiteral)?;
            let span = span.merge(self.eat(Kind::RParen)?.span());
            let expanded = env::var(env.value().unwrap()).map_err(|_| {
                Diagnostic::new_error(
                    "failed to read environment variable",
                    Label::new(
                        self.file_id,
                        span,
                        &format!("error reading \"{}\"", env.value().unwrap()),
                    ),
                )
            })?;
            Ok(span.span(ast::Expr::StringLiteral(expanded)))
        } else if self.next_is(Kind::NumberLiteral) {
            let literal = self.next_number_literal()?;
            Ok(literal.span().span(ast::Expr::NumberLiteral(literal.into_raw())))
        } else if self.next_is(Kind::StringLiteral) {
            let literal = self.next_string_literal()?;
            Ok(literal.span().span(ast::Expr::StringLiteral(literal.into_raw())))
        } else if self.next_is(Kind::LParen) {
            self.eat(Kind::LParen)?;
            let expr = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(expr)
        } else if self.next_is(Kind::LSquare) {
            let elements = self.next_array_elements()?;
            Ok(elements.span().span(ast::Expr::ArrayLiteral(elements.into_raw())))
        } else if self.next_is(Kind::Socket) {
            let span = self.eat(Kind::Socket)?.span();
            self.eat(Kind::LParen)?;
            let domain = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let ty = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let protocol = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Socket {
                domain: domain.boxed(),
                ty: ty.boxed(),
                protocol: protocol.boxed(),
            }))
        } else if self.next_is(Kind::Bind) {
            let span = self.eat(Kind::Bind)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let address = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let address_length = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Bind {
                socket_file_descriptor: socket_file_descriptor.boxed(),
                address: address.boxed(),
                address_length: address_length.boxed(),
            }))
        } else if self.next_is(Kind::Listen) {
            let span = self.eat(Kind::Listen)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let backlog = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Listen {
                socket_file_descriptor: socket_file_descriptor.boxed(),
                backlog: backlog.boxed(),
            }))
        } else if self.next_is(Kind::Accept) {
            let span = self.eat(Kind::Accept)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Accept {
                socket_file_descriptor: socket_file_descriptor.boxed(),
            }))
        } else if self.next_is(Kind::Recv) {
            let span = self.eat(Kind::Recv)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let buffer = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let buffer_length = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let flags = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Recv {
                socket_file_descriptor: socket_file_descriptor.boxed(),
                buffer: buffer.boxed(),
                buffer_length: buffer_length.boxed(),
                flags: flags.boxed(),
            }))
        } else if self.next_is(Kind::Send) {
            let span = self.eat(Kind::Send)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let buffer = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let buffer_length = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let content = self.next_expr()?;
            self.eat(Kind::Comma)?;
            let flags = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Send {
                socket_file_descriptor: socket_file_descriptor.boxed(),
                buffer: buffer.boxed(),
                buffer_length: buffer_length.boxed(),
                content: content.boxed(),
                flags: flags.boxed(),
            }))
        } else if self.next_is(Kind::Close) {
            let span = self.eat(Kind::Close)?.span();
            self.eat(Kind::LParen)?;
            let socket_file_descriptor = self.next_expr()?;
            self.eat(Kind::RParen)?;
            Ok(span.span(ast::Expr::Close {
                socket_file_descriptor: socket_file_descriptor.boxed(),
            }))
        } else if self.next_is(Kind::Ident) {
            self.next_ident_expr(res)
        } else if self.next_is_unop() {
            let kind = self.next_unop()?;
            let operand = self.next_primary(res)?;
            let span = kind.span().merge(operand.span());
            Ok(span.span(ast::Expr::Unop {
                kind,
                operand: operand.boxed(),
            }))
        } else {
            Err(Box::from(self.multi_expectation_diagnostic(vec![
                Kind::U64Literal,
                Kind::Ident,
                Kind::LParen,
            ])))
        }
    }

    fn next_binary_expr(
        &mut self,
        expr_prec: u32,
        mut left: Spanned<ast::Expr>,
        res: Restriction,
    ) -> Result<Spanned<ast::Expr>> {
        loop {
            if let Some(tok_prec) = self.next_binop_precendence() {
                if tok_prec < expr_prec {
                    return Ok(left);
                }
                let kind = self.next_binop()?;
                let mut right = self.next_primary(res)?;
                if let Some(next_prec) = self.next_binop_precendence() {
                    if tok_prec < next_prec {
                        right = self.next_binary_expr(tok_prec + 1, right, res)?;
                    }
                }
                left = left.span().merge(right.span()).span(ast::Expr::Binop {
                    kind,
                    left: left.boxed(),
                    right: right.boxed(),
                });
            } else {
                return Ok(left);
            }
        }
    }

    fn next_expr_res(&mut self, res: Restriction) -> Result<Spanned<ast::Expr>> {
        let left = self.next_primary(res)?;
        self.next_binary_expr(0, left, res)
    }

    fn next_expr(&mut self) -> Result<Spanned<ast::Expr>> {
        self.next_expr_res(Restriction::None)
    }

    fn next_pattern(&mut self) -> Result<Spanned<ast::Pattern>> {
        if self.next_is(Kind::NumberLiteral) {
            let literal = self.next_number_literal()?;
            Ok(literal
                .span()
                .span(ast::Pattern::NumberLiteral(literal.into_raw())))
        } else if self.next_is(Kind::StringLiteral) {
            let literal = self.next_string_literal()?;
            Ok(literal
                .span()
                .span(ast::Pattern::StringLiteral(literal.into_raw())))
        } else if self.next_is(Kind::Ident) {
            let ident = self.next_ident()?;
            if self.next_is(Kind::Separator) {
                self.eat(Kind::Separator)?;
                let discriminant = self.next_ident()?;
                self.eat(Kind::LParen)?;
                let bound = self.next_ident()?;
                let span = ident.span().merge(self.eat(Kind::RParen)?.span());
                Ok(span.span(ast::Pattern::Variant {
                    enum_name: ident,
                    discriminant,
                    bound,
                }))
            } else if self.next_is(Kind::LCurl) {
                let fields = self.next_comma_group(Kind::LCurl, Kind::RCurl, |this| {
                    let fieldname = this.next_ident()?;
                    this.eat(Kind::Colon)?;
                    Ok((fieldname, this.next_ident()?))
                })?;
                let span = ident.span().merge(fields.span());
                Ok(span.span(ast::Pattern::Record {
                    struct_name: ident,
                    fields: fields.into_raw(),
                }))
            } else {
                Ok(ident.span().span(ast::Pattern::Ident(ident.into_raw())))
            }
        } else {
            Err(Box::from(
                self.multi_expectation_diagnostic(vec![Kind::U64Literal, Kind::Ident])
                    .with_notes(vec!["this is in order to form a valid pattern".to_owned()]),
            ))
        }
    }

    fn next_term(&mut self) -> Result<Spanned<ast::Term>> {
        if self.next_is(Kind::Let) {
            let span = self.eat(Kind::Let)?.span();
            let binder = self.next_ident()?;
            let annotation = self.next_ty_annotation()?;
            self.eat(Kind::Eq)?;
            let expr = self.next_expr()?;
            let body = self.next_term()?;
            Ok(span.merge(body.span()).span(ast::Term::Let {
                binder,
                annotation,
                expr,
                body: Box::new(body),
            }))
        } else if self.next_is(Kind::Println) {
            let span = self.eat(Kind::Println)?.span();
            self.eat(Kind::LParen)?;
            let expr = self.next_expr()?;
            self.eat(Kind::RParen)?;
            let body = self.next_term()?;
            Ok(span.merge(body.span()).span(ast::Term::Println {
                expr,
                body: Box::new(body),
            }))
        } else if self.next_is(Kind::Match) {
            self.next_match_term()
        } else if self.next_is(Kind::If) {
            self.next_if_term()
        } else if self.next_is(Kind::LCurl) {
            self.next_block()
        } else {
            let expr = self.next_expr()?;
            Ok(expr.span().span(ast::Term::Return(expr.into_raw())))
        }
    }

    fn next_block(&mut self) -> Result<Spanned<ast::Term>> {
        let span = self.eat(Kind::LCurl)?.span();
        let term = self.next_term()?;
        let span = span.merge(self.eat(Kind::RCurl)?.span());
        Ok(term.respan(span))
    }

    fn next_match_arm_body(&mut self) -> Result<Spanned<ast::Term>> {
        if self.next_is(Kind::Match) {
            let term = self.next_match_term()?;
            self.eat(Kind::Comma)?;
            Ok(term)
        } else if self.next_is(Kind::If) {
            let term = self.next_if_term()?;
            self.eat(Kind::Comma)?;
            Ok(term)
        } else if self.next_is(Kind::LCurl) {
            self.next_block()
        } else {
            let expr = self.next_expr()?;
            self.eat(Kind::Comma)?;
            Ok(expr.span().span(ast::Term::Return(expr.into_raw())))
        }
    }

    fn next_match_term(&mut self) -> Result<Spanned<ast::Term>> {
        let span = self.eat(Kind::Match)?.span();
        let source = self.next_expr_res(Restriction::NoStructLiteral)?;
        self.eat(Kind::LCurl)?;
        let mut arms = vec![];
        while !self.next_is(Kind::RCurl) {
            let pattern = self.next_pattern()?;
            self.eat(Kind::MatchArrow)?;
            arms.push((pattern, Box::new(self.next_match_arm_body()?)));
        }
        let span = span.merge(self.eat(Kind::RCurl)?.span());
        Ok(span.span(ast::Term::Match { source, arms }))
    }

    fn next_if_term(&mut self) -> Result<Spanned<ast::Term>> {
        let span = self.eat(Kind::If)?.span();
        let source = self.next_expr_res(Restriction::NoStructLiteral)?;
        let then = Box::new(self.next_block()?);
        self.eat(Kind::Else)?;
        let otherwise = Box::new(if self.next_is(Kind::If) {
            self.next_if_term()?
        } else {
            self.next_block()?
        });
        let span = span.merge(otherwise.span());
        Ok(span.span(ast::Term::If {
            source,
            then,
            otherwise,
        }))
    }

    fn next_binding(&mut self) -> Result<Spanned<ast::Binding>> {
        let binder = self.next_ident()?;
        self.eat(Kind::Colon)?;
        let ty = self.next_ty()?;
        Ok(binder.span().merge(ty.span()).span(ast::Binding { binder, ty }))
    }

    fn next_ty_annotation(&mut self) -> Result<Option<Spanned<ast::Ty>>> {
        Ok(if self.next_is(Kind::Colon) {
            self.eat(Kind::Colon)?;
            Some(self.next_ty()?)
        } else {
            None
        })
    }

    fn next_fn_item(&mut self) -> Result<Spanned<ast::Item>> {
        let span = self.eat(Kind::Func)?.span();
        let name = self.next_ident()?;
        let params = self
            .next_comma_group(Kind::LParen, Kind::RParen, |this| this.next_binding())?
            .into_raw();
        let return_ty = self.next_ty()?;
        let body = self.next_term()?;
        let span = span.merge(body.span());
        Ok(span.span(ast::Item::Fn(Box::new(ast::FnDecl {
            name,
            params,
            return_ty,
            body,
        }))))
    }

    fn next_enum_item(&mut self) -> Result<Spanned<ast::Item>> {
        let span = self.eat(Kind::Enum)?.span();
        let name = self.next_ident()?;
        let variants = self.next_comma_group(Kind::LCurl, Kind::RCurl, |this| {
            let discriminant = this.next_ident()?;
            this.eat(Kind::LParen)?;
            let ty = this.next_ty()?;
            let span = discriminant.span().merge(this.eat(Kind::RParen)?.span());
            Ok(span.span(ast::Binding {
                binder: discriminant,
                ty,
            }))
        })?;
        let span = span.merge(variants.span());
        Ok(span.span(ast::Item::Enum(ast::Enum {
            name,
            variants: variants.into_raw(),
        })))
    }

    fn next_struct_item(&mut self) -> Result<Spanned<ast::Item>> {
        let span = self.eat(Kind::Struct)?.span();
        let name = self.next_ident()?;
        let fields = self.next_comma_group(Kind::LCurl, Kind::RCurl, |this| this.next_binding())?;
        let span = span.merge(fields.span());
        Ok(span.span(ast::Item::Struct(ast::Struct {
            name,
            fields: fields.into_raw(),
        })))
    }

    fn next_item(&mut self) -> Result<Spanned<ast::Item>> {
        if self.next_is(Kind::Func) {
            self.next_fn_item()
        } else if self.next_is(Kind::Struct) {
            self.next_struct_item()
        } else if self.next_is(Kind::Enum) {
            self.next_enum_item()
        } else {
            Err(Box::from(self.multi_expectation_diagnostic(vec![
                Kind::Func,
                Kind::Struct,
                Kind::Enum,
            ])))
        }
    }
}
