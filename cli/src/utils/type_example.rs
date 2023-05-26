use color_eyre::eyre::eyre;

use scale_info::{
    form::PortableForm, Field, PortableRegistry, TypeDef, TypeDefArray, TypeDefPrimitive,
    TypeDefTuple, TypeDefVariant,
};
use scale_value::{Value, ValueDef};
use std::fmt::Write;
use std::write;

pub fn print_type_examples<T>(
    ty: &T,
    registry: &PortableRegistry,
    type_placeholder: &str,
) -> color_eyre::Result<String>
where
    T: TypeExample,
{
    let type_examples = ty.type_example(registry)?;
    let mut output = String::new();
    match type_examples.len() {
        0 => {
            write!(
                output,
                "There are no examples available for a {type_placeholder} matching this shape:"
            )?;
        }
        1 => {
            write!(
                output,
                "Here is an example of a {type_placeholder} matching this shape:"
            )?;
        }
        i => {
            write!(
                output,
                "Here are {i} examples of a {type_placeholder} matching this shape:"
            )?;
        }
    };
    for self_value in type_examples {
        let value = <T as TypeExample>::upcast(self_value);
        let example_str = scale_value::stringify::to_string(&value);
        write!(output, "\n{}", example_str)?;
    }
    Ok(output)
}

/// a trait for producing scale value examples for a type.
pub trait TypeExample {
    type Value;
    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>>;
    fn upcast(self_value: Self::Value) -> scale_value::Value;
}

impl TypeExample for u32 {
    type Value = scale_value::Value;

    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        let ty = registry
            .resolve(*self)
            .ok_or(eyre!("Type with id {} not found in registry", *self))?;

        let examples = match &ty.type_def {
            TypeDef::Composite(composite) => composite
                .fields
                .type_example(registry)?
                .into_iter()
                .map(|e| scale_value::Value {
                    value: scale_value::ValueDef::Composite(e),
                    context: (),
                })
                .collect(),
            TypeDef::Variant(variant) => variant
                .type_example(registry)?
                .into_iter()
                .map(|e| scale_value::Value {
                    value: scale_value::ValueDef::Variant(e),
                    context: (),
                })
                .collect(),
            TypeDef::Array(array) => array
                .type_example(registry)?
                .into_iter()
                .map(|e| scale_value::Value {
                    value: scale_value::ValueDef::Composite(e),
                    context: (),
                })
                .collect(),
            TypeDef::Tuple(tuple) => tuple
                .type_example(registry)?
                .into_iter()
                .map(|e| scale_value::Value {
                    value: scale_value::ValueDef::Composite(e),
                    context: (),
                })
                .collect(),
            TypeDef::Primitive(primitive) => primitive
                .type_example(registry)?
                .into_iter()
                .map(scale_value::Value::primitive)
                .collect(),
            TypeDef::Compact(compact) => compact.type_param.id.type_example(registry)?,
            TypeDef::BitSequence(_) => {
                return Err(eyre!("no examples for BitSequence available"));
            }
            TypeDef::Sequence(sequence) => {
                // for sequences we just give an example of an array with 3 elements:
                TypeDefArray {
                    len: 3,
                    type_param: sequence.type_param,
                }
                .type_example(registry)?
                .into_iter()
                .map(|e| scale_value::Value {
                    value: scale_value::ValueDef::Composite(e),
                    context: (),
                })
                .collect()
            }
        };
        Ok(examples)
    }

    fn upcast(self_value: Self::Value) -> Value {
        self_value
    }
}

impl TypeExample for TypeDefVariant<PortableForm> {
    type Value = scale_value::Variant<()>;

    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        let mut examples: Vec<scale_value::Variant<()>> = Vec::new();

        // returns one example for each variant
        for variant in &self.variants {
            // get the first example for the variant's data and use it
            let mut variant_value_examples = variant.fields.type_example(registry)?;
            let Some(values) = variant_value_examples.pop() else {
                return Err(eyre!("no example element for variant {}", variant.name));
            };

            examples.push(scale_value::Variant {
                name: variant.name.clone(),
                values,
            });
        }

