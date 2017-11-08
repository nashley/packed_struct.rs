extern crate quote;
extern crate syn;

use std::ops::*;
use pack_parse::*;

/*
#[derive(Debug)]
pub struct FieldInfo {
    pub ident: syn::Ident,
    //pub ty: syn::Ty,

    pub serialization_wrapper_ty: SerializationWrapper,
    
    pub bit_width: usize,
    pub bits_position: BitsPositionParsed,

    pub field_kind: FieldKind
}


#[derive(Debug)]
pub enum FieldKind {
    Regular { ty: syn::Ty },
    //Array { size: usize, member_ty: syn::Ty },
    //Enum { pack_ty: syn::Ident }
}
*/

/*
#[derive(Debug)]
pub struct FieldExpanded {
    pub info: FieldInfo,
    /// The range as parsed by our parser. A single byte: 0..7
    pub bit_range: Range<usize>,
    /// The range that can be used by rust's slices. A single byte: 0..8
    pub bit_range_rust: Range<usize>
}
*/

#[derive(Debug)]
pub struct FieldMidPositioning {
    pub bit_width: usize,
    pub bits_position: BitsPositionParsed,
}

#[derive(Debug)]
pub enum FieldKind {
    Regular {
        ident: syn::Ident,
        field: FieldRegular
    },
    Array {
        ident: syn::Ident,
        size: usize,
        elements: Vec<FieldRegular>
    }
}

#[derive(Debug)]
pub struct FieldRegular {
    pub ty: syn::Ty,
    pub serialization_wrappers: Vec<SerializationWrapper>,
    pub bit_width: usize,
    /// The range as parsed by our parser. A single byte: 0..7
    pub bit_range: Range<usize>,
    /// The range that can be used by rust's slices. A single byte: 0..8
    pub bit_range_rust: Range<usize>
}

#[derive(Debug, Clone)]
pub enum SerializationWrapper {
    IntegerWrapper {
        integer: syn::Ty,
    },
    EndiannesWrapper {
        //native_integer_ty: syn::Ty,
        endian: syn::Ty
    },
    PrimitiveEnumWrapper
}


#[derive(Debug)]
pub struct PackStruct {
    pub ast: syn::MacroInput,    
    pub fields: Vec<FieldKind>,
    pub num_bytes: usize,
    pub num_bits: usize
}









