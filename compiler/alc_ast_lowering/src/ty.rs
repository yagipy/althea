use crate::{idx::Idx, idx_vec::IdxVec};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ty(usize);

impl Idx for Ty {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        Ty(index)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariantIdx(usize);

impl fmt::Debug for VariantIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "variant_{}", self.0)
    }
}

impl Idx for VariantIdx {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        VariantIdx(index)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FieldIdx(usize);

impl fmt::Debug for FieldIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "field_{}", self.0)
    }
}

impl Idx for FieldIdx {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        FieldIdx(index)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParamIdx(usize);

impl fmt::Debug for ParamIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arg_{}", self.0)
    }
}

impl Idx for ParamIdx {
    #[inline]
    fn index(&self) -> usize {
        self.0
    }

    #[inline]
    fn new(index: usize) -> Self {
        ParamIdx(index)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TyKind {
    I8,
    I16,
    I32,
    I64,
    String,
    Array(Array),
    Enum(Enum),
    Struct(Struct),
    Fn(Prototype),
}

impl TyKind {
    #[inline]
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            TyKind::Array(ref desc) => Some(desc),
            _ => None,
        }
    }

    #[inline]
    pub fn as_enum(&self) -> Option<&Enum> {
        match self {
            TyKind::Enum(ref desc) => Some(desc),
            _ => None,
        }
    }

    #[inline]
    pub fn as_enum_mut(&mut self) -> Option<&mut Enum> {
        match self {
            TyKind::Enum(ref mut desc) => Some(desc),
            _ => None,
        }
    }

    #[inline]
    pub fn as_struct(&self) -> Option<&Struct> {
        match self {
            TyKind::Struct(ref desc) => Some(desc),
            _ => None,
        }
    }

    #[inline]
    pub fn as_struct_mut(&mut self) -> Option<&mut Struct> {
        match self {
            TyKind::Struct(ref mut desc) => Some(desc),
            _ => None,
        }
    }

    #[inline]
    pub fn as_prototype(&self) -> Option<&Prototype> {
        match self {
            TyKind::Fn(ref prototype) => Some(prototype),
            _ => None,
        }
    }

    #[inline]
    pub fn field_ty(&self, field_idx: FieldIdx) -> Option<Ty> {
        self.as_struct()
            .and_then(|desc| desc.fields.get(field_idx))
            .copied()
    }

    #[inline]
    pub fn field_count(&self) -> Option<usize> {
        self.as_struct().map(|desc| desc.fields.len())
    }

    #[inline]
    pub fn variant_ty(&self, variant_idx: VariantIdx) -> Option<Ty> {
        self.as_enum()
            .and_then(|desc| desc.variants.get(variant_idx))
            .copied()
    }

    #[inline]
    pub fn variant_count(&self) -> Option<usize> {
        self.as_enum().map(|desc| desc.variants.len())
    }

    #[inline]
    pub fn param_ty(&self, param_idx: ParamIdx) -> Option<Ty> {
        self.as_prototype()
            .and_then(|desc| desc.params.get(param_idx))
            .copied()
    }

    #[inline]
    pub fn param_count(&self) -> Option<usize> {
        self.as_prototype().map(|desc| desc.params.len())
    }

    #[inline]
    pub fn return_ty(&self) -> Option<Ty> {
        self.as_prototype().map(|desc| desc.return_ty)
    }

    #[inline]
    pub fn is_i8(&self) -> bool {
        matches!(self, TyKind::I8)
    }

    #[inline]
    pub fn is_i16(&self) -> bool {
        matches!(self, TyKind::I16)
    }

    #[inline]
    pub fn is_i32(&self) -> bool {
        matches!(self, TyKind::I32)
    }

    #[inline]
    pub fn is_i64(&self) -> bool {
        matches!(self, TyKind::I64)
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, TyKind::String)
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, TyKind::Array(_))
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        matches!(self, TyKind::Enum(_))
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self, TyKind::Struct(_))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Array {
    pub element_ty: Ty,
    pub size: i32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Enum {
    pub variants: IdxVec<VariantIdx, Ty>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Struct {
    pub fields: IdxVec<FieldIdx, Ty>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Prototype {
    pub return_ty: Ty,
    pub params: IdxVec<ParamIdx, Ty>,
}

#[derive(Debug, Default)]
pub struct TySess {
    tys: RefCell<IdxVec<Ty, TyKind>>,
    uniqued: RefCell<HashMap<TyKind, Ty>>,
}

impl TySess {
    #[inline]
    pub fn new() -> TySess {
        TySess::default()
    }

    fn bind(&mut self, kind: TyKind) -> Ty {
        self.tys.borrow_mut().push(kind)
    }

    fn make_unique(&self, kind: TyKind) -> Ty {
        let mut uniqued = self.uniqued.borrow_mut();
        if let Some(ty) = uniqued.get(&kind).copied() {
            return ty;
        }
        let ty = self.tys.borrow_mut().push(kind.clone());
        uniqued.insert(kind, ty);
        ty
    }

    pub fn make_i8(&self) -> Ty {
        self.make_unique(TyKind::I8)
    }

    pub fn make_i16(&self) -> Ty {
        self.make_unique(TyKind::I16)
    }

    pub fn make_i32(&self) -> Ty {
        self.make_unique(TyKind::I32)
    }

    pub fn make_i64(&self) -> Ty {
        self.make_unique(TyKind::I64)
    }

    pub fn make_string(&self) -> Ty {
        self.make_unique(TyKind::String)
    }

    pub fn make_array(&self, element_ty: Ty, size: i32) -> Ty {
        self.make_unique(TyKind::Array(Array { element_ty, size }))
    }

    pub fn make_fn(&self, return_ty: Ty, params: IdxVec<ParamIdx, Ty>) -> Ty {
        self.make_unique(TyKind::Fn(Prototype { return_ty, params }))
    }

    pub fn make_enum(&mut self) -> Ty {
        self.bind(TyKind::Enum(Enum {
            variants: IdxVec::new(),
        }))
    }

    pub fn make_struct(&mut self) -> Ty {
        self.bind(TyKind::Struct(Struct {
            fields: IdxVec::new(),
        }))
    }

    pub fn ty_kind(&self, ty: Ty) -> TyKindRef {
        TyKindRef {
            ty,
            guard: self.tys.borrow(),
        }
    }

    pub fn ty_kind_mut(&mut self, ty: Ty) -> TyKindRefMut {
        TyKindRefMut {
            ty,
            guard: self.tys.borrow_mut(),
        }
    }

    #[inline]
    pub fn tys(&self) -> usize {
        self.tys.borrow().len()
    }
}

pub struct TyKindRef<'a> {
    ty: Ty,
    guard: Ref<'a, IdxVec<Ty, TyKind>>,
}

pub struct TyKindRefMut<'a> {
    ty: Ty,
    guard: RefMut<'a, IdxVec<Ty, TyKind>>,
}

impl<'a> Deref for TyKindRef<'a> {
    type Target = TyKind;

    fn deref(&self) -> &Self::Target {
        self.guard.get(self.ty).unwrap()
    }
}

impl<'a> Deref for TyKindRefMut<'a> {
    type Target = TyKind;

    fn deref(&self) -> &Self::Target {
        self.guard.get(self.ty).unwrap()
    }
}

impl<'a> DerefMut for TyKindRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.get_mut(self.ty).unwrap()
    }
}
