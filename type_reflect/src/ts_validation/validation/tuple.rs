use ts_quote::ts_string;

use crate::ts_validation::validation::type_validation;

pub fn tuple_validation(var_name: &str, members: &Vec<type_reflect_core::Type>) -> String {
    let member_validations: Vec<String> = members
        .iter()
        .enumerate()
        .map(|(i, member_type)| {
            let member_var = format!("{}[{}]", var_name, i);
            type_validation(&member_var, member_type)
        })
        .collect();

    let member_validations = member_validations.join("\n");

    ts_string! {
        if (!Array.isArray(#var_name)) {
            throw new Error(# "`Error parsing #var_name: expected: Array, found: ${ typeof #var_name }`");
        }
        #member_validations
    }
    .to_string()
}