use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/cdl.pest"]
pub struct CdlParser;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
    pub nets: Vec<Net>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub id: String,
    pub component_type: ComponentType,
    pub value: Option<String>,
    pub properties: HashMap<String, String>,
    pub position: Option<(f64, f64)>,
    pub rotation: Rotation,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    // Passive
    Resistor,
    Capacitor,
    Inductor,
    Potentiometer,
    Transformer,
    // Sources
    DcVoltage,
    DcCurrent,
    AcVoltage,
    AcCurrent,
    SignalGenerator,
    // Semiconductors
    Diode,
    ZenerDiode,
    SchottkyDiode,
    Led,
    NpnTransistor,
    PnpTransistor,
    NmosTransistor,
    PmosTransistor,
    Jfet,
    // ICs
    OpAmp,
    Comparator,
    Timer555,
    AndGate,
    OrGate,
    NotGate,
    NandGate,
    NorGate,
    XorGate,
    FlipFlop,
    Counter,
    Multiplexer,
    // Analog
    VoltageRegulator,
    Crystal,
    Relay,
    SpstSwitch,
    SpdtSwitch,
    DpdtSwitch,
    Fuse,
    Battery,
    // Digital
    Microcontroller,
    Connector,
    TestPoint,
    // Measurement
    Ammeter,
    Voltmeter,
    OscilloscopeProbe,
    // Misc
    Antenna,
    Speaker,
    Microphone,
    Motor,
    SignalGround,
    ChassisGround,
    EarthGround,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    pub from: ConnectionPoint,
    pub to: ConnectionPoint,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionPoint {
    pub component_id: String,
    pub pin: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Net {
    pub name: String,
    pub nodes: Vec<ConnectionPoint>,
}

#[derive(Debug)]
pub enum AstNode {
    Component(Component),
    Connection(Connection),
    Net(Net),
}

pub fn parse_cdl(input: &str) -> anyhow::Result<Circuit> {
    let pairs = CdlParser::parse(Rule::circuit, input)?;

    let mut components = Vec::new();
    let mut connections = Vec::new();
    let mut nets = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::component => {
                let component = parse_component(pair)?;
                components.push(component);
            }
            Rule::connection => {
                let connection = parse_connection(pair)?;
                connections.push(connection);
            }
            Rule::net => {
                let net = parse_net(pair)?;
                nets.push(net);
            }
            Rule::EOI => (),
            _ => return Err(anyhow!("Unexpected rule: {:?}", pair.as_rule())),
        }
    }

    Ok(Circuit {
        components,
        connections,
        nets,
    })
}

fn parse_component(pair: pest::iterators::Pair<Rule>) -> anyhow::Result<Component> {
    let mut component_id = String::new();
    let mut component_type = ComponentType::Resistor; // default
    let mut value = None;
    let mut position = None;
    let mut rotation = Rotation::Deg0;
    let mut label = None;
    let mut properties = HashMap::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                component_id = inner_pair.as_str().to_string();
            }
            Rule::component_type => {
                component_type = parse_component_type(inner_pair)?;
            }
            Rule::value => {
                value = Some(inner_pair.as_str().to_string());
            }
            Rule::position => {
                position = Some(parse_position(inner_pair)?);
            }
            Rule::rotation => {
                rotation = parse_rotation(inner_pair)?;
            }
            Rule::label => {
                label = Some(inner_pair.as_str().to_string());
            }
            Rule::property => {
                let (key, val) = parse_property(inner_pair)?;
                properties.insert(key, val);
            }
            _ => {}
        }
    }

    Ok(Component {
        id: component_id,
        component_type,
        value,
        position,
        rotation,
        label,
        properties,
    })
}

