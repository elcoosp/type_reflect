use std::path::Path;

use deno_ast::{parse_module, SourceTextInfo, ModuleSpecifier};
use dprint_plugin_typescript::{
    configuration::{Configuration, ConfigurationBuilder, NextControlFlowPosition, QuoteStyle},
    format_text, FormatTextOptions,
};
pub use ts_quote_macros::ts_quote;
pub use ts_quote_macros::ts_string;

pub use deno_ast::ParsedSource as TS;

/**
The TSSource trait is used to add a few convenience methods to the  deno_ast::ParsedSource type.
**/
pub trait TSSource: Sized {
    /**
    Creates a ParsedSource instance from a string.

    # Arguments:

    * `source` - A TypeScript source string

    # Returns

    Returns a ParsedSource, or an error if source is not valid TypeScript
    **/
    fn from_source(source: String) -> anyhow::Result<Self>;

    /**
    Returns a formatted TypeScript string.

    # Arguments:

    * `config` - Optional: a `dprint_plugin_typescript` config used for formatting the output.

    If no config is provided, the function will output using the default config:
    - `line_width`: `80`
    - `indent_width`: `2`
    - `prefer_hanging`: `true`
    - `prefer_single_line`: `false`
    - `quote_style`: `QuoteStyle::PreferSingle`
    - `next_control_flow_position`: `NextControlFlowPosition::SameLine`

    # Returns

    Returns a formatted string, or an error if formatting fails
    **/
    fn formatted(&self, config: Option<&Configuration>) -> anyhow::Result<String>;
}

impl TSSource for TS {
    fn from_source(source: String) -> anyhow::Result<Self> {
        let parsed = parse_module(deno_ast::ParseParams {
            specifier: ModuleSpecifier::parse("file:///dummy.ts").unwrap(),
            text: source.into(), // Convert String to Arc<str>
            media_type: deno_ast::MediaType::TypeScript,
            capture_tokens: true,
            scope_analysis: false,
            maybe_syntax: None,
        })?;
        Ok(parsed)
    }

    fn formatted(&self, config: Option<&Configuration>) -> anyhow::Result<String> {
        let source_text = self.text_info_lazy().text_str().to_string();
        let path = Path::new("file.ts");
        
        match config {
            Some(config) => {
                let options = FormatTextOptions {
                    path,
                    extension: Some("ts"),
                    text: source_text,
                    config,
                    external_formatter: None,
                };
                Ok(format_text(options)?.unwrap_or(String::new()))
            },
            None => {
                let config = ConfigurationBuilder::new()
                    .indent_width(2)
                    .line_width(80)
                    .prefer_hanging(true)
                    .prefer_single_line(false)
                    .quote_style(QuoteStyle::PreferSingle)
                    .next_control_flow_position(NextControlFlowPosition::SameLine)
                    .build();

                let options = FormatTextOptions {
                    path,
                    extension: Some("ts"),
                    text: source_text,
                    config: &config,
                    external_formatter: None,
                };
                Ok(format_text(options)?.unwrap_or(String::new()))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_source_from_string() -> anyhow::Result<()> {
        let ts: TS = TS::from_source("let a = 1; let b = 2;".to_string())?;

        let output = ts.formatted(None)?;

        println!("output:");
        println!("{}", output);

        assert_eq!(output.as_str(), "let a = 1;\nlet b = 2;\n");

        Ok(())
    }
}