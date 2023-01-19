use crate::{
    idx::{Idx, Idxr},
    idx_vec::IdxVec,
    ty::{FieldIdx, ParamIdx, Ty, VariantIdx},
};
use alc_diagnostic::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DefIdx(usize);

impl fmt::Debug for DefIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.0)
    }
}

impl Idx for DefIdx {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        DefIdx(index)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockIdx(usize);

impl Idx for BlockIdx {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        BlockIdx(index)
    }
}

#[derive(Copy, Clone, Eq)]
pub struct LocalIdx {
    span: Span,
    idx: usize,
}

impl fmt::Debug for LocalIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.idx)
    }
}

impl LocalIdx {
    pub fn with_span(&self, span: Span) -> LocalIdx {
        LocalIdx { span, idx: self.idx }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Idx for LocalIdx {
    #[inline]
    fn index(&self) -> usize {
        self.idx
    }

    fn new(index: usize) -> Self {
        LocalIdx {
            span: Span::dummy(),
            idx: index,
        }
    }
}

impl PartialEq for LocalIdx {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Hash for LocalIdx {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnopKind {
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
pub enum ExprKind {
    I8Literal(i8),
    I16Literal(i16),
    I32Literal(i32),
    I64Literal(i64),
    ArrayLiteral {
        element_ty: Ty,
        elements: Vec<ExprKind>,
    },
    StringLiteral(String),
    Var(LocalIdx, Vec<FieldIdx>),
    Unop {
        kind: UnopKind,
        operand: LocalIdx,
    },
    Binop {
        kind: BinopKind,
        left: LocalIdx,
        right: LocalIdx,
    },
    Call {
        target: DefIdx,
        args: IdxVec<ParamIdx, LocalIdx>,
    },
    Variant {
        ty: Ty,
        discriminant: VariantIdx,
        body: LocalIdx,
    },
    Record {
        ty: Ty,
        fields: IdxVec<FieldIdx, LocalIdx>,
    },
    Socket {
        domain: LocalIdx,
        ty: LocalIdx,
        protocol: LocalIdx,
    },
    Bind {
        socket_file_descriptor: LocalIdx,
        address: LocalIdx,
        address_length: LocalIdx,
    },
    Listen {
        socket_file_descriptor: LocalIdx,
        backlog: LocalIdx,
    },
    Accept {
        socket_file_descriptor: LocalIdx,
    },
    Recv {
        socket_file_descriptor: LocalIdx,
        buffer: LocalIdx,
        buffer_length: LocalIdx,
        flags: LocalIdx,
    },
    Send {
        socket_file_descriptor: LocalIdx,
        buffer: LocalIdx,
        buffer_length: LocalIdx,
        content: LocalIdx,
        flags: LocalIdx,
    },
    Close {
        socket_file_descriptor: LocalIdx,
    },
    ListenAndServe {
        domain: LocalIdx,
        ty: LocalIdx,
        protocol: LocalIdx,
        address: LocalIdx,
        address_length: LocalIdx,
        backlog: LocalIdx,
        recv_buffer: LocalIdx,
        recv_buffer_length: LocalIdx,
        recv_flags: LocalIdx,
        send_buffer: LocalIdx,
        send_buffer_length: LocalIdx,
        send_flags: LocalIdx,
        format_string: LocalIdx,
        http_header: LocalIdx,
        call_handler: LocalIdx,
    },
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub local_idx: LocalIdx,
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Clone, Debug)]
pub enum PatternKind {
    I8Literal(i8),
    I16Literal(i16),
    I32Literal(i32),
    I64Literal(i64),
    ArrayLiteral {
        element_ty: Ty,
        elements: Vec<ExprKind>,
    },
    StringLiteral(String),
    Ident(LocalIdx),
    Variant {
        ty: Ty,
        discriminant: VariantIdx,
        binding: LocalIdx,
    },
    Record {
        ty: Ty,
        fields: IdxVec<FieldIdx, LocalIdx>,
    },
}

#[derive(Debug)]
pub struct Arm {
    pub span: Span,
    pub pattern: PatternKind,
    pub target: Block,
}

#[derive(Clone, Debug)]
pub enum InstructionKind {
    Let {
        binding: LocalIdx,
        ty: Option<Ty>,
        expr: Expr,
    },
    Println {
        idx: LocalIdx,
    },
    Mark(LocalIdx, Ty),
    Unmark(LocalIdx, Ty),
    Free(LocalIdx, Ty),
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum Terminator {
    Return(LocalIdx),
    Match { source: LocalIdx, arms: Vec<Arm> },
}

#[derive(Debug)]
pub struct Block {
    pub owner: DefIdx,
    pub block_idx: BlockIdx,
    pub span: Span,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug)]
pub struct Entry {
    pub owner: DefIdx,
    pub param_bindings: IdxVec<ParamIdx, LocalIdx>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Def {
    pub def_idx: DefIdx,
    pub name: String,
    pub ty: Ty,
    pub span: Span,
    pub entry: Entry,
    pub local_idxr: Idxr<LocalIdx>,
}

#[derive(Debug)]
pub struct Ir {
    pub defs: IdxVec<DefIdx, Def>,
}