fn parse_component_type(pair: pest::iterators::Pair<Rule>) -> Result<ComponentType> {
    let type_str = pair.as_str().to_lowercase();

    match type_str.as_str() {
        "resistor" | "r" => Ok(ComponentType::Resistor),
        "capacitor" | "c" => Ok(ComponentType::Capacitor),
        "inductor" | "l" => Ok(ComponentType::Inductor),
        "potentiometer" | "pot" => Ok(ComponentType::Potentiometer),
        "transformer" | "t" => Ok(ComponentType::Transformer),
        "dc_voltage" | "dc_v" | "vdc" => Ok(ComponentType::DcVoltage),
        "dc_current" | "dc_i" | "idc" => Ok(ComponentType::DcCurrent),
        "ac_voltage" | "ac_v" | "vac" => Ok(ComponentType::AcVoltage),
        "ac_current" | "ac_i" | "iac" => Ok(ComponentType::AcCurrent),
        "signal_generator" | "sig_gen" | "sg" => Ok(ComponentType::SignalGenerator),
        "diode" | "d" => Ok(ComponentType::Diode),
        "zener_diode" | "zener" | "zd" => Ok(ComponentType::ZenerDiode),
        "schottky_diode" | "schottky" | "sd" => Ok(ComponentType::SchottkyDiode),
        "led" => Ok(ComponentType::Led),
        "npn_transistor" | "npn" => Ok(ComponentType::NpnTransistor),
        "pnp_transistor" | "pnp" => Ok(ComponentType::PnpTransistor),
        "nmos_transistor" | "nmos" => Ok(ComponentType::NmosTransistor),
        "pmos_transistor" | "pmos" => Ok(ComponentType::PmosTransistor),
        "jfet" => Ok(ComponentType::Jfet),
        "op_amp" | "opamp" => Ok(ComponentType::OpAmp),
        "comparator" | "comp" => Ok(ComponentType::Comparator),
        "timer_555" | "555_timer" | "555" => Ok(ComponentType::Timer555),
        "and_gate" | "and" => Ok(ComponentType::AndGate),
        "or_gate" | "or" => Ok(ComponentType::OrGate),
        "not_gate" | "not" | "inverter" => Ok(ComponentType::NotGate),
        "nand_gate" | "nand" => Ok(ComponentType::NandGate),
        "nor_gate" | "nor" => Ok(ComponentType::NorGate),
        "xor_gate" | "xor" => Ok(ComponentType::XorGate),
        "flip_flop" | "ff" => Ok(ComponentType::FlipFlop),
        "counter" => Ok(ComponentType::Counter),
        "multiplexer" | "mux" => Ok(ComponentType::Multiplexer),
        "voltage_regulator" | "regulator" | "reg" => Ok(ComponentType::VoltageRegulator),
        "crystal" | "xtal" => Ok(ComponentType::Crystal),
        "relay" => Ok(ComponentType::Relay),
        "spst_switch" | "spst" => Ok(ComponentType::SpstSwitch),
        "spdt_switch" | "spdt" => Ok(ComponentType::SpdtSwitch),
        "dpdt_switch" | "dpdt" => Ok(ComponentType::DpdtSwitch),
        "fuse" => Ok(ComponentType::Fuse),
        "battery" | "bat" => Ok(ComponentType::Battery),
        "microcontroller" | "mcu" | "u" => Ok(ComponentType::Microcontroller),
        "connector" | "conn" => Ok(ComponentType::Connector),
        "test_point" | "tp" => Ok(ComponentType::TestPoint),
        "ammeter" | "am" => Ok(ComponentType::Ammeter),
        "voltmeter" | "vm" => Ok(ComponentType::Voltmeter),
        "oscilloscope_probe" | "oscope" | "probe" => Ok(ComponentType::OscilloscopeProbe),
        "antenna" => Ok(ComponentType::Antenna),
        "speaker" => Ok(ComponentType::Speaker),
        "microphone" | "mic" => Ok(ComponentType::Microphone),
        "motor" => Ok(ComponentType::Motor),
        "signal_ground" | "sgnd" | "ground" | "gnd" => Ok(ComponentType::SignalGround),
        "chassis_ground" | "cgnd" => Ok(ComponentType::ChassisGround),
        "earth_ground" | "egnd" => Ok(ComponentType::EarthGround),
        _ => Err(anyhow!("Unknown component type: {}", type_str)),
    }
}

