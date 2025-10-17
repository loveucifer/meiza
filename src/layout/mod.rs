use crate::components::get_component_template;
use crate::parser::{Circuit, Component};
use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct PositionedComponent {
    pub component: Component,
    pub position: Point,
    pub size: (f64, f64), // width, height
    pub rotation: Rotation,
}

#[derive(Debug, Clone)]
pub struct PositionedConnection {
    pub from: Point,
    pub to: Point,
    pub path: Vec<Point>, // points for routing
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub components: Vec<PositionedComponent>,
    pub connections: Vec<PositionedConnection>,
    pub nets: Vec<Vec<Point>>, // net connections
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    pub fn to_radians(&self) -> f64 {
        match self {
            Rotation::Deg0 => 0.0,
            Rotation::Deg90 => std::f64::consts::PI / 2.0,
            Rotation::Deg180 => std::f64::consts::PI,
            Rotation::Deg270 => 3.0 * std::f64::consts::PI / 2.0,
        }
    }
}

pub fn calculate_layout(circuit: &Circuit) -> anyhow::Result<Layout> {
    let mut positioned_components = Vec::new();
    let mut positioned_connections = Vec::new();

    // First, place components that have explicit positions
    let mut positioned = HashMap::new();
    let mut unpositioned = Vec::new();

    for component in &circuit.components {
        if let Some(pos) = component.position {
            positioned_components.push(PositionedComponent {
                component: component.clone(),
                position: Point { x: pos.0, y: pos.1 },
                size: get_component_size(&component.component_type),
                rotation: convert_rotation(&component.rotation),
            });
            positioned.insert(component.id.clone(), Point { x: pos.0, y: pos.1 });
        } else {
            unpositioned.push(component);
        }
    }

    // Create a graph of connections to help with positioning
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for connection in &circuit.connections {
        adjacency
            .entry(connection.from.component_id.clone())
            .or_insert_with(Vec::new)
            .push(connection.to.component_id.clone());

        adjacency
            .entry(connection.to.component_id.clone())
            .or_insert_with(Vec::new)
            .push(connection.from.component_id.clone());
    }

    // Position unpositioned components based on connections
    for component in unpositioned {
        let position =
            determine_position(component, &positioned, &adjacency, &positioned_components)?;

        positioned_components.push(PositionedComponent {
            component: component.clone(),
            position,
            size: get_component_size(&component.component_type),
            rotation: convert_rotation(&component.rotation),
        });

        positioned.insert(component.id.clone(), position);
    }

    // Calculate connection paths
    for connection in &circuit.connections {
        let from_pos = positioned
            .get(&connection.from.component_id)
            .ok_or_else(|| anyhow!("Component not found"))?;
        let to_pos = positioned
            .get(&connection.to.component_id)
            .ok_or_else(|| anyhow!("Component not found"))?;

        // Get pin positions
        let from_pin_pos = get_pin_position(
            &connection.from.component_id,
            &connection.from.pin,
            from_pos,
            &positioned_components,
        )?;

        let to_pin_pos = get_pin_position(
            &connection.to.component_id,
            &connection.to.pin,
            to_pos,
            &positioned_components,
        )?;

        let path = route_connection(from_pin_pos, to_pin_pos, &positioned_components)?;

        positioned_connections.push(PositionedConnection {
            from: from_pin_pos,
            to: to_pin_pos,
            path,
        });
    }

    Ok(Layout {
        components: positioned_components,
        connections: positioned_connections,
        nets: Vec::new(), // Will be calculated based on connections
    })
}