        Ok(examples)
    }

    fn upcast(self_value: Self::Value) -> Value {
        Value {
            value: ValueDef::Variant(self_value),
            context: (),
        }
    }
}

impl TypeExample for TypeDefArray<PortableForm> {
    type Value = scale_value::Composite<()>;

    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        // take the first example value and set it to each element of the array
        let mut value_examples = self.type_param.id.type_example(registry)?;
        let Some(first_value_example) = value_examples.pop() else {
            return Err(eyre!("no example element for array"));
        };

        let one_example = {
            let mut values = Vec::with_capacity(self.len as usize);
            for _ in 0..self.len as usize {
                values.push(first_value_example.clone());
            }
            scale_value::Composite::<()>::Unnamed(values)
        };
        Ok(vec![one_example])
    }

    fn upcast(self_value: Self::Value) -> Value {
        Value {
            value: ValueDef::Composite(self_value),
            context: (),
        }
    }
}

impl TypeExample for TypeDefTuple<PortableForm> {
    type Value = scale_value::Composite<()>;

    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        // create unnamed fields to use the same logic already used for struct example generation
        let fields_vector: Vec<Field<PortableForm>> = self
            .fields
            .iter()
            .map(|ty| Field {
                name: None,
                ty: *ty,
                type_name: None,
                docs: Vec::new(),
            })
            .collect();
        fields_vector.type_example(registry)
    }

    fn upcast(self_value: Self::Value) -> Value {
        Value {
            value: ValueDef::Composite(self_value),
            context: (),
        }
    }
}

impl TypeExample for Vec<Field<PortableForm>> {
    type Value = scale_value::Composite<()>;

    fn type_example(&self, registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        let all_fields_named = self.iter().all(|f| f.name.is_some());
        let all_fields_unnamed = self.iter().all(|f| f.name.is_none());
        // composite apparently has no fields:
        if all_fields_named && all_fields_unnamed {
            let one_empty_example = scale_value::Composite::Unnamed(Vec::new());
            return Ok(vec![one_empty_example]);
        }

        // composite apparently has mix of named and unnamed fields:
        if !all_fields_named && !all_fields_unnamed {
            return Err(eyre!(
                "combination of named and unnamed fields in compound type"
            ));
        }

        // for each field get all the examples the type of that field can offer:
        let mut field_examples: Vec<(&Field<PortableForm>, Vec<scale_value::Value>)> = Vec::new();
        for field in self.iter() {
            let examples = field.ty.id.type_example(registry)?;
            field_examples.push((field, examples));
        }

        // Let N be the mininum number of examples any field has.
        // Return N examples for the Compound type, by choosing the ith example for each of the 0..N examples for that field.
        let n = field_examples
            .iter()
            .map(|(_, examples)| examples.len())
            .min()
            .expect("Iterator is not non-empty checked above; qed");
        let mut composite_examples: Vec<Vec<(&Field<PortableForm>, scale_value::Value)>> =
            Vec::new();
        for _ in 0..n {
            let composite_example: Vec<(&Field<PortableForm>, scale_value::Value)> = field_examples
                .iter_mut()
                .map(|(field, examples)| (*field, examples.pop().unwrap()))
                .collect(); // the pop() is safe to unwrap because of the minimum we checked before
            composite_examples.push(composite_example);
        }

        // create the vector of composite scale values. Distingiush between named and unnamed here.
        let composite_examples = composite_examples
            .into_iter()
            .map(|composite_example| {
                if all_fields_named {
                    let composite_example = composite_example
                        .into_iter()
                        .map(|(field, value)| (field.name.as_ref().unwrap().clone(), value))
                        .collect();
                    scale_value::Composite::Named(composite_example)
                } else {
                    let composite_example = composite_example
                        .into_iter()
                        .map(|(_, value)| (value))
                        .collect();
                    scale_value::Composite::Unnamed(composite_example)
                }
            })
            .collect();
        Ok(composite_examples)
    }

