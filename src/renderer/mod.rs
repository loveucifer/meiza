use std::collections::HashMap;
use crate::layout::{Layout, PositionedComponent, PositionedConnection, Point, Rotation};

#[derive(Debug, Clone)]
pub enum SvgTheme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum SvgStyle {
    Ieee,
    Iec,
    Din,
}

pub fn render_to_svg(
    layout: &Layout,
    theme: SvgTheme,
    style: SvgStyle,
) -> Result<String, Box<dyn std::error::Error>> {
    let theme_colors = get_theme_colors(&theme);
    let style_class = match style {
        SvgStyle::Ieee => "ieee",
        SvgStyle::Iec => "iec",
        SvgStyle::Din => "din",
    };
    
    let mut svg = String::new();
    svg.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"{}\" style=\"background-color: {}; color: {};\">\n", 
        style_class, theme_colors.background, theme_colors.text));
    
    // Add styles
    svg.push_str("<style>\n");
    svg.push_str(&format!(
        ".wire {{ stroke: {}; stroke-width: 2; fill: none; }}\n", 
        theme_colors.wire
    ));
    svg.push_str(&format!(
        ".component {{ stroke: {}; stroke-width: 2; fill: {}; }}\n", 
        theme_colors.component_stroke, theme_colors.component_fill
    ));
    svg.push_str(&format!(
        ".text {{ fill: {}; font-family: Arial, sans-serif; font-size: 12px; }}\n", 
        theme_colors.text
    ));
    svg.push_str("</style>\n");
    
    // Render components
    for component in &layout.components {
        svg.push_str(&render_component(component, &theme_colors, &style)?);
    }
    
    // Render connections (wires)
    for connection in &layout.connections {
        svg.push_str(&render_connection(connection, &theme_colors)?);
    }
    
    svg.push_str("</svg>");
    Ok(svg)
}

struct ThemeColors {
    background: &'static str,
    text: &'static str,
    wire: &'static str,
    component_stroke: &'static str,
    component_fill: &'static str,
}

fn get_theme_colors(theme: &SvgTheme) -> ThemeColors {
    match theme {
        SvgTheme::Light => ThemeColors {
            background: "#ffffff",
            text: "#000000",
            wire: "#000000",
            component_stroke: "#000000",
            component_fill: "#ffffff",
        },
        SvgTheme::Dark => ThemeColors {
            background: "#1e1e1e",
            text: "#ffffff",
            wire: "#cccccc",
            component_stroke: "#cccccc",
            component_fill: "#1e1e1e",
        },
    }
}

fn render_component(
    component: &PositionedComponent,
    colors: &ThemeColors,
    style: &SvgStyle,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut svg = String::new();
    
    // Determine which symbol to use based on the component type and style
    let symbol_path = get_component_symbol(&component.component.component_type, style)?;
    
    // Apply transformations for position and rotation
    let transform = format!(
        "translate({} {}) rotate({} {} {})",
        component.position.x,
        component.position.y,
        rotation_to_degrees(&component.rotation),
        0.0, // rotation center x
        0.0  // rotation center y
    );
    
    svg.push_str(&format!(
        "<g transform=\"{}\">\n",
        transform
    ));
    
    // Draw the component symbol
    svg.push_str(&format!(
        "<path d=\"{}\" class=\"component\" />\n",
        symbol_path
    ));
    
    // Add component label if available
    if let Some(ref label) = component.component.label {
        svg.push_str(&format!(
            "<text x=\"0\" y=\"-20\" class=\"text\" text-anchor=\"middle\">{}</text>\n",
            label
        ));
    }
    
    // Add component value if available
    if let Some(ref value) = component.component.value {
        svg.push_str(&format!(
            "<text x=\"0\" y=\"20\" class=\"text\" text-anchor=\"middle\">{}</text>\n",
            value
        ));
    }
    
    svg.push_str("</g>\n");
    Ok(svg)
}

