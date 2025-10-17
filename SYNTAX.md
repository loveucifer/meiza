# Mieza Circuit Description Language (CDL) Syntax Reference

Mieza CDL is a human-readable and AI-generatable language for describing electronic circuits. The syntax is designed to be intuitive and unambiguous.

## Basic Syntax Structure

### Component Declaration
```
<component_id> <component_type> <value> (<x>, <y>) [rotation=<degrees>] [text="<label>"] [property=value]
```

Example:
```
R1 resistor 1k (10, 20) rotation=90 text="Feedback Resistor"
```

### Connection Declaration
```
<component_id>.<pin> -> <component_id>.<pin>
```

Example:
```
R1.1 -> C1.1
```

## Component Types

### Passive Components
- `resistor` or `r` - Resistor
- `capacitor` or `c` - Capacitor
- `inductor` or `l` - Inductor
- `potentiometer` or `pot` - Potentiometer
- `transformer` or `t` - Transformer

### Sources
- `dc_voltage` or `dc_v` or `vdc` - DC Voltage Source
- `dc_current` or `dc_i` or `idc` - DC Current Source
- `ac_voltage` or `ac_v` or `vac` - AC Voltage Source
- `ac_current` or `ac_i` or `iac` - AC Current Source
- `signal_generator` or `sig_gen` or `sg` - Signal Generator

### Semiconductors
- `diode` or `d` - Standard Diode
- `zener_diode` or `zener` or `zd` - Zener Diode
- `schottky_diode` or `schottky` or `sd` - Schottky Diode
- `led` - LED
- `npn_transistor` or `npn` - NPN Transistor
- `pnp_transistor` or `pnp` - PNP Transistor
- `nmos_transistor` or `nmos` - N-Channel MOSFET
- `pmos_transistor` or `pmos` - P-Channel MOSFET
- `jfet` - JFET

### Integrated Circuits
- `op_amp` or `opamp` - Operational Amplifier
- `comparator` or `comp` - Comparator
- `timer_555` or `555_timer` or `555` - 555 Timer
- `and_gate` or `and` - AND Gate
- `or_gate` or `or` - OR Gate
- `not_gate` or `not` or `inverter` - NOT Gate
- `nand_gate` or `nand` - NAND Gate
- `nor_gate` or `nor` - NOR Gate
- `xor_gate` or `xor` - XOR Gate
- `flip_flop` or `ff` - Flip-Flop
- `counter` - Counter
- `multiplexer` or `mux` - Multiplexer

### Analog Components
- `voltage_regulator` or `regulator` or `reg` - Voltage Regulator
- `crystal` or `xtal` - Crystal
- `relay` - Relay
- `spst_switch` or `spst` - SPST Switch
- `spdt_switch` or `spdt` - SPDT Switch
- `dpdt_switch` or `dpdt` - DPDT Switch
- `fuse` - Fuse
- `battery` or `bat` - Battery

### Digital Components
- `microcontroller` or `mcu` or `u` - Microcontroller
- `connector` or `conn` - Connector
- `test_point` or `tp` - Test Point

### Measurement Components
- `ammeter` or `am` - Ammeter
- `voltmeter` or `vm` - Voltmeter
- `oscilloscope_probe` or `oscope` or `probe` - Oscilloscope Probe

### Grounds
- `signal_ground` or `sgnd` or `ground` or `gnd` - Signal Ground
- `chassis_ground` or `cgnd` - Chassis Ground
- `earth_ground` or `egnd` - Earth Ground

### Miscellaneous
- `antenna` - Antenna
- `speaker` - Speaker
- `microphone` or `mic` - Microphone
- `motor` - Motor

## Pin Names

### Common Pin Names
- Resistors, Capacitors, Inductors: `1`, `2`
- Diodes: `A` (Anode), `K` (Cathode)
- Transistors: `B` (Base), `C` (Collector), `E` (Emitter) for BJTs; `G` (Gate), `D` (Drain), `S` (Source) for FETs
- Op-Amps: `+` (Non-inverting input), `-` (Inverting input), `OUT` (Output), `V+` (Positive supply), `V-` (Negative supply)
- 555 Timer: `GND`, `TRIG`, `OUT`, `RESET`, `CTRL`, `THR`, `DIS`, `VCC`
- Logic Gates: `A`, `B`, `C` (inputs), `Y` (output)
- Grounds: `GND`

## Value Formats

### Resistance
- `100` (100Ω)
- `1k` (1kΩ)
- `10k` (10kΩ)
- `1M` (1MΩ)

### Capacitance
- `100pF` (100 picofarads)
- `10nF` (10 nanofarads)
- `1µF` (1 microfarad)
- `10uF` (alternative to µF)
- `1mF` (1 millifarad)

### Inductance
- `10µH` (10 microhenries)
- `100uH` (alternative to µH)
- `1mH` (1 millihenry)
- `10H` (10 henries)

### Voltage
- `5V` (5 volts DC)
- `12V` (12 volts DC)
- `1V` (1 volt AC)

### Current
- `1A` (1 ampere)
- `100mA` (100 milliamperes)
- `1uA` (1 microampere)

## Positioning and Rotation

### Position
Components can be placed at specific coordinates:
```
(10, 20)  # x=10, y=20
```

### Rotation
Components can be rotated:
- `rotation=0` or `rotation=0deg` - No rotation
- `rotation=90` or `rotation=90deg` - 90 degrees clockwise
- `rotation=180` or `rotation=180deg` - 180 degrees
- `rotation=270` or `rotation=270deg` - 270 degrees clockwise

## Comments

Comments start with `#`:
```
# This is a comment
R1 resistor 1k (0, 0)  # Inline comment
```

## Examples

### Simple LED Circuit
```
V1 dc_voltage 5V (0, 0) text="Power Supply"
R1 resistor 220 (50, 0) text="Current Limit"
D1 led (100, 0) text="Indicator LED"
GND signal_ground (150, 20) text="Ground"

V1.+ -> R1.1
R1.2 -> D1.A
D1.K -> GND.GND
V1.- -> GND.GND
```

### RC Low-pass Filter
```
VIN ac_voltage 1V (0, 0) text="Input"
R1 resistor 1k (50, 0) text="Series Resistor"
C1 capacitor 10uF (100, 0) text="Filter Capacitor"
VOUT test_point (150, 0) text="Output"
GND signal_ground (100, 50) text="Ground"

VIN.+ -> R1.1
R1.2 -> C1.1
C1.2 -> GND.GND
R1.2 -> VOUT.TP
VIN.- -> GND.GND
```

## Features

### Positioning
Components can be positioned explicitly or will be auto-positioned if no coordinates are provided.

### Themed Output
Output can be rendered in light or dark themes with IEEE, IEC, or DIN symbols.

### Net Naming (Future)
```
net power_supply (VCC.+, R1.1, U1.VCC)
```

### Properties
Additional properties can be added to components and connections:
```
R1 resistor 1k (0, 0) tolerance=5% power=0.25W text="Feedback"
```

## Integration

### WASM Integration
Mieza can be compiled to WASM for use in web applications:
```javascript
import init, { parse_cdl_to_svg } from './pkg/mieza.js';
```

### Flutter Integration
Mieza provides C-compatible FFI for Flutter/Dart integration:
```c
char* svg = parse_cdl_to_svg(cdl_input, theme, style, &length);
```

### SPICE Export
Circuits can be exported to SPICE netlists:
```
* Mieza SPICE Netlist
V1 N1 0 DC 5
R1 N1 N2 1k
D1 N2 0 D1MODEL
.model D1MODEL D
.end
```