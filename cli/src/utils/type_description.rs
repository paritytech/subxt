use color_eyre::eyre::eyre;
use std::{fs, process::Output};

use frame_metadata::RuntimeMetadataPrefixed;
use scale_info::{
    form::PortableForm,
    scale::{Decode, Encode},
    Field, PortableRegistry, Type, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, TypeInfo,
    Variant,
};
use scale_value::Value;

pub trait TypeDescription {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String>;
}

impl TypeDescription for u32 {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let ty = registry
            .resolve(*self)
            .ok_or(eyre!("Type with id {} not found in registry", *self))?;
        let ident = ty.path.ident();
        let prefix = type_def_prefix(&ty.type_def);
        let mut type_def_description = ty.type_def.type_description(registry)?;
        if let Some(ident) = ident {
            type_def_description = format!("{} {}", ident, type_def_description)
        }
        if let Some(prefix) = prefix {
            type_def_description = format!("{} {}", prefix, type_def_description)
        }
        Ok(type_def_description)
    }
}

fn type_def_prefix(type_def: &TypeDef<PortableForm>) -> Option<&str> {
    match type_def {
        TypeDef::Composite(_) => Some("struct"),
        TypeDef::Variant(_) => Some("enum"),
        _ => None,
    }
}

impl TypeDescription for TypeDef<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        match self {
            TypeDef::Composite(composite) => composite.fields.type_description(registry),
            TypeDef::Variant(variant) => variant.type_description(registry),
            TypeDef::Sequence(sequence) => sequence.type_description(registry),
            TypeDef::Array(array) => array.type_description(registry),
            TypeDef::Tuple(tuple) => tuple.type_description(registry),
            TypeDef::Primitive(primitive) => primitive.type_description(registry),
            TypeDef::Compact(compact) => compact.type_description(registry),
            TypeDef::BitSequence(bit_sequence) => bit_sequence.type_description(registry),
        }
    }
}

impl TypeDescription for TypeDefTuple<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let mut output = String::new();
        output.push('(');
        for (is_last, ty) in mark_last(self.fields.iter(), self.fields.len()) {
            let type_description = ty.id.type_description(registry)?;
            output.push_str(&type_description);
            if !is_last {
                output.push(',')
            }
        }
        output.push(')');
        Ok(output)
    }
}

impl TypeDescription for TypeDefBitSequence<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let bit_order_type = self.bit_order_type.id.type_description(registry)?;
        let bit_store_type = self.bit_store_type.id.type_description(registry)?;
        Ok(format!("BitSequence({bit_order_type}, {bit_store_type})"))
    }
}

impl TypeDescription for TypeDefSequence<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let type_description = self.type_param.id.type_description(registry)?;
        Ok(format!("Sequence({type_description})"))
    }
}

impl TypeDescription for TypeDefCompact<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let type_description = self.type_param.id.type_description(registry)?;
        Ok(format!("Compact({type_description})"))
    }
}

impl TypeDescription for TypeDefArray<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let type_description = self.type_param.id.type_description(registry)?;
        Ok(format!("[{type_description}; {}]", self.len))
    }
}

impl TypeDescription for TypeDefPrimitive {
    fn type_description(&self, _registry: &PortableRegistry) -> color_eyre::Result<String> {
        Ok(match &self {
            TypeDefPrimitive::Bool => "bool",
            TypeDefPrimitive::Char => "char",
            TypeDefPrimitive::Str => "String",
            TypeDefPrimitive::U8 => "u8",
            TypeDefPrimitive::U16 => "u16",
            TypeDefPrimitive::U32 => "u32",
            TypeDefPrimitive::U64 => "u64",
            TypeDefPrimitive::U128 => "u128",
            TypeDefPrimitive::U256 => "u256",
            TypeDefPrimitive::I8 => "i8",
            TypeDefPrimitive::I16 => "i16",
            TypeDefPrimitive::I32 => "i32",
            TypeDefPrimitive::I64 => "i64",
            TypeDefPrimitive::I128 => "i128",
            TypeDefPrimitive::I256 => "i256",
        }
        .into())
    }
}

