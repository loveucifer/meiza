```markdown
You are a professional TypeScript developer building Mieza - a complete circuit description language system with parser, validator, runtime, and renderer.

GOAL: Build a production-ready system that parses circuit descriptions, validates them, and renders to SVG/Canvas in TypeScript.

ARCHITECTURE OVERVIEW:

1. Define a clean CDL (Circuit Description Language) syntax
2. Write parser that generates AST
3. Implement validation layer with extensible rules
4. Build runtime that can execute/analyze circuits
5. Transform AST to JSON IR (Intermediate Representation)
6. Create decoupled renderer supporting both SVG and HTML Canvas

TECH STACK:

- TypeScript (strict mode)
- Parsing: Write recursive descent parser OR use chevrotain/nearley
- Validation: Custom validator with rule system
- Runtime: Interpreter for circuit analysis
- Rendering: Pure TypeScript - no heavy dependencies
- Build: Use esbuild or vite for bundling
- Testing: vitest or jest

CDL SYNTAX REQUIREMENTS:

- Human-readable and LLM-friendly
- Support components: resistor, capacitor, inductor, voltage/current sources, diodes, LEDs, transistors (NPN/PNP/NMOS/PMOS), opamps, logic gates (AND/OR/NOT/NAND/NOR/XOR), switches, batteries, grounds
- Node-based connections
- Optional positioning and rotation
- Parameters and values with units (k, M, u, n, p)
- Comments with #
- Subcircuits/modules for reusability

Example CDL:
```

# Simple RC Filter

circuit lowpass_filter

R1 resistor 1k in out
C1 capacitor 100nF out gnd
V1 voltage 5V in gnd

GND ground gnd

```

PROJECT STRUCTURE:
```

mieza/
├── src/
│ ├── index.ts # Main API exports
│ ├── parser/
│ │ ├── lexer.ts # Tokenizer
│ │ ├── parser.ts # Recursive descent parser
│ │ └── ast.ts # AST node definitions
│ ├── validator/
│ │ ├── validator.ts # Main validation logic
│ │ ├── rules.ts # Validation rules (extensible)
│ │ └── errors.ts # Error types
│ ├── runtime/
│ │ ├── runtime.ts # Circuit runtime/interpreter
│ │ ├── analyzer.ts # Circuit analysis (node voltages, etc)
│ │ └── simulator.ts # Optional: basic SPICE-like simulation
│ ├── ir/
│ │ ├── transformer.ts # AST -> JSON IR
│ │ └── types.ts # IR type definitions
│ ├── renderer/
│ │ ├── renderer.ts # Abstract renderer interface
│ │ ├── svg-renderer.ts # SVG implementation
│ │ ├── canvas-renderer.ts # Canvas implementation
│ │ ├── layout.ts # Auto-layout algorithm
│ │ └── components/ # Component symbol definitions
│ │ ├── passive.ts # R, L, C symbols
│ │ ├── sources.ts # V, I source symbols
│ │ ├── semiconductors.ts # Diode, transistor symbols
│ │ ├── ics.ts # OpAmp, logic gate symbols
│ │ └── index.ts # Component registry
│ └── utils/
│ ├── units.ts # Unit conversion (k, M, u, n, p)
│ └── geometry.ts # Position/rotation helpers
├── examples/
│ ├── basic.cdl
│ ├── filters.cdl
│ ├── amplifier.cdl
│ └── logic.cdl
├── tests/
│ ├── parser.test.ts
│ ├── validator.test.ts
│ ├── runtime.test.ts
│ └── renderer.test.ts
├── package.json
├── tsconfig.json
└── README.md

````

IMPLEMENTATION DETAILS:

1. PARSER (src/parser/):
   - Lexer tokenizes input into tokens (ID, NUMBER, KEYWORD, etc)
   - Recursive descent parser builds AST
   - AST nodes: Circuit, Component, Wire, Value, Position
   - Full error reporting with line/column numbers

