// AeroFlow Compiler - Parser (v1.0 Locked Spec)
// Recursive descent, single-pass

use crate::lexer::{Lexer, TokenKind};
use crate::ast::{Expr, Stmt};
use smallvec::SmallVec;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: TokenKind,
    previous: TokenKind,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Self {
            lexer,
            current: TokenKind::EOF,
            previous: TokenKind::EOF,
        };
        p.advance();
        p
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        self.current = self.lexer.next_token();
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if std::mem::discriminant(&self.current) == std::mem::discriminant(&kind) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if !self.match_token(kind) {
            panic!("{}", message);
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.current != TokenKind::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> Stmt {
        if self.match_token(TokenKind::Let) { self.parse_let() }
        else if self.match_token(TokenKind::Fn) { self.parse_fn(false) }
        else if self.match_token(TokenKind::Pure) { 
            self.consume(TokenKind::Fn, "Expect 'fn' after 'pure'");
            self.parse_fn(true) 
        }
        else if self.match_token(TokenKind::Actor) { self.parse_actor() }
        else if self.match_token(TokenKind::Screen) { self.parse_screen() }
        else if self.match_token(TokenKind::Agent) { self.parse_agent() }
        else if self.match_token(TokenKind::Model) { self.parse_model() }
        else if self.match_token(TokenKind::From) { self.parse_from() }
        else if self.match_token(TokenKind::Render) { self.parse_render() }
        else if self.match_token(TokenKind::Spawn) { self.parse_spawn() }
        else if self.match_token(TokenKind::If) { self.parse_if() }
        else if self.match_token(TokenKind::While) { self.parse_while() }
        else if self.match_token(TokenKind::Return) { 
            let expr = self.parse_expression();
            Stmt::Return(expr)
        }
        else { Stmt::Expr(self.parse_expression()) }
    }

    fn parse_if(&mut self) -> Stmt {
        let condition = self.parse_expression();
        self.consume(TokenKind::LBrace, "Expect '{' after if condition");
        let mut then_branch = Vec::new();
        while self.current != TokenKind::RBrace && self.current != TokenKind::EOF {
            then_branch.push(self.parse_statement());
        }
        self.consume(TokenKind::RBrace, "Expect '}' after then branch");
        
        let else_branch = if self.match_token(TokenKind::Else) {
            if self.match_token(TokenKind::If) {
                Some(vec![self.parse_if()])
            } else {
                self.consume(TokenKind::LBrace, "Expect '{' after else");
                let mut branch = Vec::new();
                while self.current != TokenKind::RBrace && self.current != TokenKind::EOF {
                    branch.push(self.parse_statement());
                }
                self.consume(TokenKind::RBrace, "Expect '}' after else branch");
                Some(branch)
            }
        } else {
            None
        };
        
        Stmt::If { condition, then_branch, else_branch }
    }

    fn parse_while(&mut self) -> Stmt {
        let condition = self.parse_expression();
        self.consume(TokenKind::LBrace, "Expect '{' after while condition");
        let mut body = Vec::new();
        while self.current != TokenKind::RBrace && self.current != TokenKind::EOF {
            body.push(self.parse_statement());
        }
        self.consume(TokenKind::RBrace, "Expect '}' after while body");
        Stmt::While { condition, body }
    }

    fn parse_type(&mut self) -> crate::ast::Type {
        if self.match_token(TokenKind::IntType) { crate::ast::Type::Int }
        else if self.match_token(TokenKind::FloatType) { crate::ast::Type::Float }
        else if self.match_token(TokenKind::StringType) { crate::ast::Type::String }
        else if self.match_token(TokenKind::BoolType) { crate::ast::Type::Bool }
        else if self.match_token(TokenKind::List) {
            self.consume(TokenKind::LAngle, "Expect '<' after 'list'");
            let inner = self.parse_type();
            self.consume(TokenKind::RAngle, "Expect '>' after list type");
            crate::ast::Type::List(Box::new(inner))
        }
        else if self.match_token(TokenKind::Dict) {
            self.consume(TokenKind::LAngle, "Expect '<' after 'dict'");
            let key = self.parse_type();
            self.consume(TokenKind::Comma, "Expect ',' between dict types");
            let value = self.parse_type();
            self.consume(TokenKind::RAngle, "Expect '>' after dict types");
            crate::ast::Type::Dict(Box::new(key), Box::new(value))
        }
        else { crate::ast::Type::Void }
    }

    fn parse_let(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect variable name after let");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        
        self.consume(TokenKind::Colon, "Expect ':' after variable name");
        let r#type = self.parse_type();

        self.consume(TokenKind::Equal, "Expect '=' after type");
        let value = self.parse_expression();
        Stmt::VarDecl { name, r#type, value }
    }

    fn parse_fn(&mut self, is_pure: bool) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect function name");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        
        self.consume(TokenKind::LParen, "Expect '(' after function name");
        let mut params = Vec::new();
        if self.current != TokenKind::RParen {
            loop {
                self.consume(TokenKind::Ident(String::new()), "Expect param name");
                let p_name = if let TokenKind::Ident(p) = &self.previous { p.clone() } else { panic!() };
                self.consume(TokenKind::Colon, "Expect ':' after param name");
                let p_type = self.parse_type();
                params.push((p_name, p_type));
                if !self.match_token(TokenKind::Comma) { break; }
            }
        }
        self.consume(TokenKind::RParen, "Expect ')' after params");
        
        let return_type = if self.match_token(TokenKind::Arrow) {
            self.parse_type()
        } else {
            crate::ast::Type::Void
        };

        self.consume(TokenKind::LBrace, "Expect '{' before function body");
        let mut body = Vec::new();
        while self.current != TokenKind::RBrace && self.current != TokenKind::EOF {
            body.push(self.parse_statement());
        }
        self.consume(TokenKind::RBrace, "Expect '}' after function body");
        
        Stmt::Fn { name, params, body, return_type, is_pure }
    }

    fn parse_from(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect package name after 'from'");
        let package = if let TokenKind::Ident(p) = &self.previous { p.clone() } else { panic!() };
        
        self.consume(TokenKind::Ident(String::new()), "Expect layer name after package");
        let layer = if let TokenKind::Ident(l) = &self.previous { l.clone() } else { panic!() };
        
        Stmt::FromImport { package, layer }
    }

    fn parse_render(&mut self) -> Stmt {
        self.consume(TokenKind::LBrace, "Expect '{' after 'render'");
        
        let render_expr = if self.match_token(TokenKind::Timeline) {
            crate::ast::RenderExpression::Timeline(self.parse_timeline())
        } else if self.match_token(TokenKind::Distributed) {
            self.consume(TokenKind::State, "Expect 'state' after 'distributed'");
            crate::ast::RenderExpression::DistributedState(self.parse_distributed_state())
        } else if self.current == TokenKind::TextWidget || self.current == TokenKind::InputWidget || self.current == TokenKind::ButtonWidget {
            let mut widgets = Vec::new();
            while self.current == TokenKind::TextWidget || self.current == TokenKind::InputWidget || self.current == TokenKind::ButtonWidget {
                widgets.push(self.parse_ui_widget());
            }
            crate::ast::RenderExpression::UIWidgets(widgets)
        } else {
            crate::ast::RenderExpression::Expr(self.parse_expression())
        };

        self.consume(TokenKind::RBrace, "Expect '}' after render block");
        Stmt::Render(render_expr)
    }

    fn parse_ui_widget(&mut self) -> crate::ast::UIWidget {
        if self.match_token(TokenKind::TextWidget) {
            self.consume(TokenKind::LBrace, "Expect '{' after Text");
            let expr = self.parse_expression();
            self.consume(TokenKind::RBrace, "Expect '}' after expression");
            crate::ast::UIWidget::Text(expr)
        } else if self.match_token(TokenKind::InputWidget) {
            self.consume(TokenKind::LBrace, "Expect '{' after Input");
            self.consume(TokenKind::Bind, "Expect 'bind' in Input");
            self.consume(TokenKind::Colon, "Expect ':' after bind");
            self.consume(TokenKind::Ident(String::new()), "Expect variable name");
            let bind = if let TokenKind::Ident(id) = &self.previous { id.clone() } else { panic!() };
            self.consume(TokenKind::RBrace, "Expect '}' after Input");
            crate::ast::UIWidget::Input { bind }
        } else if self.match_token(TokenKind::ButtonWidget) {
            self.consume(TokenKind::LBrace, "Expect '{' after Button");
            self.consume(TokenKind::String(String::new()), "Expect button label");
            let label = if let TokenKind::String(s) = &self.previous { s.clone() } else { panic!() };
            self.consume(TokenKind::Comma, "Expect ',' after label");
            self.consume(TokenKind::OnClick, "Expect 'onClick'");
            self.consume(TokenKind::Colon, "Expect ':' after onClick");
            let on_click = self.parse_expression();
            self.consume(TokenKind::RBrace, "Expect '}' after Button");
            crate::ast::UIWidget::Button { label, on_click }
        } else {
            panic!("Unexpected UI widget token: {:?}", self.current);
        }
    }

    fn parse_screen(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect screen name");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        self.consume(TokenKind::LBrace, "Expect '{'");
        let mut body = Vec::new();
        while self.current != TokenKind::RBrace {
            body.push(self.parse_statement());
        }
        self.consume(TokenKind::RBrace, "Expect '}'");
        Stmt::Screen { name, body }
    }

    fn parse_timeline(&mut self) -> crate::ast::TimelineBlock {
        self.consume(TokenKind::LBrace, "Expect '{' after 'timeline'");
        let mut events = Vec::new();
        while self.current != TokenKind::RBrace {
            self.consume(TokenKind::Ident(String::new()), "Expect node name");
            let from = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
            self.consume(TokenKind::Arrow, "Expect '->'");
            self.consume(TokenKind::Ident(String::new()), "Expect target node name");
            let to = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
            self.consume(TokenKind::At, "Expect 'at'");
            
            let at_ms = if self.match_token(TokenKind::Number(0.0)) {
                if let TokenKind::Number(n) = self.previous { n as u64 } else { 0 }
            } else {
                self.consume(TokenKind::Tick, "Expect 'tick'");
                self.consume(TokenKind::Equal, "Expect '=' after 'tick'");
                self.consume(TokenKind::Number(0.0), "Expect tick number");
                if let TokenKind::Number(n) = self.previous { n as u64 } else { 0 }
            };

            // Skip 'ms' if present (it's part of the number token usually or extra token)
            self.match_token(TokenKind::Ident("ms".to_string()));

            self.consume(TokenKind::Payload, "Expect 'payload'");
            let payload = self.parse_expression();
            events.push(crate::ast::TimelineEvent { from, to, at_ms, payload });
        }
        self.consume(TokenKind::RBrace, "Expect '}' after timeline");
        crate::ast::TimelineBlock { events }
    }

    fn parse_distributed_state(&mut self) -> crate::ast::DistributedStateBlock {
        self.consume(TokenKind::LBrace, "Expect '{' after 'distributed state'");
        let mut state_refs = Vec::new();
        while self.current != TokenKind::RBrace {
            self.consume(TokenKind::Ident(String::new()), "Expect node name");
            let node = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
            self.consume(TokenKind::Dot, "Expect '.'");
            self.consume(TokenKind::Ident(String::new()), "Expect field name");
            let field = if let TokenKind::Ident(f) = &self.previous { f.clone() } else { panic!() };
            state_refs.push(crate::ast::NodeStateRef { node, field });
        }
        self.consume(TokenKind::RBrace, "Expect '}' after distributed state");
        crate::ast::DistributedStateBlock { state_refs }
    }

    fn parse_spawn(&mut self) -> Stmt {
        let expr = self.parse_expression();
        Stmt::Spawn(expr)
    }

    fn parse_actor(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect actor name");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        self.consume(TokenKind::LBrace, "Expect '{'");
        let mut body = Vec::new();
        while self.current != TokenKind::RBrace {
            body.push(self.parse_statement());
        }
        self.consume(TokenKind::RBrace, "Expect '}'");
        Stmt::Actor { name, body }
    }

    fn parse_agent(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect agent name");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        self.consume(TokenKind::LBrace, "Expect '{'");
        
        let mut model = None;
        let mut handlers = Vec::new();
        let mut body = Vec::new();

        while self.current != TokenKind::RBrace {
            if self.match_token(TokenKind::Model) {
                self.consume(TokenKind::String(String::new()), "Expect model identifier string");
                if let TokenKind::String(s) = &self.previous { model = Some(s.clone()); }
            } else if self.match_token(TokenKind::On) {
                self.consume(TokenKind::Ident(String::new()), "Expect event name");
                let h_name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
                self.consume(TokenKind::LParen, "Expect '('");
                let mut h_params = Vec::new();
                if self.current != TokenKind::RParen {
                    loop {
                        self.consume(TokenKind::Ident(String::new()), "Expect param name");
                        let p_name = if let TokenKind::Ident(p) = &self.previous { p.clone() } else { panic!() };
                        self.consume(TokenKind::Colon, "Expect ':' after param name");
                        let p_type = self.parse_type();
                        h_params.push((p_name, p_type));
                        if !self.match_token(TokenKind::Comma) { break; }
                    }
                }
                self.consume(TokenKind::RParen, "Expect ')'");
                self.consume(TokenKind::LBrace, "Expect '{'");
                let mut h_body = Vec::new();
                while self.current != TokenKind::RBrace {
                    h_body.push(self.parse_statement());
                }
                self.consume(TokenKind::RBrace, "Expect '}'");
                handlers.push(crate::ast::EventHandler { name: h_name, params: h_params, body: h_body });
            } else {
                body.push(self.parse_statement());
            }
        }
        self.consume(TokenKind::RBrace, "Expect '}'");
        Stmt::Agent { name, model, handlers, body }
    }

    fn parse_model(&mut self) -> Stmt {
        self.consume(TokenKind::Ident(String::new()), "Expect model name");
        let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
        self.consume(TokenKind::LBrace, "Expect '{'");
        self.consume(TokenKind::RBrace, "Expect '}'");
        Stmt::Model { name, source: String::new(), body: Vec::new() }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_precedence: u8) -> Expr {
        let mut left = self.parse_primary();

        while let Some(precedence) = self.get_precedence(&self.current) {
            if precedence < min_precedence {
                break;
            }

            let op = self.current.clone();
            self.advance();
            let right = self.parse_binary(precedence + 1);
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn get_precedence(&self, token: &TokenKind) -> Option<u8> {
        match token {
            TokenKind::EqualEqual | TokenKind::BangEqual => Some(5),
            TokenKind::LAngle | TokenKind::RAngle | TokenKind::LessEqual | TokenKind::GreaterEqual => Some(10),
            TokenKind::Plus | TokenKind::Minus => Some(20),
            TokenKind::Star | TokenKind::Slash => Some(30),
            _ => None,
        }
    }

    fn parse_primary(&mut self) -> Expr {
        if self.match_token(TokenKind::Number(0.0)) {
            if let TokenKind::Number(n) = self.previous { return Expr::Number(n); }
        }
        if self.match_token(TokenKind::String(String::new())) {
            if let TokenKind::String(s) = &self.previous { return Expr::String(s.clone()); }
        }
        if self.match_token(TokenKind::Ident(String::new())) {
            let name = if let TokenKind::Ident(n) = &self.previous { n.clone() } else { panic!() };
            if self.match_token(TokenKind::LParen) {
                let mut args = SmallVec::new();
                if self.current != TokenKind::RParen {
                    loop {
                        args.push(Box::new(self.parse_expression()));
                        if !self.match_token(TokenKind::Comma) { break; }
                    }
                }
                self.consume(TokenKind::RParen, "Expect ')' after args");
                return Expr::Call { name, args };
            }
            return Expr::Ident(name);
        }
        if self.match_token(TokenKind::LParen) {
            let expr = self.parse_expression();
            self.consume(TokenKind::RParen, "Expect ')' after expression");
            return expr;
        }
        if self.match_token(TokenKind::Env) {
            self.consume(TokenKind::LParen, "Expect '(' after 'env'");
            self.consume(TokenKind::String(String::new()), "Expect string after 'env('");
            let key = if let TokenKind::String(s) = &self.previous { s.clone() } else { panic!() };
            self.consume(TokenKind::RParen, "Expect ')' after env key");
            return Expr::Env(key);
        }
        if self.match_token(TokenKind::Time) {
            return Expr::Time;
        }
        if self.match_token(TokenKind::Rand) {
            self.consume(TokenKind::LParen, "Expect '(' after 'rand'");
            self.consume(TokenKind::RParen, "Expect ')' after 'rand('");
            return Expr::Rand;
        }
        panic!("Unexpected token in primary: {:?} (slice: {})", self.current, self.lexer.slice());
    }
}