fn get_component_size(component_type: &crate::parser::ComponentType) -> (f64, f64) {
    match component_type {
        crate::parser::ComponentType::Resistor => (40.0, 10.0),
        crate::parser::ComponentType::Capacitor => (10.0, 40.0),
        crate::parser::ComponentType::Inductor => (40.0, 15.0),
        crate::parser::ComponentType::Potentiometer => (40.0, 25.0),
        crate::parser::ComponentType::Transformer => (60.0, 30.0),
        crate::parser::ComponentType::DcVoltage
        | crate::parser::ComponentType::DcCurrent
        | crate::parser::ComponentType::AcVoltage
        | crate::parser::ComponentType::AcCurrent => (20.0, 40.0),
        crate::parser::ComponentType::SignalGenerator => (40.0, 20.0),
        crate::parser::ComponentType::Diode
        | crate::parser::ComponentType::ZenerDiode
        | crate::parser::ComponentType::SchottkyDiode
        | crate::parser::ComponentType::Led => (20.0, 40.0),
        crate::parser::ComponentType::NpnTransistor
        | crate::parser::ComponentType::PnpTransistor
        | crate::parser::ComponentType::NmosTransistor
        | crate::parser::ComponentType::PmosTransistor
        | crate::parser::ComponentType::Jfet => (40.0, 30.0),
        crate::parser::ComponentType::OpAmp => (50.0, 50.0),
        crate::parser::ComponentType::Comparator => (50.0, 30.0),
        crate::parser::ComponentType::Timer555 => (60.0, 50.0),
        crate::parser::ComponentType::AndGate
        | crate::parser::ComponentType::OrGate
        | crate::parser::ComponentType::NandGate
        | crate::parser::ComponentType::NorGate
        | crate::parser::ComponentType::XorGate => (40.0, 30.0),
        crate::parser::ComponentType::NotGate => (40.0, 20.0),
        crate::parser::ComponentType::FlipFlop => (50.0, 40.0),
        crate::parser::ComponentType::Counter => (60.0, 50.0),
        crate::parser::ComponentType::Multiplexer => (60.0, 50.0),
        crate::parser::ComponentType::VoltageRegulator => (40.0, 30.0),
        crate::parser::ComponentType::Crystal => (15.0, 40.0),
        crate::parser::ComponentType::Relay => (50.0, 40.0),
        crate::parser::ComponentType::SpstSwitch => (30.0, 20.0),
        crate::parser::ComponentType::SpdtSwitch => (30.0, 30.0),
        crate::parser::ComponentType::DpdtSwitch => (40.0, 40.0),
        crate::parser::ComponentType::Fuse => (30.0, 10.0),
        crate::parser::ComponentType::Battery => (30.0, 20.0),
        crate::parser::ComponentType::Microcontroller => (80.0, 80.0),
        crate::parser::ComponentType::Connector => (15.0, 50.0),
        crate::parser::ComponentType::TestPoint => (10.0, 10.0),
        crate::parser::ComponentType::Ammeter => (40.0, 20.0),
        crate::parser::ComponentType::Voltmeter => (20.0, 40.0),
        crate::parser::ComponentType::OscilloscopeProbe => (20.0, 40.0),
        crate::parser::ComponentType::Antenna => (20.0, 20.0),
        crate::parser::ComponentType::Speaker => (30.0, 30.0),
        crate::parser::ComponentType::Microphone => (40.0, 20.0),
        crate::parser::ComponentType::Motor => (30.0, 30.0),
        crate::parser::ComponentType::SignalGround
        | crate::parser::ComponentType::ChassisGround
        | crate::parser::ComponentType::EarthGround => (20.0, 15.0),
    }
}

fn convert_rotation(rotation: &crate::parser::Rotation) -> Rotation {
    match rotation {
        crate::parser::Rotation::Deg0 => Rotation::Deg0,
        crate::parser::Rotation::Deg90 => Rotation::Deg90,
        crate::parser::Rotation::Deg180 => Rotation::Deg180,
        crate::parser::Rotation::Deg270 => Rotation::Deg270,
    }
}

