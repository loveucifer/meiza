use crate::parser::{Circuit, Component, ComponentType, Connection, ConnectionPoint};
use std::collections::HashMap;

pub fn export_spice(circuit: &Circuit) -> Result<String, Box<dyn std::error::Error>> {
    let mut spice = String::new();
    
    // Add title
    spice.push_str("* Mieza SPICE Netlist\n");
    spice.push_str("* Generated from CDL\n\n");
    
    // Create a net mapping to assign unique node names
    let net_mapping = create_net_mapping(circuit);
    
    // Add components
    for component in &circuit.components {
        let spice_line = component_to_spice(component, &net_mapping)?;
        if !spice_line.is_empty() {
            spice.push_str(&spice_line);
            spice.push('\n');
        }
    }
    
    // Add connections as comments for reference
    spice.push_str("\n* Connections:\n");
    for connection in &circuit.connections {
        spice.push_str(&format!("* {}:{} -> {}:{}\n", 
            connection.from.component_id, 
            connection.from.pin,
            connection.to.component_id,
            connection.to.pin));
    }
    
    // Add .end statement
    spice.push_str("\n.end\n");
    
    Ok(spice)
}

fn create_net_mapping(circuit: &Circuit) -> HashMap<String, String> {
    let mut net_mapping = HashMap::new();
    let mut net_counter = 1;
    
    // First, add explicit nets
    for net in &circuit.nets {
        let net_name = format!("N{}", net_counter);
        net_counter += 1;
        
        for node in &net.nodes {
            let key = format!("{}.{}", node.component_id, node.pin);
            net_mapping.insert(key, net_name.clone());
        }
    }
    
    // Then, map connections to nets
    for connection in &circuit.connections {
        let from_key = format!("{}.{}", connection.from.component_id, connection.from.pin);
        let to_key = format!("{}.{}", connection.to.component_id, connection.to.pin);
        
        // If this connection is already assigned to a net, use that net name
        if let Some(from_net) = net_mapping.get(&from_key) {
            net_mapping.insert(to_key, from_net.clone());
        } else if let Some(to_net) = net_mapping.get(&to_key) {
            net_mapping.insert(from_key, to_net.clone());
        } else {
            // Create a new net for this connection
            let net_name = format!("N{}", net_counter);
            net_counter += 1;
            net_mapping.insert(from_key, net_name.clone());
            net_mapping.insert(to_key, net_name);
        }
    }
    
    // Assign default nets to unconnected pins (e.g., ground to 0)
    for component in &circuit.components {
        // If it's a ground component, assign its pin to ground node (0)
        match component.component_type {
            ComponentType::SignalGround | ComponentType::ChassisGround | ComponentType::EarthGround => {
                let key = format!("{}.GND", component.id);
                net_mapping.insert(key, "0".to_string());
            },
            _ => {
                // For other components, we'll handle this when we connect them
            }
        }
    }
    
    net_mapping
}