fn parse_position(pair: pest::iterators::Pair<Rule>) -> Result<(f64, f64)> {
    let mut coords = pair.into_inner();
    let x = coords
        .next()
        .unwrap()
        .as_str()
        .parse::<f64>()
        .map_err(|_| anyhow!("Invalid X coordinate"))?;
    let y = coords
        .next()
        .unwrap()
        .as_str()
        .parse::<f64>()
        .map_err(|_| anyhow!("Invalid Y coordinate"))?;
    Ok((x, y))
}

fn parse_rotation(pair: pest::iterators::Pair<Rule>) -> Result<Rotation> {
    let rotation_str = pair.as_str();
    match rotation_str {
        "0" | "0deg" => Ok(Rotation::Deg0),
        "90" | "90deg" => Ok(Rotation::Deg90),
        "180" | "180deg" => Ok(Rotation::Deg180),
        "270" | "270deg" => Ok(Rotation::Deg270),
        _ => Err(anyhow!("Invalid rotation value: {}", rotation_str)),
    }
}

fn parse_property(pair: pest::iterators::Pair<Rule>) -> Result<(String, String)> {
    let mut parts = pair.into_inner();
    let key = parts.next().unwrap().as_str().to_string();
    let value = parts.next().unwrap().as_str().to_string();
    Ok((key, value))
}

fn parse_connection(pair: pest::iterators::Pair<Rule>) -> Result<Connection> {
    let mut conn_parts = pair.into_inner();
    let from_str = conn_parts.next().unwrap().as_str();
    let to_str = conn_parts.next().unwrap().as_str();

    let from = parse_connection_point(from_str)?;
    let to = parse_connection_point(to_str)?;

    // Handle properties if they exist
    let properties = if let Some(props) = conn_parts.next() {
        parse_connection_properties(props)?
    } else {
        HashMap::new()
    };

    Ok(Connection {
        from,
        to,
        properties,
    })
}

fn parse_connection_point(point_str: &str) -> Result<ConnectionPoint> {
    // Format: component_id.pin
    let parts: Vec<&str> = point_str.split('.').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid connection point format: {}", point_str));
    }

    Ok(ConnectionPoint {
        component_id: parts[0].to_string(),
        pin: parts[1].to_string(),
    })
}

fn parse_connection_properties(
    pair: pest::iterators::Pair<Rule>,
) -> Result<HashMap<String, String>> {
    let mut properties = HashMap::new();

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::property {
            let (key, val) = parse_property(inner_pair)?;
            properties.insert(key, val);
        }
    }

    Ok(properties)
}

fn parse_net(pair: pest::iterators::Pair<Rule>) -> Result<Net> {
    let mut name = String::new();
    let mut nodes = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                name = inner_pair.as_str().to_string();
            }
            Rule::net_node => {
                for node_pair in inner_pair.into_inner() {
                    let node = parse_connection_point(node_pair.as_str())?;
                    nodes.push(node);
                }
            }
            _ => {}
        }
    }

    Ok(Net { name, nodes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_resistor() {
        let input = "R1 resistor 1k (0, 0)";
        let result = parse_cdl(input);
        assert!(result.is_ok());
        let circuit = result.unwrap();
        assert_eq!(circuit.components.len(), 1);
        assert_eq!(circuit.components[0].id, "R1");
        assert!(matches!(
            circuit.components[0].component_type,
            ComponentType::Resistor
        ));
        assert_eq!(circuit.components[0].value.as_ref().unwrap(), "1k");
    }

    #[test]
    fn test_parse_component_with_rotation() {
        let input = "R1 resistor 10k (10, 20) rotation=90deg";
        let result = parse_cdl(input);
        assert!(result.is_ok());
        let circuit = result.unwrap();
        assert_eq!(circuit.components.len(), 1);
        assert!(matches!(circuit.components[0].rotation, Rotation::Deg90));
    }
}
