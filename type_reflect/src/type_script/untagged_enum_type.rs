use crate::EnumReflectionType;
use ts_quote::ts_string;
use type_reflect_core::{EnumCase, Type, TypeFieldsDefinition};

use super::{to_ts_type, union_case_type_name};

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

    let non_union_cases: Vec<&EnumCase> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Unit = c.type_ {
                false
            } else {
                true
            }
        })
        .collect();

    let unit_cases: Vec<String> = unit_cases
        .iter()
        .map(|case| emit_unit_case(&case))
        .collect();

    let unit_cases: Option<String> = if unit_cases.is_empty() {
        None
    } else {
        Some(unit_cases.join(" | "))
    };


    let member_cases: Vec<String> = non_union_cases
        .iter()
        .map(|case| emit_member_case(&case, T::name()))
        .collect();

    let member_cases_block = if member_cases.is_empty() {
        None
    } else {
        let cases = member_cases.join(", ");
        Some(ts_string! {
            {
                #cases
            }
        })
    };

    match (unit_cases, member_cases_block) {
        (None, None) => ts_string! {
            export type #name = never
        }
        (None, Some(members)) => ts_string! {
            export type #name = #members
        }
        (Some(units), None) => ts_string! {
            export type #name = #units
        }
        (Some(units), Some(members)) => ts_string! {
            export type #name = #units | #members
        }
    }
}

fn emit_unit_case(case: &EnumCase) -> String {
    let name = &case.name;
    ts_string! { #name }
}

fn emit_member_case(case: &EnumCase, parent_name: &str) -> String {
    let name = &case.name;
    let member_type = emit_case_type_name(&case, parent_name);
    ts_string! { #name : #member_type }
}

fn emit_case_type_name(case: &EnumCase, parent_name: &str) -> String {
    match case.type_ {
        TypeFieldsDefinition::Unit => unreachable!("unit cases don't have a a case type"),
        TypeFieldsDefinition::Tuple(items) => emit_tuple_case_type_name(&case, &items, parent_name),
        TypeFieldsDefinition::Named(named_fields) => union_case_type_name(case, parent_name),
    }
}

fn emit_tuple_case_type_name(
    case: &EnumCase,
    tuple_fields: &Vec<Type>,
    parent_name: &str,
) -> String {
    if let Some(field) = tuple_fields.first()
        && tuple_fields.len() == 1
    {
        to_ts_type(&field)
    } else {
        union_case_type_name(case, parent_name)
    }
}

fn emit_case_type(case: &EnumCase, parent_name: &str) -> String {
    let name = emit_case_type_name(case, parent_name);
    let contents = emit_case_type_contents(case);

    ts_string! {
        export type #name = #contents;
    }
}

fn emit_case_type_contents(case: &EnumCase) -> String {
    match case.type_ {
        TypeFieldsDefinition::Unit => unreachable!("unit cases don't have a a case type"),
        TypeFieldsDefinition::Tuple(items) => emit_tuple_case_type_name(&case, &items, parent_name),
        TypeFieldsDefinition::Named(named_fields) => union_case_type_name(case, parent_name),
    }
}

fn emit_tuple_case_type_contentns(
    case: &EnumCase,
    tuple_fields: &Vec<Type>,
    parent_name: &str,
) -> String {
    if let Some(field) = tuple_fields.first()
        && tuple_fields.len() == 1
    {
        to_ts_type(&field)
    } else {
        let members: Vec<String> = tuple_fields.map(|field| to_ts_type(&field)).collect();
        let members = members.join(", ");

        ts_string! {
            [ #members ]
        }
    }
}
