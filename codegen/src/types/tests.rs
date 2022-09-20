// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::*;
use pretty_assertions::assert_eq;
use scale_info::{
    meta_type,
    Registry,
    TypeInfo,
};
use syn::parse_quote;

const MOD_PATH: &[&str] = &["subxt_codegen", "types", "tests"];

fn get_mod<'a>(module: &'a Module, path_segs: &[&'static str]) -> Option<&'a Module> {
    let (mod_name, rest) = path_segs.split_first()?;
    let mod_ident = Ident::new(mod_name, Span::call_site());
    let module = module.children.get(&mod_ident)?;
    if rest.is_empty() {
        Some(module)
    } else {
        get_mod(module, rest)
    }
}

#[test]
fn generate_struct_with_primitives() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: bool,
        b: u32,
        c: char,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub a: ::core::primitive::bool,
                    pub b: ::core::primitive::u32,
                    pub c: ::core::primitive::char,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_struct_with_a_struct_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent {
        a: bool,
        b: Child,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child {
        a: i32,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Parent>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Child {
                    pub a: ::core::primitive::i32,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Parent {
                    pub a: ::core::primitive::bool,
                    pub b: root::subxt_codegen::types::tests::Child,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_tuple_struct() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent(bool, Child);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child(i32);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Parent>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct Child(pub ::core::primitive::i32,);

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct Parent(pub ::core::primitive::bool, pub root::subxt_codegen::types::tests::Child,);
                }
            }
                .to_string()
        )
}

#[test]
fn derive_compact_as_for_uint_wrapper_structs() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su8 {
        a: u8,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu8(u8);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su16 {
        a: u16,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu16(u16);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su32 {
        a: u32,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu32(u32);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su64 {
        a: u64,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu64(u64);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su128 {
        a: u128,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu128(u128);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Su8>());
    registry.register_type(&meta_type::<TSu8>());
    registry.register_type(&meta_type::<Su16>());
    registry.register_type(&meta_type::<TSu16>());
    registry.register_type(&meta_type::<Su32>());
    registry.register_type(&meta_type::<TSu32>());
    registry.register_type(&meta_type::<Su64>());
    registry.register_type(&meta_type::<TSu64>());
    registry.register_type(&meta_type::<Su128>());
    registry.register_type(&meta_type::<TSu128>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Su128 { pub a: ::core::primitive::u128, }

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Su16 { pub a: ::core::primitive::u16, }

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Su32 { pub a: ::core::primitive::u32, }

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Su64 { pub a: ::core::primitive::u64, }

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Su8 { pub a: ::core::primitive::u8, }

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TSu128(pub ::core::primitive::u128,);

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TSu16(pub ::core::primitive::u16,);

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TSu32(pub ::core::primitive::u32,);

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TSu64(pub ::core::primitive::u64,);

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TSu8(pub ::core::primitive::u8,);
            }
        }
        .to_string()
    )
}

#[test]
fn generate_enum() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A,
        B(bool),
        C { a: u32 },
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum E {
                    # [codec (index = 0)]
                    A,
                    # [codec (index = 1)]
                    B (::core::primitive::bool,),
                    # [codec (index = 2)]
                    C { a: ::core::primitive::u32, },
                }
            }
        }
        .to_string()
    )
}

#[test]
fn compact_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        #[codec(compact)]
        a: u32,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TupleStruct(#[codec(compact)] u32);

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A {
            #[codec(compact)]
            a: u32,
        },
        B(#[codec(compact)] u32),
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    registry.register_type(&meta_type::<TupleStruct>());
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum E {
                    # [codec (index = 0)]
                    A {
                        #[codec(compact)]
                        a: ::core::primitive::u32,
                    },
                    # [codec (index = 1)]
                    B( #[codec(compact)] ::core::primitive::u32,),
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    #[codec(compact)] pub a: ::core::primitive::u32,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct TupleStruct(#[codec(compact)] pub ::core::primitive::u32,);
            }
        }
        .to_string()
    )
}

#[test]
fn generate_array_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: [u8; 32],
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub a: [::core::primitive::u8; 32usize],
                }
            }
        }
        .to_string()
    )
}

#[test]
fn option_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: Option<bool>,
        b: Option<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub a: ::core::option::Option<::core::primitive::bool>,
                    pub b: ::core::option::Option<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn box_fields_struct() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: std::boxed::Box<bool>,
        b: Box<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub a: ::std::boxed::Box<::core::primitive::bool>,
                    pub b: ::std::boxed::Box<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn box_fields_enum() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A(Box<bool>),
        B { a: Box<u32> },
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum E {
                    # [codec (index = 0)]
                    A(::std::boxed::Box<::core::primitive::bool>,),
                    # [codec (index = 1)]
                    B { a: ::std::boxed::Box<::core::primitive::u32>, },
                }
            }
        }
        .to_string()
    )
}