fn get_component_symbol(
    component_type: &crate::parser::ComponentType,
    style: &SvgStyle,
) -> Result<String, Box<dyn std::error::Error>> {
    let symbol = match component_type {
        crate::parser::ComponentType::Resistor => {
            match style {
                SvgStyle::Ieee => "M -20 0 L -15 -5 L -5 -5 L 5 5 L 15 5 L 20 0",
                SvgStyle::Iec => "M -20 0 H 20 V 5 H -20 V -5 H 20",
                SvgStyle::Din => "M -20 0 L -15 -5 L -5 -5 L 5 5 L 15 5 L 20 0",
            }
        },
        crate::parser::ComponentType::Capacitor => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 -10 M -5 -10 L 10 -10 M -5 10 L 10 10 M 0 10 L 0 20",
                SvgStyle::Iec => "M -5 -20 V 20 M 5 -20 V 20",
                SvgStyle::Din => "M 0 -20 L 0 -10 M -5 -10 L 10 -10 M -5 10 L 10 10 M 0 10 L 0 20",
            }
        },
        crate::parser::ComponentType::Inductor => {
            match style {
                SvgStyle::Ieee => "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0",
                SvgStyle::Iec => "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0",
                SvgStyle::Din => "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0",
            }
        },
        crate::parser::ComponentType::Potentiometer => {
            match style {
                SvgStyle::Ieee => "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10",
                SvgStyle::Iec => "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10",
                SvgStyle::Din => "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10",
            }
        },
        crate::parser::ComponentType::Transformer => {
            match style {
                SvgStyle::Ieee => "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10",
                SvgStyle::Iec => "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10",
                SvgStyle::Din => "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10",
            }
        },
        crate::parser::ComponentType::DcVoltage => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10",
                SvgStyle::Iec => "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10",
                SvgStyle::Din => "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10",
            }
        },
        crate::parser::ComponentType::DcCurrent => {
            match style {
                SvgStyle::Ieee => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5",
                SvgStyle::Iec => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5",
                SvgStyle::Din => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5",
            }
        },
        crate::parser::ComponentType::AcVoltage => {
            match style {
                SvgStyle::Ieee => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
                SvgStyle::Iec => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
                SvgStyle::Din => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
            }
        },
        crate::parser::ComponentType::AcCurrent => {
            match style {
                SvgStyle::Ieee => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
                SvgStyle::Iec => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
                SvgStyle::Din => "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0",
            }
        },
        crate::parser::ComponentType::SignalGenerator => {
            match style {
                SvgStyle::Ieee => "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0",
                SvgStyle::Iec => "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0",
                SvgStyle::Din => "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0",
            }
        },
        crate::parser::ComponentType::Diode => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0",
                SvgStyle::Iec => "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0",
                SvgStyle::Din => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0",
            }
        },
        crate::parser::ComponentType::ZenerDiode => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -2 -10 L -6 -6 M 6 6 L 2 2",
                SvgStyle::Iec => "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0",
                SvgStyle::Din => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -2 -10 L -6 -6 M 6 6 L 2 2",
            }
        },
        crate::parser::ComponentType::SchottkyDiode => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -8 0 L -10 0 L -10 2 M 10 -2 L 10 0 L 8 0",
                SvgStyle::Iec => "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0",
                SvgStyle::Din => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -8 0 L -10 0 L -10 2 M 10 -2 L 10 0 L 8 0",
            }
        },
        crate::parser::ComponentType::Led => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32",
                SvgStyle::Iec => "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32",
                SvgStyle::Din => "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32",
            }
        },
        crate::parser::ComponentType::NpnTransistor => {
            match style {
                SvgStyle::Ieee => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0",
                SvgStyle::Iec => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0",
                SvgStyle::Din => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0",
            }
        },
        crate::parser::ComponentType::PnpTransistor => {
            match style {
                SvgStyle::Ieee => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0",
                SvgStyle::Iec => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0",
                SvgStyle::Din => "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0",
            }
        },
        crate::parser::ComponentType::NmosTransistor => {
            match style {
                SvgStyle::Ieee => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13",
                SvgStyle::Iec => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13",
                SvgStyle::Din => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13",
            }
        },
        crate::parser::ComponentType::PmosTransistor => {
            match style {
                SvgStyle::Ieee => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7",
                SvgStyle::Iec => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7",
                SvgStyle::Din => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7",
            }
        },
        crate::parser::ComponentType::Jfet => {
            match style {
                SvgStyle::Ieee => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5",
                SvgStyle::Iec => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5",
                SvgStyle::Din => "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5",
            }
        },
        crate::parser::ComponentType::OpAmp => {
            match style {
                SvgStyle::Ieee => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
                SvgStyle::Iec => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
                SvgStyle::Din => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
            }
        },
        crate::parser::ComponentType::Comparator => {
            match style {
                SvgStyle::Ieee => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
                SvgStyle::Iec => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
                SvgStyle::Din => "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0",
            }
        },
        crate::parser::ComponentType::Timer555 => {
            match style {
                SvgStyle::Ieee => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Iec => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Din => "M -30 -25 H 30 V 50 H -30 Z",
            }
        },
        crate::parser::ComponentType::AndGate => {
            match style {
                SvgStyle::Ieee => "M -10 -15 A 15 15 0 0 1 0 -15 A 15 15 0 0 1 10 -15 V 15 A 15 15 0 0 1 0 15 A 15 15 0 0 1 -10 15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
                SvgStyle::Iec => "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
                SvgStyle::Din => "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
            }
        },
        crate::parser::ComponentType::OrGate => {
            match style {
                SvgStyle::Ieee => "M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
                SvgStyle::Iec => "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
                SvgStyle::Din => "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0",
            }
        },
        crate::parser::ComponentType::NotGate => {
            match style {
                SvgStyle::Ieee => "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0",
                SvgStyle::Iec => "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0",
                SvgStyle::Din => "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0",
            }
        },
        crate::parser::ComponentType::NandGate => {
            match style {
                SvgStyle::Ieee => "M -10 -15 A 15 15 0 0 1 0 -15 A 15 15 0 0 1 10 -15 V 15 A 15 15 0 0 1 0 15 A 15 15 0 0 1 -10 15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
                SvgStyle::Iec => "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
                SvgStyle::Din => "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
            }
        },
        crate::parser::ComponentType::NorGate => {
            match style {
                SvgStyle::Ieee => "M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
                SvgStyle::Iec => "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
                SvgStyle::Din => "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2",
            }
        },
        crate::parser::ComponentType::XorGate => {
            match style {
                SvgStyle::Ieee => "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z",
                SvgStyle::Iec => "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z",
                SvgStyle::Din => "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z",
            }
        },
        crate::parser::ComponentType::FlipFlop => {
            match style {
                SvgStyle::Ieee => "M -25 -20 H 25 V 40 H -25 Z",
                SvgStyle::Iec => "M -25 -20 H 25 V 40 H -25 Z",
                SvgStyle::Din => "M -25 -20 H 25 V 40 H -25 Z",
            }
        },
        crate::parser::ComponentType::Counter => {
            match style {
                SvgStyle::Ieee => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Iec => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Din => "M -30 -25 H 30 V 50 H -30 Z",
            }
        },
        crate::parser::ComponentType::Multiplexer => {
            match style {
                SvgStyle::Ieee => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Iec => "M -30 -25 H 30 V 50 H -30 Z",
                SvgStyle::Din => "M -30 -25 H 30 V 50 H -30 Z",
            }
        },
        crate::parser::ComponentType::VoltageRegulator => {
            match style {
                SvgStyle::Ieee => "M -20 -10 H 20 V 20 H -20 Z",
                SvgStyle::Iec => "M -20 -10 H 20 V 20 H -20 Z",
                SvgStyle::Din => "M -20 -10 H 20 V 20 H -20 Z",
            }
        },
        crate::parser::ComponentType::Crystal => {
            match style {
                SvgStyle::Ieee => "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15",
                SvgStyle::Iec => "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15",
                SvgStyle::Din => "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15",
            }
        },
        crate::parser::ComponentType::Relay => {
            match style {
                SvgStyle::Ieee => "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5",
                SvgStyle::Iec => "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5",
                SvgStyle::Din => "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5",
            }
        },
        crate::parser::ComponentType::SpstSwitch => {
            match style {
                SvgStyle::Ieee => "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5",
                SvgStyle::Iec => "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5",
                SvgStyle::Din => "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5",
            }
        },
        crate::parser::ComponentType::SpdtSwitch => {
            match style {
                SvgStyle::Ieee => "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0",
                SvgStyle::Iec => "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0",
                SvgStyle::Din => "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0",
            }
        },
        crate::parser::ComponentType::DpdtSwitch => {
            match style {
                SvgStyle::Ieee => "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15",
                SvgStyle::Iec => "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15",
                SvgStyle::Din => "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15",
            }
        },
        crate::parser::ComponentType::Fuse => {
            match style {
                SvgStyle::Ieee => "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10",
                SvgStyle::Iec => "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10",
                SvgStyle::Din => "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10",
            }
        },
        crate::parser::ComponentType::Battery => {
            match style {
                SvgStyle::Ieee => "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0",
                SvgStyle::Iec => "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0",
                SvgStyle::Din => "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0",
            }
        },
        crate::parser::ComponentType::Microcontroller => {
            match style {
                SvgStyle::Ieee => "M -40 -40 H 40 V 80 H -40 Z",
                SvgStyle::Iec => "M -40 -40 H 40 V 80 H -40 Z",
                SvgStyle::Din => "M -40 -40 H 40 V 80 H -40 Z",
            }
        },
        crate::parser::ComponentType::Connector => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 15 -20 L 15 20 L 0 20",
                SvgStyle::Iec => "M 0 -20 L 15 -20 L 15 20 L 0 20",
                SvgStyle::Din => "M 0 -20 L 15 -20 L 15 20 L 0 20",
            }
        },
        crate::parser::ComponentType::TestPoint => {
            match style {
                SvgStyle::Ieee => "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z",
                SvgStyle::Iec => "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z",
                SvgStyle::Din => "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z",
            }
        },
        crate::parser::ComponentType::Ammeter => {
            match style {
                SvgStyle::Ieee => "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5",
                SvgStyle::Iec => "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5",
                SvgStyle::Din => "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5",
            }
        },
        crate::parser::ComponentType::Voltmeter => {
            match style {
                SvgStyle::Ieee => "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5",
                SvgStyle::Iec => "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5",
                SvgStyle::Din => "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5",
            }
        },
        crate::parser::ComponentType::OscilloscopeProbe => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z",
                SvgStyle::Iec => "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z",
                SvgStyle::Din => "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z",
            }
        },
        crate::parser::ComponentType::Antenna => {
            match style {
                SvgStyle::Ieee => "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15",
                SvgStyle::Iec => "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15",
                SvgStyle::Din => "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15",
            }
        },
        crate::parser::ComponentType::Speaker => {
            match style {
                SvgStyle::Ieee => "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10",
                SvgStyle::Iec => "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10",
                SvgStyle::Din => "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10",
            }
        },
        crate::parser::ComponentType::Microphone => {
            match style {
                SvgStyle::Ieee => "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20",
                SvgStyle::Iec => "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20",
                SvgStyle::Din => "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20",
            }
        },
        crate::parser::ComponentType::Motor => {
            match style {
                SvgStyle::Ieee => "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0",
                SvgStyle::Iec => "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0",
                SvgStyle::Din => "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0",
            }
        },
        crate::parser::ComponentType::SignalGround => {
            match style {
                SvgStyle::Ieee => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10",
                SvgStyle::Iec => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10",
                SvgStyle::Din => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10",
            }
        },
        crate::parser::ComponentType::ChassisGround => {
            match style {
                SvgStyle::Ieee => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5",
                SvgStyle::Iec => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5",
                SvgStyle::Din => "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5",
            }
        },
        crate::parser::ComponentType::EarthGround => {
            match style {
                SvgStyle::Ieee => "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15",
                SvgStyle::Iec => "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15",
                SvgStyle::Din => "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15",
            }
        },
    };
    
    Ok(symbol.to_string())
}

