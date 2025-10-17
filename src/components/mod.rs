use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTemplate {
    pub name: String,
    pub symbol: String, // SVG path or symbol reference
    pub pins: Vec<PinDefinition>,
    pub width: f64,
    pub height: f64,
    pub style_variants: HashMap<String, String>, // e.g., "ieee" -> "path_data"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinDefinition {
    pub name: String,
    pub position: (f64, f64), // x, y relative to component center
    pub direction: PinDirection,
    pub pin_type: PinType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PinDirection {
    Input,
    Output,
    Bidirectional,
    Passive,
    Power,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PinType {
    Digital,
    Analog,
    Power,
    Ground,
}

// Component templates registry
use std::sync::OnceLock;
static COMPONENT_TEMPLATES: OnceLock<HashMap<String, ComponentTemplate>> = OnceLock::new();

pub fn get_component_templates() -> &'static HashMap<String, ComponentTemplate> {
    COMPONENT_TEMPLATES.get_or_init(|| initialize_component_templates())
}

fn initialize_component_templates() -> HashMap<String, ComponentTemplate> {
    let mut templates = HashMap::new();
    
    // Passive components
    templates.insert("resistor".to_string(), ComponentTemplate {
        name: "resistor".to_string(),
        symbol: "resistor".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-20.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (20.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 10.0,
        style_variants: create_resistor_variants(),
    });
    
    templates.insert("capacitor".to_string(), ComponentTemplate {
        name: "capacitor".to_string(),
        symbol: "capacitor".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (0.0, -20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (0.0, 20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 10.0,
        height: 40.0,
        style_variants: create_capacitor_variants(),
    });
    
    templates.insert("inductor".to_string(), ComponentTemplate {
        name: "inductor".to_string(),
        symbol: "inductor".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-20.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (20.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 15.0,
        style_variants: create_inductor_variants(),
    });
    
    templates.insert("potentiometer".to_string(), ComponentTemplate {
        name: "potentiometer".to_string(),
        symbol: "potentiometer".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-20.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (-20.0, 10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "W".to_string(), position: (20.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 25.0,
        style_variants: create_potentiometer_variants(),
    });
    
    templates.insert("transformer".to_string(), ComponentTemplate {
        name: "transformer".to_string(),
        symbol: "transformer".to_string(),
        pins: vec![
            PinDefinition { name: "P1".to_string(), position: (-30.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "P2".to_string(), position: (-30.0, 10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "S1".to_string(), position: (30.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "S2".to_string(), position: (30.0, 10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 60.0,
        height: 30.0,
        style_variants: create_transformer_variants(),
    });
    
    // Sources
    templates.insert("dc_voltage".to_string(), ComponentTemplate {
        name: "dc_voltage".to_string(),
        symbol: "voltage_source".to_string(),
        pins: vec![
            PinDefinition { name: "+".to_string(), position: (0.0, -20.0), direction: PinDirection::Power, pin_type: PinType::Power },
            PinDefinition { name: "-".to_string(), position: (0.0, 20.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_dc_voltage_variants(),
    });
    
    templates.insert("dc_current".to_string(), ComponentTemplate {
        name: "dc_current".to_string(),
        symbol: "current_source".to_string(),
        pins: vec![
            PinDefinition { name: "+".to_string(), position: (0.0, -20.0), direction: PinDirection::Power, pin_type: PinType::Power },
            PinDefinition { name: "-".to_string(), position: (0.0, 20.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_current_source_variants(),
    });
    
    templates.insert("ac_voltage".to_string(), ComponentTemplate {
        name: "ac_voltage".to_string(),
        symbol: "ac_voltage".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (0.0, -20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (0.0, 20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_ac_voltage_variants(),
    });
    
    templates.insert("ac_current".to_string(), ComponentTemplate {
        name: "ac_current".to_string(),
        symbol: "ac_current".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (0.0, -20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (0.0, 20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_ac_current_variants(),
    });
    
    templates.insert("signal_generator".to_string(), ComponentTemplate {
        name: "signal_generator".to_string(),
        symbol: "signal_generator".to_string(),
        pins: vec![
            PinDefinition { name: "OUT".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "GND".to_string(), position: (-20.0, 0.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 40.0,
        height: 20.0,
        style_variants: create_signal_generator_variants(),
    });
    
    // Semiconductors
    templates.insert("diode".to_string(), ComponentTemplate {
        name: "diode".to_string(),
        symbol: "diode".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "K".to_string(), position: (0.0, 20.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_diode_variants(),
    });
    
    templates.insert("zener_diode".to_string(), ComponentTemplate {
        name: "zener_diode".to_string(),
        symbol: "zener_diode".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "K".to_string(), position: (0.0, 20.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_zener_diode_variants(),
    });
    
    templates.insert("schottky_diode".to_string(), ComponentTemplate {
        name: "schottky_diode".to_string(),
        symbol: "schottky_diode".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "K".to_string(), position: (0.0, 20.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_schottky_diode_variants(),
    });
    
    templates.insert("led".to_string(), ComponentTemplate {
        name: "led".to_string(),
        symbol: "led".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "K".to_string(), position: (0.0, 20.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_led_variants(),
    });
    
    templates.insert("npn_transistor".to_string(), ComponentTemplate {
        name: "npn_transistor".to_string(),
        symbol: "transistor".to_string(),
        pins: vec![
            PinDefinition { name: "B".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "C".to_string(), position: (20.0, -10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "E".to_string(), position: (20.0, 10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_npn_transistor_variants(),
    });
    
    templates.insert("pnp_transistor".to_string(), ComponentTemplate {
        name: "pnp_transistor".to_string(),
        symbol: "transistor".to_string(),
        pins: vec![
            PinDefinition { name: "B".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "C".to_string(), position: (20.0, 10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "E".to_string(), position: (20.0, -10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_pnp_transistor_variants(),
    });
    
    templates.insert("nmos_transistor".to_string(), ComponentTemplate {
        name: "nmos_transistor".to_string(),
        symbol: "mosfet".to_string(),
        pins: vec![
            PinDefinition { name: "G".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "D".to_string(), position: (20.0, -10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "S".to_string(), position: (20.0, 10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_nmos_variants(),
    });
    
    templates.insert("pmos_transistor".to_string(), ComponentTemplate {
        name: "pmos_transistor".to_string(),
        symbol: "mosfet".to_string(),
        pins: vec![
            PinDefinition { name: "G".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "D".to_string(), position: (20.0, 10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "S".to_string(), position: (20.0, -10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_pmos_variants(),
    });
    
    templates.insert("jfet".to_string(), ComponentTemplate {
        name: "jfet".to_string(),
        symbol: "jfet".to_string(),
        pins: vec![
            PinDefinition { name: "G".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "D".to_string(), position: (20.0, -10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "S".to_string(), position: (20.0, 10.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_jfet_variants(),
    });
    
    // ICs
    templates.insert("op_amp".to_string(), ComponentTemplate {
        name: "op_amp".to_string(),
        symbol: "op_amp".to_string(),
        pins: vec![
            PinDefinition { name: "+".to_string(), position: (-25.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "-".to_string(), position: (-25.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "OUT".to_string(), position: (25.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "V+".to_string(), position: (0.0, -25.0), direction: PinDirection::Power, pin_type: PinType::Power },
            PinDefinition { name: "V-".to_string(), position: (0.0, 25.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 50.0,
        height: 50.0,
        style_variants: create_op_amp_variants(),
    });
    
    templates.insert("comparator".to_string(), ComponentTemplate {
        name: "comparator".to_string(),
        symbol: "comparator".to_string(),
        pins: vec![
            PinDefinition { name: "+".to_string(), position: (-25.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "-".to_string(), position: (-25.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "OUT".to_string(), position: (25.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 50.0,
        height: 30.0,
        style_variants: create_comparator_variants(),
    });
    
    templates.insert("timer_555".to_string(), ComponentTemplate {
        name: "timer_555".to_string(),
        symbol: "timer_555".to_string(),
        pins: vec![
            PinDefinition { name: "GND".to_string(), position: (0.0, 25.0), direction: PinDirection::Power, pin_type: PinType::Ground },
            PinDefinition { name: "TRIG".to_string(), position: (-30.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "OUT".to_string(), position: (30.0, -20.0), direction: PinDirection::Output, pin_type: PinType::Digital },
            PinDefinition { name: "RESET".to_string(), position: (30.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "CTRL".to_string(), position: (30.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "THR".to_string(), position: (-30.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "DIS".to_string(), position: (-30.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "VCC".to_string(), position: (0.0, -25.0), direction: PinDirection::Power, pin_type: PinType::Power },
        ],
        width: 60.0,
        height: 50.0,
        style_variants: create_timer_555_variants(),
    });
    
    // Logic gates
    templates.insert("and_gate".to_string(), ComponentTemplate {
        name: "and_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-20.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "B".to_string(), position: (-20.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_and_gate_variants(),
    });
    
    templates.insert("or_gate".to_string(), ComponentTemplate {
        name: "or_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-20.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "B".to_string(), position: (-20.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_or_gate_variants(),
    });
    
    templates.insert("not_gate".to_string(), ComponentTemplate {
        name: "not_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 40.0,
        height: 20.0,
        style_variants: create_not_gate_variants(),
    });
    
    templates.insert("nand_gate".to_string(), ComponentTemplate {
        name: "nand_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-20.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "B".to_string(), position: (-20.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (25.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 45.0,
        height: 30.0,
        style_variants: create_nand_gate_variants(),
    });
    
    templates.insert("nor_gate".to_string(), ComponentTemplate {
        name: "nor_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-20.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "B".to_string(), position: (-20.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (25.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 45.0,
        height: 30.0,
        style_variants: create_nor_gate_variants(),
    });
    
    templates.insert("xor_gate".to_string(), ComponentTemplate {
        name: "xor_gate".to_string(),
        symbol: "logic_gate".to_string(),
        pins: vec![
            PinDefinition { name: "A".to_string(), position: (-25.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "B".to_string(), position: (-25.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Y".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 45.0,
        height: 30.0,
        style_variants: create_xor_gate_variants(),
    });
    
    templates.insert("flip_flop".to_string(), ComponentTemplate {
        name: "flip_flop".to_string(),
        symbol: "flip_flop".to_string(),
        pins: vec![
            PinDefinition { name: "D".to_string(), position: (-25.0, -15.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "CLK".to_string(), position: (-25.0, -5.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "CLR".to_string(), position: (-25.0, 5.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Q".to_string(), position: (25.0, -5.0), direction: PinDirection::Output, pin_type: PinType::Digital },
            PinDefinition { name: "Q̄".to_string(), position: (25.0, 5.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 50.0,
        height: 40.0,
        style_variants: create_flip_flop_variants(),
    });
    
    templates.insert("counter".to_string(), ComponentTemplate {
        name: "counter".to_string(),
        symbol: "counter".to_string(),
        pins: vec![
            PinDefinition { name: "CLK".to_string(), position: (-30.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "RST".to_string(), position: (-30.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "Q0".to_string(), position: (30.0, -15.0), direction: PinDirection::Output, pin_type: PinType::Digital },
            PinDefinition { name: "Q1".to_string(), position: (30.0, -5.0), direction: PinDirection::Output, pin_type: PinType::Digital },
            PinDefinition { name: "Q2".to_string(), position: (30.0, 5.0), direction: PinDirection::Output, pin_type: PinType::Digital },
            PinDefinition { name: "Q3".to_string(), position: (30.0, 15.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 60.0,
        height: 50.0,
        style_variants: create_counter_variants(),
    });
    
    templates.insert("multiplexer".to_string(), ComponentTemplate {
        name: "multiplexer".to_string(),
        symbol: "multiplexer".to_string(),
        pins: vec![
            PinDefinition { name: "I0".to_string(), position: (-30.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "I1".to_string(), position: (-30.0, -10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "I2".to_string(), position: (-30.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "I3".to_string(), position: (-30.0, 10.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "SEL0".to_string(), position: (-30.0, 20.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "SEL1".to_string(), position: (-20.0, 20.0), direction: PinDirection::Input, pin_type: PinType::Digital },
            PinDefinition { name: "OUT".to_string(), position: (30.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Digital },
        ],
        width: 60.0,
        height: 50.0,
        style_variants: create_multiplexer_variants(),
    });
    
    // Analog
    templates.insert("voltage_regulator".to_string(), ComponentTemplate {
        name: "voltage_regulator".to_string(),
        symbol: "voltage_regulator".to_string(),
        pins: vec![
            PinDefinition { name: "IN".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "OUT".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "GND".to_string(), position: (0.0, 20.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 40.0,
        height: 30.0,
        style_variants: create_voltage_regulator_variants(),
    });
    
    templates.insert("crystal".to_string(), ComponentTemplate {
        name: "crystal".to_string(),
        symbol: "crystal".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (0.0, -20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (0.0, 20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 15.0,
        height: 40.0,
        style_variants: create_crystal_variants(),
    });
    
    templates.insert("relay".to_string(), ComponentTemplate {
        name: "relay".to_string(),
        symbol: "relay".to_string(),
        pins: vec![
            PinDefinition { name: "COIL1".to_string(), position: (-25.0, -15.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "COIL2".to_string(), position: (-25.0, 15.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "COM".to_string(), position: (25.0, -5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NO".to_string(), position: (25.0, -15.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NC".to_string(), position: (25.0, 5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 50.0,
        height: 40.0,
        style_variants: create_relay_variants(),
    });
    
    templates.insert("spst_switch".to_string(), ComponentTemplate {
        name: "spst_switch".to_string(),
        symbol: "switch".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-15.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (15.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 30.0,
        height: 20.0,
        style_variants: create_spst_switch_variants(),
    });
    
    templates.insert("spdt_switch".to_string(), ComponentTemplate {
        name: "spdt_switch".to_string(),
        symbol: "switch".to_string(),
        pins: vec![
            PinDefinition { name: "COM".to_string(), position: (-15.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NO".to_string(), position: (15.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NC".to_string(), position: (15.0, 10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 30.0,
        height: 30.0,
        style_variants: create_spdt_switch_variants(),
    });
    
    templates.insert("dpdt_switch".to_string(), ComponentTemplate {
        name: "dpdt_switch".to_string(),
        symbol: "switch".to_string(),
        pins: vec![
            PinDefinition { name: "COM1".to_string(), position: (-20.0, -5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NO1".to_string(), position: (20.0, -15.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NC1".to_string(), position: (20.0, 5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "COM2".to_string(), position: (-20.0, 5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NO2".to_string(), position: (20.0, -5.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "NC2".to_string(), position: (20.0, 15.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 40.0,
        style_variants: create_dpdt_switch_variants(),
    });
    
    templates.insert("fuse".to_string(), ComponentTemplate {
        name: "fuse".to_string(),
        symbol: "fuse".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-15.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (15.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 30.0,
        height: 10.0,
        style_variants: create_fuse_variants(),
    });
    
    templates.insert("battery".to_string(), ComponentTemplate {
        name: "battery".to_string(),
        symbol: "battery".to_string(),
        pins: vec![
            PinDefinition { name: "+".to_string(), position: (-15.0, 0.0), direction: PinDirection::Power, pin_type: PinType::Power },
            PinDefinition { name: "-".to_string(), position: (15.0, 0.0), direction: PinDirection::Power, pin_type: PinType::Power },
        ],
        width: 30.0,
        height: 20.0,
        style_variants: create_battery_variants(),
    });
    
    // Digital and others
    templates.insert("microcontroller".to_string(), ComponentTemplate {
        name: "microcontroller".to_string(),
        symbol: "microcontroller".to_string(),
        pins: vec![
            PinDefinition { name: "VCC".to_string(), position: (0.0, -40.0), direction: PinDirection::Power, pin_type: PinType::Power },
            PinDefinition { name: "GND".to_string(), position: (0.0, 40.0), direction: PinDirection::Power, pin_type: PinType::Ground },
            PinDefinition { name: "PA0".to_string(), position: (-40.0, -30.0), direction: PinDirection::Bidirectional, pin_type: PinType::Digital },
            PinDefinition { name: "PA1".to_string(), position: (-40.0, -20.0), direction: PinDirection::Bidirectional, pin_type: PinType::Digital },
            PinDefinition { name: "PB0".to_string(), position: (-40.0, 20.0), direction: PinDirection::Bidirectional, pin_type: PinType::Digital },
            PinDefinition { name: "PB1".to_string(), position: (-40.0, 30.0), direction: PinDirection::Bidirectional, pin_type: PinType::Digital },
            PinDefinition { name: "XTAL1".to_string(), position: (40.0, -30.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "XTAL2".to_string(), position: (40.0, -20.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 80.0,
        height: 80.0,
        style_variants: create_microcontroller_variants(),
    });
    
    templates.insert("connector".to_string(), ComponentTemplate {
        name: "connector".to_string(),
        symbol: "connector".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (0.0, -20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (0.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "3".to_string(), position: (0.0, 0.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "4".to_string(), position: (0.0, 10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
            PinDefinition { name: "5".to_string(), position: (0.0, 20.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 15.0,
        height: 50.0,
        style_variants: create_connector_variants(),
    });
    
    templates.insert("test_point".to_string(), ComponentTemplate {
        name: "test_point".to_string(),
        symbol: "test_point".to_string(),
        pins: vec![
            PinDefinition { name: "TP".to_string(), position: (0.0, -10.0), direction: PinDirection::Passive, pin_type: PinType::Analog },
        ],
        width: 10.0,
        height: 10.0,
        style_variants: create_test_point_variants(),
    });
    
    templates.insert("ammeter".to_string(), ComponentTemplate {
        name: "ammeter".to_string(),
        symbol: "ammeter".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-20.0, 0.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 40.0,
        height: 20.0,
        style_variants: create_ammeter_variants(),
    });
    
    templates.insert("voltmeter".to_string(), ComponentTemplate {
        name: "voltmeter".to_string(),
        symbol: "voltmeter".to_string(),
        pins: vec![
            PinDefinition { name: "POS".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "NEG".to_string(), position: (0.0, 20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_voltmeter_variants(),
    });
    
    templates.insert("oscilloscope_probe".to_string(), ComponentTemplate {
        name: "oscilloscope_probe".to_string(),
        symbol: "oscilloscope_probe".to_string(),
        pins: vec![
            PinDefinition { name: "SIG".to_string(), position: (0.0, -20.0), direction: PinDirection::Input, pin_type: PinType::Analog },
            PinDefinition { name: "GND".to_string(), position: (0.0, 20.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 40.0,
        style_variants: create_oscilloscope_probe_variants(),
    });
    
    // Misc
    templates.insert("antenna".to_string(), ComponentTemplate {
        name: "antenna".to_string(),
        symbol: "antenna".to_string(),
        pins: vec![
            PinDefinition { name: "ANT".to_string(), position: (0.0, -20.0), direction: PinDirection::Bidirectional, pin_type: PinType::Analog },
        ],
        width: 20.0,
        height: 20.0,
        style_variants: create_antenna_variants(),
    });
    
    templates.insert("speaker".to_string(), ComponentTemplate {
        name: "speaker".to_string(),
        symbol: "speaker".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-15.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (15.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 30.0,
        height: 30.0,
        style_variants: create_speaker_variants(),
    });
    
    templates.insert("microphone".to_string(), ComponentTemplate {
        name: "microphone".to_string(),
        symbol: "microphone".to_string(),
        pins: vec![
            PinDefinition { name: "OUT".to_string(), position: (20.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "GND".to_string(), position: (-20.0, 0.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 40.0,
        height: 20.0,
        style_variants: create_microphone_variants(),
    });
    
    templates.insert("motor".to_string(), ComponentTemplate {
        name: "motor".to_string(),
        symbol: "motor".to_string(),
        pins: vec![
            PinDefinition { name: "1".to_string(), position: (-15.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
            PinDefinition { name: "2".to_string(), position: (15.0, 0.0), direction: PinDirection::Output, pin_type: PinType::Analog },
        ],
        width: 30.0,
        height: 30.0,
        style_variants: create_motor_variants(),
    });
    
    templates.insert("signal_ground".to_string(), ComponentTemplate {
        name: "signal_ground".to_string(),
        symbol: "ground".to_string(),
        pins: vec![
            PinDefinition { name: "GND".to_string(), position: (0.0, -15.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 15.0,
        style_variants: create_ground_variants(),
    });
    
    templates.insert("chassis_ground".to_string(), ComponentTemplate {
        name: "chassis_ground".to_string(),
        symbol: "ground".to_string(),
        pins: vec![
            PinDefinition { name: "GND".to_string(), position: (0.0, -15.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 15.0,
        style_variants: create_chassis_ground_variants(),
    });
    
    templates.insert("earth_ground".to_string(), ComponentTemplate {
        name: "earth_ground".to_string(),
        symbol: "ground".to_string(),
        pins: vec![
            PinDefinition { name: "GND".to_string(), position: (0.0, -15.0), direction: PinDirection::Power, pin_type: PinType::Ground },
        ],
        width: 20.0,
        height: 15.0,
        style_variants: create_earth_ground_variants(),
    });
    
    templates
}

fn create_resistor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L -15 -5 L -5 -5 L 5 5 L 15 5 L 20 0".to_string());
    variants.insert("iec".to_string(), "M -20 0 H 20 V 5 H -20 V -5 H 20".to_string());
    variants.insert("din".to_string(), "M -20 0 L -15 -5 L -5 -5 L 5 5 L 15 5 L 20 0".to_string());
    variants
}

fn create_capacitor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 -10 M -5 -10 L 10 -10 M -5 10 L 10 10 M 0 10 L 0 20".to_string());
    variants.insert("iec".to_string(), "M -5 -20 V 20 M 5 -20 V 20".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 -10 M -5 -10 L 10 -10 M -5 10 L 10 10 M 0 10 L 0 20".to_string());
    variants
}

fn create_diode_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0".to_string());
    variants
}

fn create_ground_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10".to_string());
    variants.insert("iec".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10".to_string());
    variants.insert("din".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10".to_string());
    variants
}

fn create_battery_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0".to_string());
    variants.insert("iec".to_string(), "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0".to_string());
    variants.insert("din".to_string(), "M -15 0 L -10 -10 M -10 -5 L -10 5 M 10 -10 L 10 10 M 10 5 L 15 0".to_string());
    variants
}

fn create_inductor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0".to_string());
    variants.insert("iec".to_string(), "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0".to_string());
    variants.insert("din".to_string(), "M -20 0 Q -15 -10 -5 -10 Q 5 -10 15 -10 Q 20 0".to_string());
    variants
}

fn create_potentiometer_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10".to_string());
    variants.insert("iec".to_string(), "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10".to_string());
    variants.insert("din".to_string(), "M -20 -10 H 20 M -20 10 H 20 M 0 10 L 0 -10 M 5 -10 L -5 10".to_string());
    variants
}

fn create_transformer_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10".to_string());
    variants.insert("iec".to_string(), "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10".to_string());
    variants.insert("din".to_string(), "M -30 -10 Q -25 -15 -20 -10 Q -15 -5 -10 -10 Q -5 -15 0 -10 Q 5 -5 10 -10 Q 15 -15 20 -10 Q 25 -5 30 -10 M -30 10 Q -25 5 -20 10 Q -15 15 -10 10 Q -5 5 0 10 Q 5 15 10 10 Q 15 5 20 10 Q 25 15 30 10".to_string());
    variants
}

fn create_dc_voltage_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 20 M -10 -5 H 10 M -10 5 H 10".to_string());
    variants
}

fn create_current_source_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5".to_string());
    variants.insert("iec".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5".to_string());
    variants.insert("din".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5".to_string());
    variants
}

fn create_ac_voltage_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants.insert("iec".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants.insert("din".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants
}

fn create_ac_current_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants.insert("iec".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants.insert("din".to_string(), "M 0 -20 A 10 10 0 0 1 0 20 A 10 10 0 0 1 0 -20 Z M 15 0 L 10 -5 M 15 0 L 10 5 M -5 0 Q 0 -5 5 0 Q 0 5 -5 0".to_string());
    variants
}

fn create_signal_generator_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0".to_string());
    variants.insert("iec".to_string(), "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0".to_string());
    variants.insert("din".to_string(), "M -20 -10 H 20 V 20 H -20 Z M -10 0 Q -5 -5 0 0 Q -5 5 -10 0".to_string());
    variants
}

fn create_zener_diode_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -2 -10 L -6 -6 M 6 6 L 2 2".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -2 -10 L -6 -6 M 6 6 L 2 2".to_string());
    variants
}

fn create_schottky_diode_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -8 0 L -10 0 L -10 2 M 10 -2 L 10 0 L 8 0".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M -8 0 L -10 0 L -10 2 M 10 -2 L 10 0 L 8 0".to_string());
    variants
}

fn create_led_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L -10 0 L 0 20 L 10 0 Z M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 0 L -10 0 L 0 20 M 10 -20 L 0 0 M 5 -25 L 0 -30 M 2 -27 L 0 -30 L 3 -32".to_string());
    variants
}

fn create_npn_transistor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0".to_string());
    variants.insert("iec".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0".to_string());
    variants.insert("din".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 15 -5 L 10 0 L 15 5 M 5 5 L 10 0".to_string());
    variants
}

fn create_pnp_transistor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0".to_string());
    variants.insert("iec".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0".to_string());
    variants.insert("din".to_string(), "M -20 0 L 0 0 M 0 -10 L 0 10 M 0 0 L 15 0 M 10 -5 L 15 0 L 10 5 M 5 -5 L 10 0".to_string());
    variants
}

fn create_nmos_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13".to_string());
    variants.insert("iec".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13".to_string());
    variants.insert("din".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 5 L 15 5 M 10 0 L 10 10 M 15 0 L 15 10 M 12 7 L 15 10 L 18 13".to_string());
    variants
}

fn create_pmos_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7".to_string());
    variants.insert("iec".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7".to_string());
    variants.insert("din".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 -5 L 15 -5 M 10 -10 L 10 0 M 15 -10 L 15 0 M 12 -13 L 15 -10 L 18 -7".to_string());
    variants
}

fn create_jfet_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5".to_string());
    variants.insert("iec".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5".to_string());
    variants.insert("din".to_string(), "M -20 0 L -5 0 M -5 -10 L -5 10 M 0 0 L 15 0 M 10 -5 L 10 5".to_string());
    variants
}

fn create_op_amp_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants.insert("iec".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants.insert("din".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants
}

fn create_comparator_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants.insert("iec".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants.insert("din".to_string(), "M -25 -15 L 25 0 L -25 15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 25 0 L 30 0".to_string());
    variants
}

fn create_timer_555_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("iec".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("din".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants
}

fn create_and_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -10 -15 A 15 15 0 0 1 0 -15 A 15 15 0 0 1 10 -15 V 15 A 15 15 0 0 1 0 15 A 15 15 0 0 1 -10 15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants.insert("iec".to_string(), "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants.insert("din".to_string(), "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants
}

fn create_or_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants.insert("iec".to_string(), "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants.insert("din".to_string(), "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 20 0 L 25 0".to_string());
    variants
}

fn create_not_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0".to_string());
    variants.insert("iec".to_string(), "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0".to_string());
    variants.insert("din".to_string(), "M -10 -10 L 10 0 L -10 10 Z M 10 0 L 15 0".to_string());
    variants
}

fn create_nand_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -10 -15 A 15 15 0 0 1 0 -15 A 15 15 0 0 1 10 -15 V 15 A 15 15 0 0 1 0 15 A 15 15 0 0 1 -10 15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants.insert("iec".to_string(), "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants.insert("din".to_string(), "M -20 -15 V 30 H 20 Q 20 0 20 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants
}

fn create_nor_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants.insert("iec".to_string(), "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants.insert("din".to_string(), "M -20 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z M -20 -10 L -10 -10 M -20 10 L -10 10 M 15 0 L 25 0 M 20 -2 L 25 0 L 20 2".to_string());
    variants
}

fn create_xor_gate_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z".to_string());
    variants.insert("iec".to_string(), "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z".to_string());
    variants.insert("din".to_string(), "M -20 -15 C 0 -15 10 0 0 15 C -20 15 -20 -15 -20 -15 Z M -25 -10 L -15 -10 M -25 10 L -15 10 M 20 0 L 25 0 M -10 -15 C 10 -15 20 0 10 15 C -10 15 -10 -15 -10 -15 Z".to_string());
    variants
}

fn create_flip_flop_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -25 -20 H 25 V 40 H -25 Z".to_string());
    variants.insert("iec".to_string(), "M -25 -20 H 25 V 40 H -25 Z".to_string());
    variants.insert("din".to_string(), "M -25 -20 H 25 V 40 H -25 Z".to_string());
    variants
}

fn create_counter_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("iec".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("din".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants
}

fn create_multiplexer_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("iec".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants.insert("din".to_string(), "M -30 -25 H 30 V 50 H -30 Z".to_string());
    variants
}

fn create_voltage_regulator_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 -10 H 20 V 20 H -20 Z".to_string());
    variants.insert("iec".to_string(), "M -20 -10 H 20 V 20 H -20 Z".to_string());
    variants.insert("din".to_string(), "M -20 -10 H 20 V 20 H -20 Z".to_string());
    variants
}

fn create_crystal_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15".to_string());
    variants.insert("iec".to_string(), "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15".to_string());
    variants.insert("din".to_string(), "M 0 -20 V 20 M -7 -15 H 7 M -7 15 H 7 M -7 -15 V 15 M 7 -15 V 15".to_string());
    variants
}

fn create_relay_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5".to_string());
    variants.insert("iec".to_string(), "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5".to_string());
    variants.insert("din".to_string(), "M -25 -15 H 25 V 30 H -25 Z M 15 -5 L 25 -15 M 15 -5 L 25 5 M 15 -5 L 15 5".to_string());
    variants
}

fn create_spst_switch_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5".to_string());
    variants.insert("iec".to_string(), "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5".to_string());
    variants.insert("din".to_string(), "M -15 0 L -5 0 M 5 0 L 15 0 M -5 -5 L 5 5".to_string());
    variants
}

fn create_spdt_switch_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0".to_string());
    variants.insert("iec".to_string(), "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0".to_string());
    variants.insert("din".to_string(), "M -15 0 L -5 0 M 5 -10 L 15 -10 M 5 10 L 15 10 M -5 -5 L 5 0".to_string());
    variants
}

fn create_dpdt_switch_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15".to_string());
    variants.insert("iec".to_string(), "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15".to_string());
    variants.insert("din".to_string(), "M -20 -5 L -10 -5 M 10 -15 L 20 -15 M 10 5 L 20 5 M -10 -5 L 10 0 M -20 5 L -10 5 M 10 -5 L 20 -5 M 10 15 L 20 15".to_string());
    variants
}

fn create_fuse_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10".to_string());
    variants.insert("iec".to_string(), "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10".to_string());
    variants.insert("din".to_string(), "M -15 0 H 15 M -10 -5 V 10 H 10 V -10 H -10 V 10".to_string());
    variants
}

fn create_microcontroller_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -40 -40 H 40 V 80 H -40 Z".to_string());
    variants.insert("iec".to_string(), "M -40 -40 H 40 V 80 H -40 Z".to_string());
    variants.insert("din".to_string(), "M -40 -40 H 40 V 80 H -40 Z".to_string());
    variants
}

fn create_connector_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 15 -20 L 15 20 L 0 20".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L 15 -20 L 15 20 L 0 20".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 15 -20 L 15 20 L 0 20".to_string());
    variants
}

fn create_test_point_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z".to_string());
    variants.insert("iec".to_string(), "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z".to_string());
    variants.insert("din".to_string(), "M 0 -10 L 0 0 C 0 5 5 5 5 0 C 5 -5 0 -5 0 -10 Z".to_string());
    variants
}

fn create_ammeter_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5".to_string());
    variants.insert("iec".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5".to_string());
    variants.insert("din".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M -20 -5 H 20 M -20 5 H 20 M 0 0 L 5 5".to_string());
    variants
}

fn create_voltmeter_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5".to_string());
    variants.insert("iec".to_string(), "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5".to_string());
    variants.insert("din".to_string(), "M 0 -20 A 10 10 0 1 1 0 -19.9 M 0 -5 H 0 5 M -10 0 H 10 M 0 0 L 5 5".to_string());
    variants
}

fn create_oscilloscope_probe_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 20 M -10 -10 H 10 V 20 H -10 Z".to_string());
    variants
}

fn create_antenna_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15".to_string());
    variants.insert("iec".to_string(), "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15".to_string());
    variants.insert("din".to_string(), "M 0 -20 L 0 0 L -10 15 M 0 0 L 10 15".to_string());
    variants
}

fn create_speaker_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10".to_string());
    variants.insert("iec".to_string(), "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10".to_string());
    variants.insert("din".to_string(), "M -15 0 L -10 -10 V 20 L -15 0 M -10 -10 H 15 V 20 H -10".to_string());
    variants
}

fn create_microphone_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20".to_string());
    variants.insert("iec".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20".to_string());
    variants.insert("din".to_string(), "M -20 0 A 10 10 0 1 1 -20 0.1 M 10 -10 V 20".to_string());
    variants
}

fn create_motor_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0".to_string());
    variants.insert("iec".to_string(), "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0".to_string());
    variants.insert("din".to_string(), "M -15 0 A 15 15 0 1 1 30 0 A 15 15 0 1 1 -30 0".to_string());
    variants
}

fn create_chassis_ground_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5".to_string());
    variants.insert("iec".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5".to_string());
    variants.insert("din".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10 M 0 0 L 0 5".to_string());
    variants
}

fn create_earth_ground_variants() -> HashMap<String, String> {
    let mut variants = HashMap::new();
    variants.insert("ieee".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15".to_string());
    variants.insert("iec".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15".to_string());
    variants.insert("din".to_string(), "M 0 -15 L 0 0 M -10 0 L 10 0 M -8 5 L 8 5 M -6 10 L 6 10 M -4 15 L 4 15".to_string());
    variants
}

// Get component template by name
pub fn get_component_template(component_type: &str) -> Option<&'static ComponentTemplate> {
    let templates = get_component_templates();
    templates.get(&component_type.to_lowercase())
}

// Get list of all available component types
pub fn get_component_list() -> Vec<String> {
    let templates = get_component_templates();
    templates.keys().cloned().collect()
}

// Get pin definitions for a component type
pub fn get_component_pins(component_type: &str) -> Option<&[PinDefinition]> {
    if let Some(template) = get_component_template(component_type) {
        Some(&template.pins)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_component_template() {
        let template = get_component_template("resistor");
        assert!(template.is_some());
        assert_eq!(template.unwrap().name, "resistor");
    }

    #[test]
    fn test_get_component_pins() {
        let pins = get_component_pins("resistor");
        assert!(pins.is_some());
        assert_eq!(pins.unwrap().len(), 2);
        assert_eq!(pins.unwrap()[0].name, "1");
        assert_eq!(pins.unwrap()[1].name, "2");
    }

    #[test]
    fn test_get_component_list() {
        let components = get_component_list();
        assert!(components.contains(&"resistor".to_string()));
        assert!(components.contains(&"capacitor".to_string()));
        assert!(components.contains(&"diode".to_string()));
    }
}