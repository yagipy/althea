use alc_diagnostic::Spanned;

pub type Ident = String;

#[derive(Clone, Debug)]
pub enum Ty {
    I8,
    I16,
    I32,
    I64,
    String,
    Array(Box<Ty>, i32),
    TyName(Ident),
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub binder: Spanned<Ident>,
    pub ty: Spanned<Ty>,
}

#[derive(Debug, Copy, Clone)]
pub enum UnopKind {
    Not,
}

#[derive(Debug, Copy, Clone)]
pub enum BinopKind {
    Plus,
    Minus,
    Mul,
    Div,
    Less,
    Leq,
    Greater,
    Geq,
    Eq,
    Neq,
    And,
    Or,
    Xor,
    LShift,
    RShift,
}

#[derive(Clone, Debug)]
pub enum Expr {
    NumberLiteral(i64),
    ArrayLiteral(Vec<Spanned<Expr>>),
    StringLiteral(String),
    Var(Vec<Spanned<Ident>>),
    Unop {
        kind: Spanned<UnopKind>,
        operand: Spanned<Box<Expr>>,
    },
    Binop {
        kind: Spanned<BinopKind>,
        left: Spanned<Box<Expr>>,
        right: Spanned<Box<Expr>>,
    },
    Call {
        target: Spanned<Ident>,
        args: Vec<Spanned<Expr>>,
    },
    Variant {
        enum_name: Spanned<Ident>,
        discriminant: Spanned<Ident>,
        body: Spanned<Box<Expr>>,
    },
    Record {
        struct_name: Spanned<Ident>,
        fields: Vec<(Spanned<Ident>, Spanned<Expr>)>,
    },
    Socket {
        domain: Spanned<Box<Expr>>,
        ty: Spanned<Box<Expr>>,
        protocol: Spanned<Box<Expr>>,
    },
    Bind {
        socket_file_descriptor: Spanned<Box<Expr>>,
        address: Spanned<Box<Expr>>,
        address_length: Spanned<Box<Expr>>,
    },
    Listen {
        socket_file_descriptor: Spanned<Box<Expr>>,
        backlog: Spanned<Box<Expr>>,
    },
    Accept {
        socket_file_descriptor: Spanned<Box<Expr>>,
    },
    Recv {
        socket_file_descriptor: Spanned<Box<Expr>>,
        buffer: Spanned<Box<Expr>>,
        buffer_length: Spanned<Box<Expr>>,
        flags: Spanned<Box<Expr>>,
    },
    Send {
        socket_file_descriptor: Spanned<Box<Expr>>,
        buffer: Spanned<Box<Expr>>,
        buffer_length: Spanned<Box<Expr>>,
        content: Spanned<Box<Expr>>,
        flags: Spanned<Box<Expr>>,
    },
    Close {
        socket_file_descriptor: Spanned<Box<Expr>>,
    },
    ListenAndServe {
        domain: Spanned<Box<Expr>>,
        ty: Spanned<Box<Expr>>,
        protocol: Spanned<Box<Expr>>,
        address: Spanned<Box<Expr>>,
        address_length: Spanned<Box<Expr>>,
        backlog: Spanned<Box<Expr>>,
        recv_buffer: Spanned<Box<Expr>>,
        recv_buffer_length: Spanned<Box<Expr>>,
        recv_flags: Spanned<Box<Expr>>,
        send_buffer: Spanned<Box<Expr>>,
        send_buffer_length: Spanned<Box<Expr>>,
        send_flags: Spanned<Box<Expr>>,
        format_string: Spanned<Box<Expr>>,
        http_header: Spanned<Box<Expr>>,
        call_handler: Spanned<Box<Expr>>,
    },
}

#[derive(Clone, Debug)]
pub enum Pattern {
    NumberLiteral(i64),
    ArrayLiteral(Vec<Spanned<Expr>>),
    StringLiteral(String),
    Ident(Ident),
    Variant {
        enum_name: Spanned<Ident>,
        discriminant: Spanned<Ident>,
        bound: Spanned<Ident>,
    },
    Record {
        struct_name: Spanned<Ident>,
        fields: Vec<(Spanned<Ident>, Spanned<Ident>)>,
    },
}

#[derive(Clone, Debug)]
pub enum Term {
    Let {
        binder: Spanned<Ident>,
        annotation: Option<Spanned<Ty>>,
        expr: Spanned<Expr>,
        body: Box<Spanned<Term>>,
    },
    Match {
        source: Spanned<Expr>,
        arms: Vec<(Spanned<Pattern>, Box<Spanned<Term>>)>,
    },
    If {
        source: Spanned<Expr>,
        then: Box<Spanned<Term>>,
        otherwise: Box<Spanned<Term>>,
    },
    Println {
        expr: Spanned<Expr>,
        body: Box<Spanned<Term>>,
    },
    Return(Expr),
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    pub name: Spanned<Ident>,
    pub params: Vec<Spanned<Binding>>,
    pub return_ty: Spanned<Ty>,
    pub body: Spanned<Term>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: Spanned<Ident>,
    pub variants: Vec<Spanned<Binding>>,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: Spanned<Ident>,
    pub fields: Vec<Spanned<Binding>>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Fn(Box<FnDecl>),
    Enum(Enum),
    Struct(Struct),
}

#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Spanned<Item>>,
}
