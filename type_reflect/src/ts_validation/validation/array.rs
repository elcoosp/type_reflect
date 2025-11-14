use ts_quote::ts_string;

use crate::ts_validation::validation::type_validation;

pub fn array_validation(var_name: &str, item_type: &type_reflect_core::Type) -> String {
    let item_validation = type_validation(&format!("{var_name}[i]"), item_type);

    ts_string! {
        if (!Array.isArray(#var_name)) {
            throw new Error(# "`Error parsing #var_name: expected: Array, found: ${ typeof #var_name }`");
        }
        for (let i = 0; i < #var_name.length; i++) {
            #item_validation
        }
    }
    .to_string()
}