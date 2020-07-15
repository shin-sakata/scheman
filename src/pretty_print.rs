use dprint_plugin_typescript as ty;
use dprint_plugin_typescript::configuration::*;
use std::path::PathBuf;

pub fn pretty(source: &str) -> Result<String, String> {
    let config = ConfigurationBuilder::new().line_width(80).build();
    let formatter = ty::Formatter::new(config);

    formatter.format_text(&PathBuf::default(), source)
}
