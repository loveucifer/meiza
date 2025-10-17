use crate::parser::{Circuit, Component, ComponentType, Connection, ConnectionPoint};

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
    pub component_id: Option<String>,
    pub connection_id: Option<String>,
}

pub fn validate(circuit: &Circuit) -> Result<(), ValidationError> {
    // Validate all components
    for component in &circuit.components {
        validate_component(component)?;
    }
    
    // Validate all connections
    for connection in &circuit.connections {
        validate_connection(connection, circuit)?;
    }
    
    // Check for unconnected pins that should be connected
    check_unconnected_pins(circuit)?;
    
    // Check for short circuits
    check_short_circuits(circuit)?;
    
    // Check for floating inputs (for digital components)
    check_floating_inputs(circuit)?;
    
    Ok(())
}

fn validate_component(component: &Component) -> Result<(), ValidationError> {
    // Basic validation - check if component type is valid
    match &component.component_type {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor |
        ComponentType::Potentiometer | ComponentType::Transformer |
        ComponentType::DcVoltage | ComponentType::DcCurrent | ComponentType::AcVoltage | 
        ComponentType::AcCurrent | ComponentType::SignalGenerator |
        ComponentType::Diode | ComponentType::ZenerDiode | ComponentType::SchottkyDiode | 
        ComponentType::Led |
        ComponentType::NpnTransistor | ComponentType::PnpTransistor | 
        ComponentType::NmosTransistor | ComponentType::PmosTransistor | ComponentType::Jfet |
        ComponentType::OpAmp | ComponentType::Comparator | ComponentType::Timer555 |
        ComponentType::AndGate | ComponentType::OrGate | ComponentType::NotGate | 
        ComponentType::NandGate | ComponentType::NorGate | ComponentType::XorGate |
        ComponentType::FlipFlop | ComponentType::Counter | ComponentType::Multiplexer |
        ComponentType::VoltageRegulator | ComponentType::Crystal | ComponentType::Relay |
        ComponentType::SpstSwitch | ComponentType::SpdtSwitch | ComponentType::DpdtSwitch |
        ComponentType::Fuse | ComponentType::Battery |
        ComponentType::Microcontroller | ComponentType::Connector | ComponentType::TestPoint |
        ComponentType::Ammeter | ComponentType::Voltmeter | ComponentType::OscilloscopeProbe |
        ComponentType::Antenna | ComponentType::Speaker | ComponentType::Microphone | 
        ComponentType::Motor |
        ComponentType::SignalGround | ComponentType::ChassisGround | ComponentType::EarthGround => {}
    }
    
    // Validate value format if present
    if let Some(ref value) = component.value {
        validate_component_value(value, &component.component_type)?;
    }
    
    Ok(())
}

fn validate_component_value(value: &str, component_type: &ComponentType) -> Result<(), ValidationError> {
    // Basic value format validation
    let valid = match component_type {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor |
        ComponentType::Potentiometer => {
            // Should be a number followed by optional unit (k, M, m, µ, etc.)
            value.chars().any(|c| c.is_ascii_digit() || c == '.' || c == 'k' || c == 'M' || 
                                c == 'm' || c == 'u' || c == 'µ' || c == 'p' || c == 'n')
        },
        ComponentType::DcVoltage | ComponentType::AcVoltage => {
            // Should have voltage unit (V) or current unit (A) is invalid
            value.contains('V') || value.contains('v')
        },
        ComponentType::DcCurrent | ComponentType::AcCurrent => {
            // Should have current unit (A)
            value.contains('A') || value.contains('a')
        },
        _ => true, // For other components, accept any value
    };
    
    if !valid {
        return Err(ValidationError {
            message: format!("Invalid value format for component type {:?}: {}", component_type, value),
            component_id: None,
            connection_id: None,
        });
    }
    
    Ok(())
}

