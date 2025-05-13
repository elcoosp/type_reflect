use type_reflect_core::EnumType;
use untagged::emit_untagged_enum_type;

use crate::{ts_validation::validation_namespace, EnumReflectionType};

mod complex;
use complex::*;

mod case_type;

mod untagged;

pub fn emit_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    match T::enum_type() {
        EnumType::Simple => emit_simple_enum_type::<T>(),
        EnumType::Complex {
            case_key,
            content_key,
        } => emit_complex_enum_type::<T>(&case_key, &content_key),
        EnumType::Untagged => emit_untagged_enum_type::<T>(),
    }
}

fn emit_simple_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let validation_impl = format!(
        r#"
if(Object.values({name}).includes(input as {name})) {{
    return input as {name};
}}
throw new Error(`Error parsing {name}: value does not conform: ${{JSON.stringify(input)}}`)
"#,
        name = T::name(),
    );
    validation_namespace(T::name(), validation_impl.as_str())
}