    fn upcast(self_value: Self::Value) -> Value {
        Value {
            value: ValueDef::Composite(self_value),
            context: (),
        }
    }
}

/// 3-4 example values for each primitive
impl TypeExample for TypeDefPrimitive {
    type Value = scale_value::Primitive;

    fn type_example(&self, _registry: &PortableRegistry) -> color_eyre::Result<Vec<Self::Value>> {
        let value = match &self {
            TypeDefPrimitive::Bool => vec![
                scale_value::Primitive::Bool(true),
                scale_value::Primitive::Bool(false),
            ],
            TypeDefPrimitive::Char => vec![
                scale_value::Primitive::Char('r'),
                scale_value::Primitive::Char('u'),
                scale_value::Primitive::Char('s'),
                scale_value::Primitive::Char('t'),
            ],
            TypeDefPrimitive::Str => vec![
                scale_value::Primitive::String("Alice".into()),
                scale_value::Primitive::String("Bob".into()),
                scale_value::Primitive::String("Foo".into()),
                scale_value::Primitive::String("Bar".into()),
            ],
            TypeDefPrimitive::U8 => vec![
                scale_value::Primitive::U128(u8::MIN as u128),
                scale_value::Primitive::U128(69),
                scale_value::Primitive::U128(u8::MAX as u128),
            ],
            TypeDefPrimitive::U16 => vec![
                scale_value::Primitive::U128(u16::MIN as u128),
                scale_value::Primitive::U128(420),
                scale_value::Primitive::U128(u16::MAX as u128),
            ],
            TypeDefPrimitive::U32 => vec![
                scale_value::Primitive::U128(u32::MIN as u128),
                scale_value::Primitive::U128(99000),
                scale_value::Primitive::U128(u32::MAX as u128),
            ],
            TypeDefPrimitive::U64 => vec![
                scale_value::Primitive::U128(u64::MIN as u128),
                scale_value::Primitive::U128(99000),
                scale_value::Primitive::U128(u64::MAX as u128),
            ],
            TypeDefPrimitive::U128 => vec![
                scale_value::Primitive::U128(u128::MIN),
                scale_value::Primitive::U128(99000),
                scale_value::Primitive::U128(u128::MAX),
            ],
            TypeDefPrimitive::U256 => vec![
                scale_value::Primitive::U256([u8::MIN; 32]),
                scale_value::Primitive::U256([3; 32]),
                scale_value::Primitive::U256([u8::MAX; 32]),
            ],
            TypeDefPrimitive::I8 => vec![
                scale_value::Primitive::I128(i8::MIN as i128),
                scale_value::Primitive::I128(69),
                scale_value::Primitive::I128(i8::MAX as i128),
            ],
            TypeDefPrimitive::I16 => vec![
                scale_value::Primitive::I128(i16::MIN as i128),
                scale_value::Primitive::I128(420),
                scale_value::Primitive::I128(i16::MAX as i128),
            ],
            TypeDefPrimitive::I32 => vec![
                scale_value::Primitive::I128(i32::MIN as i128),
                scale_value::Primitive::I128(99000),
                scale_value::Primitive::I128(i32::MAX as i128),
            ],
            TypeDefPrimitive::I64 => vec![
                scale_value::Primitive::I128(i64::MIN as i128),
                scale_value::Primitive::I128(99000),
                scale_value::Primitive::I128(i64::MAX as i128),
            ],
            TypeDefPrimitive::I128 => vec![
                scale_value::Primitive::I128(i128::MIN),
                scale_value::Primitive::I128(99000),
                scale_value::Primitive::I128(i128::MAX),
            ],
            TypeDefPrimitive::I256 => vec![
                scale_value::Primitive::I256([u8::MIN; 32]),
                scale_value::Primitive::I256([3; 32]),
                scale_value::Primitive::I256([u8::MAX; 32]),
            ],
        };
        Ok(value)
    }

    fn upcast(self_value: Self::Value) -> Value {
        Value {
            value: ValueDef::Primitive(self_value),
            context: (),
        }
    }
}
