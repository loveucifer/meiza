"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.toIR = toIR;
// Component symbol definitions
const SYMBOLS = {
    'resistor': 'M 0 0 L 10 0 L 15 5 L 20 0 L 30 0', // Simplified zigzag
    'capacitor': 'M 0 0 L 10 0 M 10 -5 L 10 5 M 20 -5 L 20 5 M 20 0 L 30 0', // Two parallel lines
    'inductor': 'M 0 0 L 5 0 C 10 -5, 15 -5, 20 0 C 25 5, 30 5, 35 0 L 40 0', // Series of loops
    'voltage': 'M 0 0 L 10 0 M 10 -5 A 5 5 0 1 0 10 5 M 10 5 A 5 5 0 1 0 10 -5 M 20 0 L 30 0', // Circle with + and -
    'current': 'M 0 0 L 10 0 M 10 0 A 10 10 0 1 1 30 0 M 30 0 L 40 0', // Circle with arrow
    'diode': 'M 0 0 L 15 0 L 20 5 L 15 10 L 0 10 Z M 20 0 L 30 0', // Triangle with line
    'led': 'M 0 0 L 15 0 L 20 5 L 15 10 L 0 10 Z M 20 0 L 30 0 M 25 -5 L 27 -7 M 27 -7 L 28 -5 M 25 -5 L 26 -7', // Diode with light rays
    'npn': 'M 0 0 L 10 0 M 10 -5 L 10 5 M 15 -5 L 10 0 L 15 5 M 10 0 L 20 0', // NPN symbol
    'pnp': 'M 0 0 L 10 0 M 10 -5 L 10 5 M 5 -5 L 10 0 L 5 5 M 10 0 L 20 0', // PNP symbol
    'nmos': 'M 0 0 L 10 0 M 10 -5 L 10 5 M 10 0 L 15 0 M 15 0 L 15 5 M 15 -5 L 15 -10 M 5 -10 L 15 -10 M 15 -10 L 17 -12', // NMOS symbol
    'pmos': 'M 0 0 L 10 0 M 10 -5 L 10 5 M 10 0 L 15 0 M 15 0 L 15 5 M 15 -5 L 15 -10 M 5 -10 L 15 -10 M 13 -12 L 15 -10', // PMOS symbol
    'opamp': 'M 0 0 L 20 10 L 20 -10 Z M 5 -5 L 0 -5 M 5 5 L 0 5 M 15 0 L 20 0', // Triangle symbol
    'and': 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 L 0 10 Z M 25 0 L 30 0', // AND gate
    'or': 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 Z M 25 0 L 30 0', // OR gate
    'not': 'M 0 0 L 10 0 L 15 5 L 10 10 L 0 10 Z M 15 5 L 20 5 M 18 3 L 20 5 L 18 7', // NOT gate (inverter)
    'nand': 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 L 0 10 Z M 25 5 L 30 5 M 28 3 L 30 5 L 28 7', // NAND gate
    'nor': 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 Z M 25 0 L 30 0 M 30 0 L 32 -2', // NOR gate with inversion
    'xor': 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 C 30 -7.5, 30 7.5, 25 0 Z M 25 0 L 30 0', // XOR gate
    'ground': 'M 0 0 L 0 5 M -5 5 L 5 5 M -3 7 L 3 7 M -1 9 L 1 9', // Ground symbol
    'battery': 'M 0 0 L 5 0 M 7 0 L 12 0 M 5 -5 L 5 5 M 7 -3 L 7 3', // Battery symbol
    'switch': 'M 0 0 L 10 5 M 10 5 L 20 0' // Switch symbol
};
// Default pin positions for each component type
const PIN_POSITIONS = {
    'resistor': [
        { name: '1', position: { x: 0, y: 0 }, type: 'input' },
        { name: '2', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'capacitor': [
        { name: '1', position: { x: 0, y: 0 }, type: 'input' },
        { name: '2', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'inductor': [
        { name: '1', position: { x: 0, y: 0 }, type: 'input' },
        { name: '2', position: { x: 40, y: 0 }, type: 'output' }
    ],
    'voltage': [
        { name: '+', position: { x: 0, y: 0 }, type: 'power' },
        { name: '-', position: { x: 30, y: 0 }, type: 'ground' }
    ],
    'current': [
        { name: 'out', position: { x: 0, y: 0 }, type: 'output' },
        { name: 'ref', position: { x: 40, y: 0 }, type: 'ground' }
    ],
    'diode': [
        { name: 'A', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'C', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'led': [
        { name: 'A', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'C', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'npn': [
        { name: 'C', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'B', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'E', position: { x: 0, y: 5 }, type: 'output' }
    ],
    'pnp': [
        { name: 'E', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'B', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'C', position: { x: 0, y: 5 }, type: 'output' }
    ],
    'nmos': [
        { name: 'D', position: { x: 0, y: -10 }, type: 'input' },
        { name: 'G', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'S', position: { x: 0, y: 5 }, type: 'output' }
    ],
    'pmos': [
        { name: 'S', position: { x: 0, y: -10 }, type: 'input' },
        { name: 'G', position: { x: 0, y: 0 }, type: 'input' },
        { name: 'D', position: { x: 0, y: 5 }, type: 'output' }
    ],
    'opamp': [
        { name: '+', position: { x: 0, y: -5 }, type: 'input' },
        { name: '-', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 30, y: 0 }, type: 'output' },
        { name: 'V+', position: { x: 15, y: -10 }, type: 'power' },
        { name: 'V-', position: { x: 15, y: 10 }, type: 'ground' }
    ],
    'and': [
        { name: 'in1', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'in2', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'or': [
        { name: 'in1', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'in2', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'not': [
        { name: 'in', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 30, y: 5 }, type: 'output' }
    ],
    'nand': [
        { name: 'in1', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'in2', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 32, y: 0 }, type: 'output' }
    ],
    'nor': [
        { name: 'in1', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'in2', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 32, y: 0 }, type: 'output' }
    ],
    'xor': [
        { name: 'in1', position: { x: 0, y: -5 }, type: 'input' },
        { name: 'in2', position: { x: 0, y: 5 }, type: 'input' },
        { name: 'out', position: { x: 30, y: 0 }, type: 'output' }
    ],
    'ground': [
        { name: 'gnd', position: { x: 0, y: 0 }, type: 'ground' }
    ],
    'battery': [
        { name: '+', position: { x: 0, y: 0 }, type: 'power' },
        { name: '-', position: { x: 12, y: 0 }, type: 'ground' }
    ],
    'switch': [
        { name: '1', position: { x: 0, y: 0 }, type: 'input' },
        { name: '2', position: { x: 20, y: 0 }, type: 'output' }
    ]
};
function toIR(circuit) {
    const components = [];
    const wires = [];
    // Calculate default positions if not provided
    const positions = calculateDefaultPositions(circuit.components);
    // Transform components
    for (const component of circuit.components) {
        const position = component.position || positions[component.id] || { x: 0, y: 0 };
        const compIR = {
            id: component.id,
            type: component.type,
            symbol: SYMBOLS[component.type],
            position: [position.x, position.y],
            rotation: component.rotation || 0,
            pins: PIN_POSITIONS[component.type] || [],
            label: component.id,
            value: component.value ? `${component.value.value}${component.value.unit || ''}` : undefined
        };
        components.push(compIR);
    }
    // Transform wires
    for (const wire of circuit.wires) {
        // For now, we'll represent the wire with simple start/end points
        // In a real implementation, we'd need to map the node names to actual coordinates
        // This is a simplified representation
        wires.push({
            from: { x: 0, y: 0 }, // Placeholder - would need to look up actual positions
            to: { x: 30, y: 0 }, // Placeholder - would need to look up actual positions
            path: [] // Actual routing path - would be computed by layout algorithm
        });
    }
    // Calculate bounding box
    const bounds = calculateBoundingBox(components, wires);
    return {
        name: circuit.name,
        components,
        wires,
        bounds,
        metadata: {
            width: 800, // Default width
            height: 600 // Default height
        }
    };
}
// Calculate default positions for components that don't have explicit positions
function calculateDefaultPositions(components) {
    const positions = {};
    const gridSpacing = 50; // pixels between components
    let x = 50;
    let y = 50;
    for (const component of components) {
        if (!component.position) {
            positions[component.id] = { x, y };
            // Move to next position in grid
            x += gridSpacing;
            if (x > 400) { // If we reach the end of the row, go to next row
                x = 50;
                y += gridSpacing;
            }
        }
        else {
            positions[component.id] = component.position;
        }
    }
    return positions;
}
// Calculate the bounding box of the circuit
function calculateBoundingBox(components, wires) {
    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;
    // Check component positions
    for (const comp of components) {
        minX = Math.min(minX, comp.position[0]);
        minY = Math.min(minY, comp.position[1]);
        maxX = Math.max(maxX, comp.position[0] + 30); // Assuming component width of 30
        maxY = Math.max(maxY, comp.position[1] + 20); // Assuming component height of 20
    }
    // Check wire positions
    for (const wire of wires) {
        minX = Math.min(minX, wire.from.x, wire.to.x);
        minY = Math.min(minY, wire.from.y, wire.to.y);
        maxX = Math.max(maxX, wire.from.x, wire.to.x);
        maxY = Math.max(maxY, wire.from.y, wire.to.y);
    }
    return {
        x: minX,
        y: minY,
        width: maxX - minX,
        height: maxY - minY
    };
}
//# sourceMappingURL=transformer.js.map