use crate::EnumReflectionType;
use ts_quote::ts_string;
use type_reflect_core::{EnumCase, TypeFieldsDefinition};

pub fn emit_untaggedd_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let cases = T::cases();

    let unit_cases: Vec<&EnumCase> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Unit = c.type_ {
                true
            } else {
                false
            }
        })
        .collect();

    let named_cases: Vec<&EnumCase> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Named(_) = c.type_ {
                true
            } else {
                false
            }
        })
        .collect();

    let tuple_cases: Vec<&EnumCase> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Tuple(_) = c.type_ {
                true
            } else {
                false
            }
        })
        .collect();

    let unit_cases: Vec<String> = unit_cases
        .iter()
        .map(|case| emit_unit_case(&case))
        .collect();

    let unit_cases = unit_cases.join(" | ");

    ts_string! {
        export type #name = #unit_cases
    }
}

fn emit_unit_case(case: &EnumCase) -> String {
    let name = &case.name;
    ts_string! { #name }
}