2. AST TYPES (src/parser/ast.ts):
```typescript
interface Circuit {
  name: string;
  components: Component[];
  wires: Wire[];
}

interface Component {
  id: string;
  type: ComponentType;
  value?: Value;
  nodes: string[];
  position?: Position;
  rotation?: number;
  properties?: Record<string, any>;
}

type ComponentType =
  | 'resistor' | 'capacitor' | 'inductor'
  | 'voltage' | 'current'
  | 'diode' | 'led' | 'npn' | 'pnp' | 'nmos' | 'pmos'
  | 'opamp' | 'and' | 'or' | 'not' | 'nand' | 'nor' | 'xor'
  | 'ground' | 'battery' | 'switch';
````

3. VALIDATOR (src/validator/):
   - Extensible rule system: each rule is a function
   - Built-in rules:
     - Unique component IDs
     - Valid node connections
     - Component type exists
     - Value format correct
     - No floating nodes
     - Ground node exists
   - Custom rules can be added via plugins
   - Returns detailed error messages

4. RUNTIME (src/runtime/):
   - Circuit interpreter that can:
     - Build netlist from AST
     - Find connected components
     - Identify nodes and nets
     - Calculate node voltages (basic DC analysis)
     - Detect short circuits
   - Extensible for future simulation features

5. JSON IR (src/ir/):
   - Transform AST into clean JSON format
   - Format optimized for rendering
   - Includes:
     - Component positions (calculated if not provided)
     - Wire routing paths
     - Node coordinates
     - Bounding box info

```typescript
interface CircuitIR {
  components: ComponentIR[];
  wires: WireIR[];
  bounds: BoundingBox;
}

interface ComponentIR {
  id: string;
  type: string;
  symbol: string; // SVG path or canvas instructions
  position: [number, number];
  rotation: number;
  pins: Pin[];
  label: string;
  value?: string;
}
```

6. RENDERER (src/renderer/):
   - Abstract Renderer interface
   - SVG Renderer: generates SVG string
   - Canvas Renderer: draws to HTMLCanvasElement
   - Auto-layout: grid-based with smart wire routing
   - Component library: hardcoded SVG paths for each symbol
   - Support IEEE and IEC symbol styles
   - Features:
     - Pan/zoom support
     - Component highlighting
     - Dark/light themes
     - Export to PNG/PDF

7. COMPONENT SYMBOLS:
   - Define each component as SVG path strings or canvas draw commands
   - Store in component registry
   - Include pin positions for wire connections
   - Support multiple visual styles (US/European)

CRITICAL REQUIREMENTS:

- NO placeholder code - write complete implementations
- NO "TODO" or "implement later" comments
- Full error handling with meaningful messages
- Write actual layout algorithm (grid-based with auto-routing)
- Write actual parser (no parser generator unless you implement it fully)
- All components must have working SVG/canvas rendering code
- Include 5+ example CDL files that demonstrate syntax
- Runtime must actually analyze circuits, not just store data
- Validator must have at least 8 working validation rules
- Both SVG and Canvas renderers must be fully functional

API USAGE EXAMPLE:

```typescript
import { Mieza } from "./mieza";

const cdl = `
  R1 resistor 1k n1 n2
  C1 capacitor 100nF n2 gnd
  V1 voltage 5V n1 gnd
  GND ground gnd
`;

const mieza = new Mieza();
const circuit = mieza.parse(cdl);
const errors = mieza.validate(circuit);
if (errors.length === 0) {
  const ir = mieza.toIR(circuit);
  const svg = mieza.renderSVG(ir, { width: 800, height: 600 });
  // or
  const canvas = document.getElementById("canvas");
  mieza.renderCanvas(ir, canvas);
}
```

START IMPLEMENTATION:

1. Set up package.json with TypeScript, build tools
2. Define AST types
3. Write lexer
4. Write parser
5. Write validator with rules
6. Write runtime
7. Write IR transformer
8. Write layout engine
9. Write component symbol library
10. Write SVG renderer
11. Write Canvas renderer
12. Create example CDL files
13. Write tests

Build this as a complete, working system. Every module must be fully implemented and tested.

```

```