fn render_connection(
    connection: &PositionedConnection,
    colors: &ThemeColors,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut svg = String::new();
    
    // Create path for the connection
    if connection.path.len() > 1 {
        let mut path_data = String::from("M ");
        for (i, point) in connection.path.iter().enumerate() {
            if i == 0 {
                path_data.push_str(&format!("{} {}", point.x, point.y));
            } else {
                path_data.push_str(&format!(" L {} {}", point.x, point.y));
            }
        }
        
        svg.push_str(&format!(
            "<path d=\"{}\" class=\"wire\" />\n",
            path_data
        ));
    } else {
        // Direct line if only two points
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"wire\" />\n",
            connection.from.x, connection.from.y,
            connection.to.x, connection.to.y
        ));
    }
    
    Ok(svg)
}

fn rotation_to_degrees(rotation: &Rotation) -> f64 {
    match rotation {
        Rotation::Deg0 => 0.0,
        Rotation::Deg90 => 90.0,
        Rotation::Deg180 => 180.0,
        Rotation::Deg270 => 270.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Circuit, Component, ComponentType, Rotation as ParserRotation};
    use crate::layout::{calculate_layout};
    
    #[test]
    fn test_render_to_svg() {
        let circuit = Circuit {
            components: vec![
                Component {
                    id: "R1".to_string(),
                    component_type: ComponentType::Resistor,
                    value: Some("1k".to_string()),
                    position: Some((0.0, 0.0)),
                    rotation: ParserRotation::Deg0,
                    label: Some("R1".to_string()),
                    properties: std::collections::HashMap::new(),
                },
                Component {
                    id: "R2".to_string(),
                    component_type: ComponentType::Resistor,
                    value: Some("2k".to_string()),
                    position: Some((100.0, 0.0)),
                    rotation: ParserRotation::Deg0,
                    label: Some("R2".to_string()),
                    properties: std::collections::HashMap::new(),
                },
            ],
            connections: vec![],
            nets: vec![],
        };
        
        let layout = calculate_layout(&circuit).expect("Failed to calculate layout");
        let svg = render_to_svg(&layout, SvgTheme::Light, SvgStyle::Ieee);
        assert!(svg.is_ok());
        
        let svg_content = svg.unwrap();
        assert!(svg_content.contains("svg"));
        assert!(svg_content.contains("path"));
        assert!(svg_content.contains("R1"));
        assert!(svg_content.contains("R2"));
    }
}