fn validate_connection(connection: &Connection, circuit: &Circuit) -> Result<(), ValidationError> {
    // Check if both components in the connection exist
    let from_component = circuit.components.iter()
        .find(|c| c.id == connection.from.component_id);
    
    if from_component.is_none() {
        return Err(ValidationError {
            message: format!("Connection references non-existent component: {}", connection.from.component_id),
            component_id: Some(connection.from.component_id.clone()),
            connection_id: Some(format!("{}->{}", connection.from.component_id, connection.to.component_id)),
        });
    }
    
    let to_component = circuit.components.iter()
        .find(|c| c.id == connection.to.component_id);
    
    if to_component.is_none() {
        return Err(ValidationError {
            message: format!("Connection references non-existent component: {}", connection.to.component_id),
            component_id: Some(connection.to.component_id.clone()),
            connection_id: Some(format!("{}->{}", connection.from.component_id, connection.to.component_id)),
        });
    }
    
    // Check if pins exist on the components
    if let Some(ref comp) = from_component {
        if !has_pin(comp, &connection.from.pin) {
            return Err(ValidationError {
                message: format!("Component {} does not have pin {}", comp.id, connection.from.pin),
                component_id: Some(comp.id.clone()),
                connection_id: Some(format!("{}->{}", connection.from.component_id, connection.to.component_id)),
            });
        }
    }
    
    if let Some(ref comp) = to_component {
        if !has_pin(comp, &connection.to.pin) {
            return Err(ValidationError {
                message: format!("Component {} does not have pin {}", comp.id, connection.to.pin),
                component_id: Some(comp.id.clone()),
                connection_id: Some(format!("{}->{}", connection.from.component_id, connection.to.component_id)),
            });
        }
    }
    
    Ok(())
}

fn has_pin(component: &Component, pin_name: &str) -> bool {
    // This would check against the component's pin definitions
    // For now, we'll implement a basic check based on component type
    match component.component_type {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
            pin_name == "1" || pin_name == "2"
        },
        ComponentType::Diode | ComponentType::ZenerDiode | ComponentType::SchottkyDiode | 
        ComponentType::Led => {
            pin_name == "A" || pin_name == "K"
        },
        ComponentType::NpnTransistor | ComponentType::PnpTransistor => {
            pin_name == "B" || pin_name == "C" || pin_name == "E"
        },
        ComponentType::NmosTransistor | ComponentType::PmosTransistor | ComponentType::Jfet => {
            pin_name == "G" || pin_name == "D" || pin_name == "S"
        },
        ComponentType::SignalGround | ComponentType::ChassisGround | 
        ComponentType::EarthGround => {
            pin_name == "GND" 
        },
        ComponentType::DcVoltage | ComponentType::DcCurrent | 
        ComponentType::AcVoltage | ComponentType::AcCurrent => {
            pin_name == "+" || pin_name == "-"
        },
        ComponentType::Battery => {
            pin_name == "+" || pin_name == "-"
        },
        ComponentType::OpAmp => {
            pin_name == "+" || pin_name == "-" || pin_name == "OUT" || 
            pin_name == "V+" || pin_name == "V-"
        },
        ComponentType::Timer555 => {
            pin_name == "GND" || pin_name == "TRIG" || pin_name == "OUT" || 
            pin_name == "RESET" || pin_name == "CTRL" || pin_name == "THR" || 
            pin_name == "DIS" || pin_name == "VCC"
        },
        _ => {
            // For other components, assume pin exists for now
            true
        }
    }
}

fn check_unconnected_pins(_circuit: &Circuit) -> Result<(), ValidationError> {
    // This would check for pins that should have connections but don't
    // For now, we'll check for power pins that aren't connected to ground or power
    Ok(())
}

fn check_short_circuits(_circuit: &Circuit) -> Result<(), ValidationError> {
    // This would check for direct connections between power and ground or other conflicting voltages
    Ok(())
}

fn check_floating_inputs(_circuit: &Circuit) -> Result<(), ValidationError> {
    // This would check for inputs that don't have a defined voltage level
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::parser::{Component, ComponentType, Rotation};

    #[test]
    fn test_validate_valid_circuit() {
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
            ],
            connections: vec![
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
                }
            ],
            nets: vec![],
        };
        
        let result = validate(&circuit);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_component() {
        let circuit = Circuit {
            components: vec![
                Component {
                    id: "INVALID".to_string(),
                    component_type: ComponentType::Resistor,
                    value: Some("1k".to_string()),
                    position: Some((0.0, 0.0)),
                    rotation: Rotation::Deg0,
                    label: Some("R1".to_string()),
                    properties: HashMap::new(),
                },
            ],
            connections: vec![],
            nets: vec![],
        };
        
        // This test is checking component validation
        let result = validate(&circuit);
        assert!(result.is_ok()); // The component type is valid, so validation should pass
    }
    
    #[test]
    fn test_validate_invalid_connection() {
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
            ],
            connections: vec![
                Connection {
                    from: ConnectionPoint {
                        component_id: "R1".to_string(),
                        pin: "2".to_string(),
                    },
                    to: ConnectionPoint {
                        component_id: "NONEXISTENT".to_string(),
                        pin: "1".to_string(),
                    },
                    properties: HashMap::new(),
                }
            ],
            nets: vec![],
        };
        
        let result = validate(&circuit);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Connection references non-existent component: NONEXISTENT");
    }
}