fn component_to_spice(
    component: &Component,
    net_mapping: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut spice_line = String::new();
    
    // Get the node names for the component pins
    let pins = get_component_pins_for_spice(&component.component_type);
    
    // Build the connection part of the SPICE line
    let mut node_names = Vec::new();
    for pin in &pins {
        let key = format!("{}.{}", component.id, pin);
        if let Some(node_name) = net_mapping.get(&key) {
            node_names.push(node_name.clone());
        } else {
            // If no explicit connection, assign to ground (0) or leave unconnected
            node_names.push("0".to_string()); // Default to ground for unconnected pins
        }
    }
    
    // Add the component type and value
    match &component.component_type {
        ComponentType::Resistor => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("1");
                spice_line.push_str(&format!("R{} {} {} {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_resistance_value(value)?));
            }
        },
        ComponentType::Capacitor => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("1pF");
                spice_line.push_str(&format!("C{} {} {} {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_capacitance_value(value)?));
            }
        },
        ComponentType::Inductor => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("1µH");
                spice_line.push_str(&format!("L{} {} {} {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_inductance_value(value)?));
            }
        },
        ComponentType::DcVoltage => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("0V");
                spice_line.push_str(&format!("V{} {} {} DC {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_voltage_value(value)?));
            }
        },
        ComponentType::DcCurrent => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("0A");
                spice_line.push_str(&format!("I{} {} {} DC {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_current_value(value)?));
            }
        },
        ComponentType::AcVoltage => {
            if node_names.len() >= 2 {
                let value = component.value.as_deref().unwrap_or("0V");
                // For AC, we'll use a simple AC voltage source
                spice_line.push_str(&format!("V{} {} {} AC {}", 
                    component.id, 
                    node_names[0], 
                    node_names[1], 
                    normalize_voltage_value(value)?));
            }
        },
        ComponentType::Diode => {
            if node_names.len() >= 2 {
                spice_line.push_str(&format!("D{} {} {} D{}", 
                    component.id, 
                    node_names[0], // Anode
                    node_names[1], // Cathode
                    get_diode_model(component)?));
            }
        },
        ComponentType::NpnTransistor => {
            if node_names.len() >= 3 {
                spice_line.push_str(&format!("Q{} {} {} {} QNPN", 
                    component.id, 
                    node_names[1], // Collector
                    node_names[0], // Base
                    node_names[2])); // Emitter
            }
        },
        ComponentType::PnpTransistor => {
            if node_names.len() >= 3 {
                spice_line.push_str(&format!("Q{} {} {} {} QPNP", 
                    component.id, 
                    node_names[2], // Collector
                    node_names[0], // Base
                    node_names[1])); // Emitter
            }
        },
        ComponentType::NmosTransistor => {
            if node_names.len() >= 3 {
                spice_line.push_str(&format!("M{} {} {} {} {} NMOS", 
                    component.id, 
                    node_names[1], // Drain
                    node_names[0], // Gate
                    node_names[2], // Source
                    node_names[2])); // Bulk (typically same as source)
            }
        },
        ComponentType::PmosTransistor => {
            if node_names.len() >= 3 {
                spice_line.push_str(&format!("M{} {} {} {} {} PMOS", 
                    component.id, 
                    node_names[1], // Drain
                    node_names[0], // Gate
                    node_names[2], // Source
                    node_names[2])); // Bulk (typically same as source)
            }
        },
        ComponentType::OpAmp => {
            if node_names.len() >= 3 {
                spice_line.push_str(&format!("X{} {} {} {} OPAMP", 
                    component.id, 
                    node_names[0], // Inverting input
                    node_names[1], // Non-inverting input
                    node_names[2])); // Output
            }
        },
        ComponentType::SignalGround | ComponentType::ChassisGround | ComponentType::EarthGround => {
            // Ground is implicit in SPICE (node 0), so we don't need to add a line
            // But we make sure the component is connected to node 0
            return Ok("".to_string()); // No specific component line needed for ground
        },
        _ => {
            // For components not yet implemented in SPICE export, create a comment
            spice_line.push_str(&format!("* Component {} of type {:?} not supported in SPICE export", 
                component.id, component.component_type));
        }
    }
    
    Ok(spice_line)
}

fn get_component_pins_for_spice(component_type: &ComponentType) -> Vec<String> {
    match component_type {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
            vec!["1".to_string(), "2".to_string()]
        },
        ComponentType::DcVoltage | ComponentType::DcCurrent | 
        ComponentType::AcVoltage | ComponentType::AcCurrent => {
            vec!["+".to_string(), "-".to_string()]
        },
        ComponentType::Diode => {
            vec!["A".to_string(), "K".to_string()] // Anode, Cathode
        },
        ComponentType::NpnTransistor | ComponentType::PnpTransistor => {
            vec!["B".to_string(), "C".to_string(), "E".to_string()] // Base, Collector, Emitter
        },
        ComponentType::NmosTransistor | ComponentType::PmosTransistor => {
            vec!["G".to_string(), "D".to_string(), "S".to_string()] // Gate, Drain, Source
        },
        ComponentType::OpAmp => {
            vec!["-".to_string(), "+".to_string(), "OUT".to_string()] // Inverting, Non-inverting, Output
        },
        ComponentType::SignalGround | ComponentType::ChassisGround | ComponentType::EarthGround => {
            vec!["GND".to_string()]
        },
        _ => vec![], // For other components, return empty vector
    }
}

fn normalize_resistance_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = value.trim();
    
    // Handle common resistance units
    if value.ends_with("k") || value.ends_with("K") {
        let num = value[..value.len()-1].parse::<f64>()?;
        Ok(format!("{}k", num))
    } else if value.ends_with("M") {
        let num = value[..value.len()-1].parse::<f64>()?;
        Ok(format!("{}M", num))
    } else if value.ends_with("m") {
        // Milliohms (rare but possible)
        let num = value[..value.len()-1].parse::<f64>()?;
        Ok(format!("{}m", num))
    } else if value.ends_with("Ω") || value.ends_with("R") {
        Ok(value[..value.len()-1].to_string())
    } else {
        Ok(value.to_string())
    }
}

fn normalize_capacitance_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = value.trim();
    
    // Handle common capacitance units
    if value.ends_with("pF") || value.ends_with("p") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}p", num))
    } else if value.ends_with("nF") || value.ends_with("n") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}n", num))
    } else if value.ends_with("µF") || value.ends_with("uF") || value.ends_with("u") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}µ", num))
    } else if value.ends_with("mF") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}m", num))
    } else if value.ends_with("F") {
        let num = value[..value.len()-1].parse::<f64>()?;
        Ok(format!("{}", num))
    } else {
        // If no unit specified, assume Farads
        Ok(format!("{}F", value))
    }
}