#[test]
fn range_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: core::ops::Range<u32>,
        b: core::ops::RangeInclusive<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub a: ::core::ops::Range<::core::primitive::u32>,
                    pub b: ::core::ops::RangeInclusive<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T> {
        a: T,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar {
        b: Foo<u32>,
        c: Foo<u8>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Bar>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Bar {
                    pub b: root::subxt_codegen::types::tests::Foo<::core::primitive::u32>,
                    pub c: root::subxt_codegen::types::tests::Foo<::core::primitive::u8>,
                }
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Foo<_0> {
                    pub a: _0,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics_nested() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T, U> {
        a: T,
        b: Option<(T, U)>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar<T> {
        b: Foo<T, u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Bar<bool>>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Bar<_0> {
                    pub b: root::subxt_codegen::types::tests::Foo<_0, ::core::primitive::u32>,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Foo<_0, _1> {
                    pub a: _0,
                    pub b: ::core::option::Option<(_0, _1,)>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_bitvec() {
    use bitvec::{
        order::{
            Lsb0,
            Msb0,
        },
        vec::BitVec,
    };

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        lsb: BitVec<u8, Lsb0>,
        msb: BitVec<u16, Msb0>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct S {
                    pub lsb: ::subxt::ext::bitvec::vec::BitVec<::core::primitive::u8, root::bitvec::order::Lsb0>,
                    pub msb: ::subxt::ext::bitvec::vec::BitVec<::core::primitive::u16, root::bitvec::order::Msb0>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics_with_alias_adds_phantom_data_marker() {
    trait Trait {
        type Type;
    }

    impl Trait for bool {
        type Type = u32;
    }

    type Foo<T> = <T as Trait>::Type;
    type Bar<T, U> = (<T as Trait>::Type, <U as Trait>::Type);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct NamedFields<T: Trait> {
        b: Foo<T>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct UnnamedFields<T: Trait, U: Trait>(Bar<T, U>);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<NamedFields<bool>>());
    registry.register_type(&meta_type::<UnnamedFields<bool, bool>>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct NamedFields<_0> {
                        pub b: ::core::primitive::u32,
                        #[codec(skip)] pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>
                    }
                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct UnnamedFields<_0, _1> (
                        pub (::core::primitive::u32, ::core::primitive::u32,),
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1)>
                    );
                }
            }
                .to_string()
        )
}

#[test]
fn modules() {
    mod m {
        pub mod a {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo;

            pub mod b {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Bar {
                    a: super::Foo,
                }
            }
        }

        pub mod c {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo {
                a: super::a::b::Bar,
            }
        }
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<m::c::Foo>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                pub mod m {
                    use super::root;
                    pub mod a {
                        use super::root;

                        pub mod b {
                            use super::root;

                            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                            pub struct Bar {
                                pub a: root::subxt_codegen::types::tests::m::a::Foo,
                            }
                        }

                        #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                        pub struct Foo;
                    }

                    pub mod c {
                        use super::root;

                        #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                        pub struct Foo {
                            pub a: root::subxt_codegen::types::tests::m::a::b::Bar,
                        }
                    }
                }
            }
        }
        .to_string()
    )
}

#[test]
fn dont_force_struct_names_camel_case() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct AB;

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<AB>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct AB;
            }
        }
        .to_string()
    )
}

#[test]
fn apply_user_defined_derives_for_all_types() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A(B);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct B;

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<A>());
    let portable_types: PortableRegistry = registry.into();

    // configure derives
    let mut derives = DerivesRegistry::default();
    derives.extend_for_all(vec![parse_quote!(Clone), parse_quote!(Eq)]);

    let type_gen =
        TypeGenerator::new(&portable_types, "root", Default::default(), derives);
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Clone, Debug, Eq)]
                pub struct A(pub root :: subxt_codegen :: types :: tests :: B,);

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Clone, Debug, Eq)]
                pub struct B;
            }
        }
            .to_string()
    )
}

#[test]
fn apply_user_defined_derives_for_specific_types() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A(B);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct B(C);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct C;

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<A>());
    let portable_types: PortableRegistry = registry.into();

    // configure derives
    let mut derives = DerivesRegistry::default();
    // for all types
    derives.extend_for_all(vec![parse_quote!(Eq)]);
    // for specific types
    derives.extend_for_type(
        parse_quote!(subxt_codegen::types::tests::B),
        vec![parse_quote!(Hash)],
    );
    // duplicates (in this case `Eq`) will be combined (i.e. a set union)
    derives.extend_for_type(
        parse_quote!(subxt_codegen::types::tests::C),
        vec![
            parse_quote!(Eq),
            parse_quote!(Ord),
            parse_quote!(PartialOrd),
        ],
    );

    let type_gen =
        TypeGenerator::new(&portable_types, "root", Default::default(), derives);
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug, Eq)]
                pub struct A(pub root :: subxt_codegen :: types :: tests :: B,);

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug, Eq, Hash)]
                pub struct B(pub root :: subxt_codegen :: types :: tests :: C,);

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug, Eq, Ord, PartialOrd)]
                pub struct C;
            }
        }
            .to_string()
    )
}
