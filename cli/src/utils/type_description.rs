use color_eyre::eyre::eyre;

use scale_info::{
    form::PortableForm, Field, PortableRegistry, TypeDef, TypeDefArray, TypeDefBitSequence,
    TypeDefCompact, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};

/// pretty formatted type description
pub fn print_type_description<T>(ty: &T, registry: &PortableRegistry) -> color_eyre::Result<String>
where
    T: TypeDescription,
{
    let type_description = ty.type_description(registry)?;
    let type_description = format_type_description(&type_description);
    Ok(type_description)
}

/// a trait for producing human readable type descriptions with a rust-like syntax.
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
        let mut output = "(".to_string();
        let mut iter = self.fields.iter().peekable();
        while let Some(ty) = iter.next() {
            let type_description = ty.id.type_description(registry)?;
            output.push_str(&type_description);
            if iter.peek().is_some() {
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
        let mut variants_string = String::new();
        variants_string.push('{');
        let mut iter = self.variants.iter().peekable();
        while let Some(variant) = iter.next() {
            let variant_string = variant.type_description(registry)?;
            variants_string.push_str(&variant_string);

            if iter.peek().is_some() {
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
        } else if fields_string.starts_with('(') {
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
        let mut iter = self.iter().peekable();
        while let Some(field) = iter.next() {
            let field_description = field.type_description(registry)?;
            fields_string.push_str(&field_description);

            if iter.peek().is_some() {
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

fn format_type_description(input: &str) -> String {
    fn add_indentation(output: &mut String, indent_level: i32) {
        for _ in 0..indent_level {
            output.push_str("    ");
        }
    }

    let mut output = String::new();
    let mut indent_level = 0;
    // in a tuple we will not set line breaks on comma, so we keep track of it here:
    let mut in_tuple = 0;
    let mut tokens_since_last_bracket_or_comma: usize = 0;
    for ch in input.chars() {
        let mut token_is_bracket_or_comma = true;
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
                // makes small tuples e.g. (u8, u16, u8, u8) not cause line breaks.
                if in_tuple > 0 && tokens_since_last_bracket_or_comma < 5 {
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
                token_is_bracket_or_comma = false;
                output.push(ch);
            }
        }
        if token_is_bracket_or_comma {
            tokens_since_last_bracket_or_comma = 0;
        } else {
            tokens_since_last_bracket_or_comma += 1;
        }
    }
    output
}

#[cfg(test)]
mod test {
    use crate::utils::type_description::print_type_description;
    use scale_info::scale::{Decode, Encode};
    use scale_info::TypeInfo;
    use std::fmt::Write;
    use std::write;

    #[derive(Encode, Decode, Debug, Clone, TypeInfo)]
    pub struct Foo {
        hello: String,
        num: i32,
    }

    /// Given a type definition, return type ID and registry representing it.
    fn make_type<T: scale_info::TypeInfo + 'static>() -> (u32, scale_info::PortableRegistry) {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: scale_info::PortableRegistry = types.into();
        (id.id, portable_registry)
    }

    #[test]
    fn test_type_description() {
        let (foo_type_id, foo_registry) = make_type::<Foo>();
        let description = print_type_description(&foo_type_id, &foo_registry).unwrap();
        let mut output = String::new();
        writeln!(output, "struct Foo {{").unwrap();
        writeln!(output, "    hello: String,").unwrap();
        writeln!(output, "    num: i32").unwrap();
        write!(output, "}}").unwrap();
        assert_eq!(description, output);
    }
}