fn normalize_inductance_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = value.trim();
    
    // Handle common inductance units
    if value.ends_with("nH") || value.ends_with("n") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}n", num))
    } else if value.ends_with("µH") || value.ends_with("uH") || value.ends_with("u") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}µ", num))
    } else if value.ends_with("mH") {
        let num = value[..value.len()-2].parse::<f64>()?;
        Ok(format!("{}m", num))
    } else if value.ends_with("H") {
        let num = value[..value.len()-1].parse::<f64>()?;
        Ok(format!("{}H", num))
    } else {
        // If no unit specified, assume Henries
        Ok(format!("{}H", value))
    }
}

fn normalize_voltage_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = value.trim();
    
    if value.ends_with("V") || value.ends_with("v") {
        Ok(value[..value.len()-1].to_string())
    } else {
        Ok(value.to_string())
    }
}

fn normalize_current_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = value.trim();
    
    if value.ends_with("A") || value.ends_with("a") {
        Ok(value[..value.len()-1].to_string())
    } else {
        Ok(value.to_string())
    }
}

fn get_diode_model(component: &Component) -> Result<String, Box<dyn std::error::Error>> {
    // Create a simple diode model based on the diode type
    match component.component_type {
        ComponentType::ZenerDiode => Ok("DZENER".to_string()),
        ComponentType::SchottkyDiode => Ok("DSCHOTTKY".to_string()),
        ComponentType::Led => Ok("DLED".to_string()),
        _ => Ok("DDIODE".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Component, ComponentType, Rotation};
    use std::collections::HashMap;

    #[test]
    fn test_export_spice_simple() {
        let circuit = Circuit {
            components: vec![
                Component {
                    id: "R1".to_string(),
                    component_type: ComponentType::Resistor,
                    value: Some("1k".to_string()),
                    position: Some((0.0, 0.0)),
                    rotation: Rotation::Deg0,
                    label: Some("R1".to_string()),
                    properties: HashMap::new(),
                },
                Component {
                    id: "R2".to_string(),
                    component_type: ComponentType::Resistor,
                    value: Some("2k".to_string()),
                    position: Some((100.0, 0.0)),
                    rotation: Rotation::Deg0,
                    label: Some("R2".to_string()),
                    properties: HashMap::new(),
                },
                Component {
                    id: "V1".to_string(),
                    component_type: ComponentType::DcVoltage,
                    value: Some("5V".to_string()),
                    position: Some((0.0, 50.0)),
                    rotation: Rotation::Deg0,
                    label: Some("V1".to_string()),
                    properties: HashMap::new(),
                },
                Component {
                    id: "GND1".to_string(),
                    component_type: ComponentType::SignalGround,
                    value: None,
                    position: Some((0.0, 100.0)),
                    rotation: Rotation::Deg0,
                    label: Some("GND".to_string()),
                    properties: HashMap::new(),
                },
            ],
            connections: vec![
                Connection {
                    from: ConnectionPoint {
                        component_id: "V1".to_string(),
                        pin: "+".to_string(),
                    },
                    to: ConnectionPoint {
                        component_id: "R1".to_string(),
                        pin: "1".to_string(),
                    },
                    properties: HashMap::new(),
                },
                Connection {
                    from: ConnectionPoint {
                        component_id: "R1".to_string(),
                        pin: "2".to_string(),
                    },
                    to: ConnectionPoint {
                        component_id: "R2".to_string(),
                        pin: "1".to_string(),
                    },
                    properties: HashMap::new(),
                },
                Connection {
                    from: ConnectionPoint {
                        component_id: "R2".to_string(),
                        pin: "2".to_string(),
                    },
                    to: ConnectionPoint {
                        component_id: "GND1".to_string(),
                        pin: "GND".to_string(),
                    },
                    properties: HashMap::new(),
                },
                Connection {
                    from: ConnectionPoint {
                        component_id: "V1".to_string(),
                        pin: "-".to_string(),
                    },
                    to: ConnectionPoint {
                        component_id: "GND1".to_string(),
                        pin: "GND".to_string(),
                    },
                    properties: HashMap::new(),
                },
            ],
            nets: vec![],
        };
        
        let spice = export_spice(&circuit);
        assert!(spice.is_ok());
        
        let spice_content = spice.unwrap();
        assert!(spice_content.contains("R1"));
        assert!(spice_content.contains("R2"));
        assert!(spice_content.contains("V1"));
        assert!(spice_content.contains(".end"));
    }
}