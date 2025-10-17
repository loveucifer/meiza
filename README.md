# Mieza - Modern Circuit Description Language and Rendering Library

Mieza is a modern circuit description language and rendering library designed for AI systems to understand and generate circuits efficiently. It provides a clean, human-readable format that LLMs can easily generate and a robust Rust implementation for parsing and rendering.

## Features

- **Human-readable format**: Simple, clean syntax that's easy to read and write
- **AI-generatable**: Designed specifically for LLM generation with consistent patterns
- **Multiple symbol standards**: Supports IEEE, IEC, and DIN circuit symbols
- **Cross-platform**: Can compile to WASM for web, FFI for Flutter
- **SPICE export**: Generate SPICE netlists for simulation
- **Auto-layout**: Intelligent positioning of components
- **Full component library**: Supports resistors, capacitors, transistors, ICs, and more

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mieza = { git = "https://github.com/your-repo/mieza" }
```

For WASM support:

```toml
[dependencies]
mieza = { git = "https://github.com/your-repo/mieza", features = ["wasm"] }
```

## Usage

### Basic Usage
```rust
use mieza::{parse_and_render, SvgTheme, SvgStyle};

fn main() {
    let cdl = r#"
        V1 dc_voltage 5V (0, 0) text="Power Supply"
        R1 resistor 220 (50, 0) text="Current Limit"
        D1 led (100, 0) text="Indicator LED"
        GND signal_ground (150, 20) text="Ground"

        V1.+ -> R1.1
        R1.2 -> D1.A
        D1.K -> GND.GND
        V1.- -> GND.GND
    "#;

    let svg = parse_and_render(cdl, SvgTheme::Light, SvgStyle::Ieee);
    match svg {
        Ok(svg_content) => println!("{}", svg_content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Web/WASM Usage
```javascript
import init, { parse_cdl_to_svg } from 'mieza';

async function renderCircuit() {
    await init();

    const cdl = `R1 resistor 1k (0, 0)
                 R2 resistor 2k (50, 0)
                 R1.2 -> R2.1`;

    const svg = parse_cdl_to_svg(cdl, "light", "ieee");
    document.getElementById('circuit').innerHTML = svg;
}
```

### Flutter Usage
Use the C-compatible FFI functions defined in the library.

## CDL Syntax

Mieza uses a clean, intuitive syntax:

```
# Component: ID Type Value (X, Y) Properties
R1 resistor 1k (10, 20) rotation=90 text="Feedback"

# Connection: From -> To
R1.1 -> C1.1

# Comments start with #
V1 dc_voltage 5V (0, 0)  # Power supply
```

See [SYNTAX.md](SYNTAX.md) for complete syntax reference.

## Supported Components

### Passive
- Resistors, Capacitors, Inductors
- Potentiometers, Transformers

### Sources
- DC/AC Voltage and Current Sources
- Signal Generators

### Semiconductors
- Diodes (Standard, Zener, Schottky, LED)
- Transistors (BJT, MOSFET, JFET)

### ICs
- Op-Amps, Comparators
- 555 Timers
- Logic Gates (AND, OR, NOT, NAND, NOR, XOR)
- Flip-Flops, Counters, Multiplexers

### Others
- Voltage Regulators, Crystals
- Relays, Switches, Fuses
- Batteries, Grounds
- Measurement devices (Ammeters, Voltmeters)
- Connectors, Test Points

## Output Styles

Mieza supports three major circuit symbol standards:

- **IEEE**: Standard used in North America
- **IEC**: International Electrotechnical Commission standard
- **DIN**: German Institute for Standardization

## Example Circuits

Several example circuits are included in the `examples/` directory:

- LED driver circuit
- RC low-pass filter
- Transistor amplifier
- 555 timer oscillator
- Op-amp amplifier

## Building

### Standard Build
```bash
cargo build
```

### WASM Build
```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build for web
wasm-pack build --target web
```

## Testing

Run the test suite:
```bash
cargo test
```

## License

MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please submit pull requests for new components, features, or bug fixes.

## Support

For support, please open an issue in the GitHub repository.