fn determine_position(
    component: &Component,
    positioned: &HashMap<String, Point>,
    adjacency: &HashMap<String, Vec<String>>,
    positioned_components: &[PositionedComponent],
) -> anyhow::Result<Point> {
    // If there are connected components that are already positioned, place near them
    let mut connected_positions = Vec::new();

    for (comp_id, connected_ids) in adjacency {
        if comp_id == &component.id {
            for connected_id in connected_ids {
                if let Some(pos) = positioned.get(connected_id) {
                    connected_positions.push(*pos);
                }
            }
        }
    }

    if !connected_positions.is_empty() {
        // Average position of connected components, with some offset
        let avg_x: f64 = connected_positions.iter().map(|p| p.x).sum();
        let avg_y: f64 = connected_positions.iter().map(|p| p.y).sum();
        let count = connected_positions.len() as f64;

        // Add some spacing
        let spacing = 50.0;
        let new_x = avg_x / count + spacing;
        let new_y = avg_y / count + spacing;

        return Ok(Point { x: new_x, y: new_y });
    }

    // If no connected components are positioned, use a grid layout
    let grid_size = 100.0;
    let index = positioned_components.len();
    let x = (index % 10) as f64 * grid_size;
    let y = (index / 10) as f64 * grid_size;

    Ok(Point { x, y })
}

fn get_pin_position(
    component_id: &str,
    pin_name: &str,
    component_pos: &Point,
    positioned_components: &[PositionedComponent],
) -> anyhow::Result<Point> {
    // Find the component in the positioned components
    let positioned_comp = positioned_components
        .iter()
        .find(|pc| pc.component.id == component_id)
        .ok_or_else(|| anyhow!("Component not found in positioned components"))?;

    // Get the pin definition
    if let Some(template) = get_component_template(
        &format!("{:?}", positioned_comp.component.component_type).to_lowercase(),
    ) {
        if let Some(pin_def) = template.pins.iter().find(|p| p.name == pin_name) {
            // Calculate the actual position considering component position and rotation
            let rotated_pos = rotate_point(
                pin_def.position.0,
                pin_def.position.1,
                positioned_comp.rotation,
            );

            return Ok(Point {
                x: component_pos.x + rotated_pos.0,
                y: component_pos.y + rotated_pos.1,
            });
        }
    }

    // If we can't find the pin, return the component center
    Ok(*component_pos)
}

fn rotate_point(x: f64, y: f64, rotation: Rotation) -> (f64, f64) {
    match rotation {
        Rotation::Deg0 => (x, y),
        Rotation::Deg90 => (-y, x),
        Rotation::Deg180 => (-x, -y),
        Rotation::Deg270 => (y, -x),
    }
}

fn route_connection(
    from: Point,
    to: Point,
    _positioned_components: &[PositionedComponent],
) -> anyhow::Result<Vec<Point>> {
    // Simple Manhattan routing algorithm
    let mut path = Vec::new();
    path.push(from);

    // Find a route that avoids components
    let mut current = from;

    // Go horizontally first, then vertically (L-shaped route)
    if (to.x - current.x).abs() > 1.0 {
        // Add intermediate point halfway between x values
        let mid_x = current.x + (to.x - current.x) / 2.0;
        path.push(Point {
            x: mid_x,
            y: current.y,
        });
        current.x = mid_x;
    }

    if (to.y - current.y).abs() > 1.0 {
        path.push(Point {
            x: current.x,
            y: to.y,
        });
        current.y = to.y;
    }

    if (to.x - current.x).abs() > 1.0 {
        path.push(to);
    } else if path.last().map_or(true, |p| p.x != to.x || p.y != to.y) {
        path.push(to);
    }

    // TODO: Implement obstacle avoidance if needed
    // For now, return the simple L-shaped path

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Component, ComponentType, Rotation as ParserRotation};

    #[test]
    fn test_calculate_layout() {
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
            connections: vec![Connection {
                from: ConnectionPoint {
                    component_id: "R1".to_string(),
                    pin: "2".to_string(),
                },
                to: ConnectionPoint {
                    component_id: "R2".to_string(),
                    pin: "1".to_string(),
                },
                properties: std::collections::HashMap::new(),
            }],
            nets: vec![],
        };

        let layout = calculate_layout(&circuit);
        assert!(layout.is_ok());

        let layout = layout.unwrap();
        assert_eq!(layout.components.len(), 2);
        assert_eq!(layout.connections.len(), 1);
    }
}