impl TypeDescription for TypeDefVariant<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        const MIN_VARIANT_COUNT_FOR_TRAILING_COMMA: usize = 100;
        let add_trailing_comma = self.variants.len() >= MIN_VARIANT_COUNT_FOR_TRAILING_COMMA;

        let mut variants_string = String::new();
        variants_string.push('{');
        for (is_last, variant) in mark_last(self.variants.iter(), self.variants.len()) {
            let variant_string = variant.type_description(registry)?;
            variants_string.push_str(&variant_string);

            if !is_last || add_trailing_comma {
                variants_string.push(',');
            }
        }
        variants_string.push('}');
        Ok(variants_string)
    }
}

impl TypeDescription for Variant<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let fields_string = self.fields.type_description(registry)?;
        let output = if fields_string.is_empty() {
            self.name.to_string()
        } else if fields_string.starts_with("(") {
            format!("{}{}", &self.name, fields_string)
        } else {
            format!("{} {}", &self.name, fields_string)
        };
        Ok(output)
    }
}

impl TypeDescription for Vec<Field<PortableForm>> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        if self.is_empty() {
            return Ok("()".to_string());
        }

        const MIN_FIELD_COUNT_FOR_TRAILING_COMMA: usize = 100;
        let add_trailing_comma = self.len() >= MIN_FIELD_COUNT_FOR_TRAILING_COMMA;

        let all_fields_named = self.iter().all(|f| f.name.is_some());
        let all_fields_unnamed = self.iter().all(|f| f.name.is_none());
        let brackets = match (all_fields_named, all_fields_unnamed) {
            (true, false) => ('{', '}'),
            (false, true) => ('(', ')'),
            _ => {
                return Err(eyre!(
                    "combination of named and unnamed fields in compound type"
                ));
            }
        };

        let mut fields_string = String::new();
        fields_string.push(brackets.0);
        for (is_last, field) in mark_last(self.iter(), self.len()) {
            let field_description = field.type_description(registry)?;
            fields_string.push_str(&field_description);

            if !is_last || add_trailing_comma {
                fields_string.push(',')
            }
        }
        fields_string.push(brackets.1);
        Ok(fields_string)
    }
}

impl TypeDescription for Field<PortableForm> {
    fn type_description(&self, registry: &PortableRegistry) -> color_eyre::Result<String> {
        let type_description = self.ty.id.type_description(registry)?;
        let type_description_maybe_named = if let Some(name) = &self.name {
            format!("{}: {}", name, type_description)
        } else {
            type_description
        };
        Ok(type_description_maybe_named)
    }
}

pub fn format_type_description(input: &str) -> String {
    fn add_indentation(output: &mut String, indent_level: i32) {
        for _ in 0..indent_level {
            output.push_str("    ");
        }
    }

    let mut output = String::new();
    let mut indent_level = 0;
    // in a tuple we will not set line breaks on comma, so we keep track of it here:
    let mut in_tuple = 0;
    for ch in input.chars() {
        match ch {
            '{' => {
                indent_level += 1;
                output.push(ch);
                output.push('\n');
                add_indentation(&mut output, indent_level);
            }
            '}' => {
                indent_level -= 1;
                output.push('\n');
                add_indentation(&mut output, indent_level);
                output.push(ch);
            }
            ',' => {
                output.push(ch);
                if in_tuple > 0 {
                    output.push(' ');
                } else {
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }
            }

            '(' => {
                output.push(ch);
                in_tuple += 1;
            }
            ')' => {
                output.push(ch);
                in_tuple -= 1;
            }
            _ => {
                output.push(ch);
            }
        }
    }
    output
}

fn mark_last<T>(items: impl Iterator<Item = T>, len: usize) -> impl Iterator<Item = (bool, T)> {
    items.enumerate().map(move |(i, e)| (i == len - 1, e))
}
