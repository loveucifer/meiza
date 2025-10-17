use std::collections::HashMap;

pub mod parser;
pub mod layout;
pub mod renderer;
pub mod components;
pub mod validator;
pub mod netlist;

pub use parser::{parse_cdl, AstNode, Circuit};
pub use renderer::{render_to_svg, SvgTheme, SvgStyle};

/// Main entry point for parsing CDL and rendering to SVG
pub fn parse_and_render(cdl_text: &str, theme: SvgTheme, style: SvgStyle) -> Result<String, Box<dyn std::error::Error>> {
    let circuit = parser::parse_cdl(cdl_text)?;
    let layout = layout::calculate_layout(&circuit)?;
    let svg = renderer::render_to_svg(&layout, theme, style)?;
    Ok(svg)
}

/// Validate CDL circuit
pub fn validate_circuit(cdl_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let circuit = parser::parse_cdl(cdl_text)?;
    validator::validate(&circuit)
}

/// Export to SPICE netlist
pub fn export_spice(cdl_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let circuit = parser::parse_cdl(cdl_text)?;
    netlist::export_spice(&circuit)
}

/// Get list of available components
pub fn list_components() -> Vec<String> {
    components::get_component_list()
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    use super::*;

    #[wasm_bindgen]
    pub fn parse_cdl_to_svg(cdl_text: &str, theme: &str, style: &str) -> Result<String, JsValue> {
        let theme = match theme {
            "dark" => SvgTheme::Dark,
            _ => SvgTheme::Light,
        };
        
        let style = match style {
            "iec" => SvgStyle::Iec,
            "din" => SvgStyle::Din,
            _ => SvgStyle::Ieee,
        };

        let result = parse_and_render(cdl_text, theme, style);
        result.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn validate_cdl(cdl_text: &str) -> Result<(), JsValue> {
        let result = validate_circuit(cdl_text);
        result.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn list_available_components() -> js_sys::Array {
        let components = list_components();
        let js_array = js_sys::Array::new();
        for component in components {
            js_array.push(&JsValue::from_str(&component));
        }
        js_array
    }
}