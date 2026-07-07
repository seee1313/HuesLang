use crate::ast::{AST, Expr, Op, Type};
use crate::token::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<AST>, String> {
        let mut program = Vec::new();

        while self.tokens.peek().is_some() {
            let stmt = self.parse_statement()?;
            program.push(stmt);
        }
        Ok(program)
    }

    pub fn parse_let(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Let)) {
            return Err("Syntax error".into());
        }
        self.tokens.next();

        let name = match self.tokens.next() {
            Some(Token::Ident(n)) => n,
            None => return Err("Syntax Err".into()),
            _ => return Err("Syntax Error".into()),
        };
        let ty = if matches!(self.tokens.peek(), Some(Token::Colon)) {
            self.tokens.next();
            Some(self.parse_type()?)
        } else {
            None
        };

        if !matches!(self.tokens.next(), Some(Token::Assign)) {
            return Err("Expected '='".into());
        }

        let value = self.parse_expr()?;
        self.parse_semicolon()?;
        Ok(AST::VarDecl { name, value, ty })
    }

    pub fn parse_semicolon(&mut self) -> Result<(), String> {
        if !matches!(self.tokens.next(), Some(Token::Semicolon)) {
            return Err("Expected ';'".into());
        }
        Ok(())
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.tokens.next() {
            Some(Token::IntLit(n)) => Ok(Expr::Number(n)),
            Some(Token::FloatLit(f)) => Ok(Expr::FloatLit(f)),
            Some(Token::StringLit(s)) => Ok(Expr::StringLit(s)),
            Some(Token::True) => Ok(Expr::Boolean(true)),
            Some(Token::False) => Ok(Expr::Boolean(false)),
            Some(Token::Ident(name)) => {
                if matches!(self.tokens.peek(), Some(Token::LParen)) {
                    self.tokens.next();
                    let mut args = Vec::new();
                    if matches!(self.tokens.peek(), Some(Token::RParen)) {
                        self.tokens.next();
                        return Ok(Expr::FuncCall { name, args });
                    }
                    loop {
                        args.push(self.parse_expr()?);
                        match self.tokens.next() {
                            Some(Token::Comma) => continue,
                            Some(Token::RParen) => break,
                            None => return Err("Unexpected EOF".into()),
                            _ => return Err("Unknown Token".into()),
                        }
                    }
                    Ok(Expr::FuncCall { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            None => return Err("Unexpected EOF".into()),
            _ => return Err("Unexpected Token".into()),
        }
    }

    fn get_precedence(token: &Token) -> Option<(Op, u8)> {
        match token {
            Token::Star => Some((Op::Mul, 20)),
            Token::Slash => Some((Op::Div, 20)),
            Token::Percent => Some((Op::Percent, 20)),

            Token::Plus => Some((Op::Add, 10)),
            Token::Minus => Some((Op::Sub, 10)),

            Token::Eq => Some((Op::Eq, 5)),
            Token::NotEq => Some((Op::Neq, 5)),
            Token::Less => Some((Op::Lt, 5)),
            Token::Greater => Some((Op::Gt, 5)),
            Token::LessEq => Some((Op::Le, 5)),
            Token::GreaterEq => Some((Op::Ge, 5)),

            _ => None,
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_expr_pratt(0)
    }

    fn parse_expr_pratt(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_atom()?;

        while let Some(token) = self.tokens.peek() {
            if let Some((op, prec)) = Self::get_precedence(token) {
                if prec < min_prec {
                    break;
                }

                self.tokens.next();

                let right = self.parse_expr_pratt(prec + 1)?;

                left = Expr::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_ret(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Ret)) {
            return Err("Expected Token Ret".into());
        }
        self.tokens.next();
        let value = self.parse_expr()?;

        self.parse_semicolon()?;

        Ok(AST::Return { value })
    }
    fn parse_loop(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Loop)) {
            return Err("Expected 'loop'".into());
        }
        self.tokens.next();
        let body = self.parse_block()?;
        Ok(AST::Loop { body })
    }

    fn parse_while(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::While)) {
            return Err("Unexpected Identifier".into());
        }
        self.tokens.next();

        let condition = self.parse_expr()?;

        let body = self.parse_block()?;

        Ok(AST::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::For)) {
            return Err("Unexpected Identifier".into());
        }
        self.tokens.next();

        let var = match self.tokens.next() {
            Some(Token::Ident(n)) => n,
            _ => return Err("Syntax Err,,".into()),
        };

        if matches!(self.tokens.peek(), Some(Token::In)) {
            self.tokens.next();
        } // In необязателен
        let start = self.parse_expr()?;
        if !matches!(self.tokens.peek(), Some(Token::DotDot)) {
            return Err("Syntax Err".into());
        }
        let end = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(AST::For {
            var,
            start,
            end,
            body,
        })
    }
    fn parse_break(&mut self) -> Result<AST, String> {
        self.tokens.next();
        self.parse_semicolon()?;
        Ok(AST::Break)
    }

    fn parse_statement(&mut self) -> Result<AST, String> {
        match self.tokens.peek() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::Ret) => self.parse_ret(),
            Some(Token::Hue) => self.parse_hue(),
            Some(Token::Loop) => self.parse_loop(),
            Some(Token::Asm) => self.parse_asm(),
            Some(Token::Extern) => self.parse_extern_fn(),
            Some(Token::If) => self.parse_if_else(),
            Some(Token::While) => self.parse_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Break) => self.parse_break(),
            None => {
                return Err("Unexpected statement".into());
            }
            _ => {
                let expr = self.parse_expr()?;
                self.parse_semicolon()?;
                Ok(AST::ExprStmt(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Vec<AST>, String> {
        if !matches!(self.tokens.peek(), Some(Token::LBrace)) {
            return Err("Syntax Error".into());
        }
        self.tokens.next();

        let mut body = Vec::new();

        while !matches!(self.tokens.peek(), Some(Token::RBrace)) {
            if self.tokens.peek().is_none() {
                return Err("Unexpected EOF Expected '}'".into());
            }
            body.push(self.parse_statement()?);
        }
        self.tokens.next();
        Ok(body)
    }

    fn parse_hue(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Hue)) {
            return Err("Syntax Error".into());
        }
        self.tokens.next();

        let name = match self.tokens.next() {
            Some(Token::Ident(n)) => n,
            None => return Err("Syntax Error".into()),
            _ => return Err("Unexpected Token".into()),
        };

        if !matches!(self.tokens.next(), Some(Token::LParen)) {
            return Err("Expected '('".into());
        }

        let mut args = Vec::new();

        while !matches!(self.tokens.peek(), Some(Token::RParen)) {
            let arg_name = match self.tokens.next() {
                Some(Token::Ident(n)) => n,
                None => return Err("Unexpected EOF in function parameters".into()),
                _ => return Err("Unexpected Token compiler internall err".into()),
            };

            if !matches!(self.tokens.next(), Some(Token::Colon)) {
                return Err("Expected Colon".into());
            }

            let arg_type = self.parse_type()?;
            args.push((arg_name, arg_type));

            match self.tokens.peek() {
                Some(Token::Comma) => {
                    self.tokens.next();
                }
                Some(Token::RParen) => continue,
                _ => return Err("Expected arg or ','".into()),
            }
        }
        self.tokens.next();

        let return_type = if matches!(self.tokens.peek(), Some(Token::Arrow)) {
            self.tokens.next();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(AST::FuncDecl {
            name,
            args,
            body,
            return_type,
        })
    }

    fn parse_if_else(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::If)) {
            return Err("Unexpected Expression".into());
        }
        self.tokens.next();

        let condition = self.parse_expr()?;

        let then_branch = self.parse_block()?;

        let else_branch = if matches!(self.tokens.peek(), Some(Token::Else)) {
            self.tokens.next();

            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(AST::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_extern_fn(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Extern)) {
            return Err("Syntax Error".into());
        }
        self.tokens.next();

        if !matches!(self.tokens.peek(), Some(Token::Hue)) {
            return Err("Expected 'hue' after 'extern'".into());
        }
        self.tokens.next();

        let name = match self.tokens.next() {
            Some(Token::Ident(n)) => n,
            None => return Err("Syntax Error".into()),
            _ => return Err("Unexpected Token".into()),
        };

        if !matches!(self.tokens.next(), Some(Token::LParen)) {
            return Err("Expected '('".into());
        }

        let mut args = Vec::new();

        while !matches!(self.tokens.peek(), Some(Token::RParen)) {
            let arg_name = match self.tokens.next() {
                Some(Token::Ident(n)) => n,
                None => return Err("Unexpected EOF in function parameters".into()),
                _ => return Err("Unexpected Token compiler internall err".into()),
            };

            if !matches!(self.tokens.next(), Some(Token::Colon)) {
                return Err("Expected Colon".into());
            }

            let arg_type = self.parse_type()?;
            args.push((arg_name, arg_type));

            if matches!(self.tokens.peek(), Some(Token::Comma)) {
                self.tokens.next();
            }
        }
        self.tokens.next();

        let return_type = if matches!(self.tokens.peek(), Some(Token::Arrow)) {
            self.tokens.next();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.parse_semicolon()?;

        Ok(AST::ExternFn {
            name,
            args,
            return_type,
        })
    }

    fn parse_asm(&mut self) -> Result<AST, String> {
        if !matches!(self.tokens.peek(), Some(Token::Asm)) {
            return Err("Expected 'asm'".into());
        }
        self.tokens.next(); // consume 'asm'
        if !matches!(self.tokens.peek(), Some(Token::LBrace)) {
            return Err("Expected '{' after 'asm'".into());
        }
        self.tokens.next(); // consume '{'

        let mut assembly = String::new();
        while let Some(tokens) = self.tokens.peek() {
            if matches!(tokens, Token::RBrace) {
                break;
            }

            match self.tokens.next() {
                Some(Token::StringLit(s)) => {
                    assembly.push_str(s.trim_matches('"'));
                    assembly.push_str("\n");
                }
                _ => return Err("ASM ERROR".into()),
            }
        }
        if !matches!(self.tokens.next(), Some(Token::RBrace)) {
            return Err("Expected '}' at the end of asm block".into());
        }
        return Ok(AST::AsmBlock { assembly });
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if matches!(self.tokens.peek(), Some(Token::Star)) {
            self.tokens.next();
            let inner = self.parse_type()?;
            return Ok(Type::Ptr(Box::new(inner)));
        }

        match self.tokens.next() {
            Some(Token::I8) => Ok(Type::I8),
            Some(Token::I16) => Ok(Type::I16),
            Some(Token::I32) => Ok(Type::I32),
            Some(Token::I64) => Ok(Type::I64),
            Some(Token::I128) => Ok(Type::I128),
            Some(Token::F32) => Ok(Type::F32),
            Some(Token::F64) => Ok(Type::F64),
            Some(Token::Void) => Ok(Type::Void),
            Some(Token::Bool) => Ok(Type::Bool),
            Some(Token::Ident(name)) => Ok(Type::Custom(name)),
            _ => return Err("unknown type".into()),
        }
    }
}

pub fn parse_manager(tokens: Vec<Token>) -> Result<Vec<AST>, String> {
    Parser::new(tokens).parse_program()
}
