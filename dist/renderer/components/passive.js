"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.switchSymbol = exports.batterySymbol = exports.groundSymbol = exports.xorSymbol = exports.norSymbol = exports.nandSymbol = exports.notSymbol = exports.orSymbol = exports.andSymbol = exports.opampSymbol = exports.pmosSymbol = exports.nmosSymbol = exports.pnpSymbol = exports.npnSymbol = exports.ledSymbol = exports.diodeSymbol = exports.currentSymbol = exports.voltageSymbol = exports.inductorSymbol = exports.capacitorSymbol = exports.resistorSymbol = void 0;
// Resistor symbol (IEEE style)
exports.resistorSymbol = {
    path: 'M 0 0 L 10 0 L 15 5 L 20 0 L 30 0', // Simplified zigzag
    pins: [
        { name: '1', dx: 0, dy: 0, type: 'input' },
        { name: '2', dx: 30, dy: 0, type: 'output' }
    ]
};
// Capacitor symbol
exports.capacitorSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 L 10 5 M 20 -5 L 20 5 M 20 0 L 30 0', // Two parallel lines
    pins: [
        { name: '1', dx: 0, dy: 0, type: 'input' },
        { name: '2', dx: 30, dy: 0, type: 'output' }
    ]
};
// Inductor symbol
exports.inductorSymbol = {
    path: 'M 0 0 L 5 0 C 10 -5, 15 -5, 20 0 C 25 5, 30 5, 35 0 L 40 0', // Series of loops
    pins: [
        { name: '1', dx: 0, dy: 0, type: 'input' },
        { name: '2', dx: 40, dy: 0, type: 'output' }
    ]
};
// Voltage source symbol
exports.voltageSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 A 5 5 0 1 0 10 5 M 10 5 A 5 5 0 1 0 10 -5 M 20 0 L 30 0', // Circle with + and -
    pins: [
        { name: '+', dx: 0, dy: 0, type: 'power' },
        { name: '-', dx: 30, dy: 0, type: 'ground' }
    ]
};
// Current source symbol
exports.currentSymbol = {
    path: 'M 0 0 L 10 0 M 10 0 A 10 10 0 1 1 30 0 M 30 0 L 40 0', // Circle with arrow
    pins: [
        { name: 'out', dx: 0, dy: 0, type: 'output' },
        { name: 'ref', dx: 40, dy: 0, type: 'ground' }
    ]
};
// Diode symbol
exports.diodeSymbol = {
    path: 'M 0 0 L 15 0 L 20 5 L 15 10 L 0 10 Z M 20 0 L 30 0', // Triangle with line
    pins: [
        { name: 'A', dx: 0, dy: 0, type: 'input' },
        { name: 'C', dx: 30, dy: 0, type: 'output' }
    ]
};
// LED symbol
exports.ledSymbol = {
    path: 'M 0 0 L 15 0 L 20 5 L 15 10 L 0 10 Z M 20 0 L 30 0 M 25 -5 L 27 -7 M 27 -7 L 28 -5 M 25 -5 L 26 -7', // Diode with light rays
    pins: [
        { name: 'A', dx: 0, dy: 0, type: 'input' },
        { name: 'C', dx: 30, dy: 0, type: 'output' }
    ]
};
// NPN transistor symbol
exports.npnSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 L 10 5 M 15 -5 L 10 0 L 15 5 M 10 0 L 20 0', // NPN symbol
    pins: [
        { name: 'C', dx: 0, dy: -5, type: 'input' },
        { name: 'B', dx: 0, dy: 0, type: 'input' },
        { name: 'E', dx: 0, dy: 5, type: 'output' }
    ]
};
// PNP transistor symbol
exports.pnpSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 L 10 5 M 5 -5 L 10 0 L 5 5 M 10 0 L 20 0', // PNP symbol
    pins: [
        { name: 'E', dx: 0, dy: -5, type: 'input' },
        { name: 'B', dx: 0, dy: 0, type: 'input' },
        { name: 'C', dx: 0, dy: 5, type: 'output' }
    ]
};
// NMOS transistor symbol
exports.nmosSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 L 10 5 M 10 0 L 15 0 M 15 0 L 15 5 M 15 -5 L 15 -10 M 5 -10 L 15 -10 M 15 -10 L 17 -12', // NMOS symbol
    pins: [
        { name: 'D', dx: 0, dy: -10, type: 'input' },
        { name: 'G', dx: 0, dy: 0, type: 'input' },
        { name: 'S', dx: 0, dy: 5, type: 'output' }
    ]
};
// PMOS transistor symbol
exports.pmosSymbol = {
    path: 'M 0 0 L 10 0 M 10 -5 L 10 5 M 10 0 L 15 0 M 15 0 L 15 5 M 15 -5 L 15 -10 M 5 -10 L 15 -10 M 13 -12 L 15 -10', // PMOS symbol
    pins: [
        { name: 'S', dx: 0, dy: -10, type: 'input' },
        { name: 'G', dx: 0, dy: 0, type: 'input' },
        { name: 'D', dx: 0, dy: 5, type: 'output' }
    ]
};
// Op-amp symbol
exports.opampSymbol = {
    path: 'M 0 0 L 20 10 L 20 -10 Z M 5 -5 L 0 -5 M 5 5 L 0 5 M 15 0 L 20 0', // Triangle symbol
    pins: [
        { name: '+', dx: 0, dy: -5, type: 'input' },
        { name: '-', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 30, dy: 0, type: 'output' },
        { name: 'V+', dx: 15, dy: -10, type: 'power' },
        { name: 'V-', dx: 15, dy: 10, type: 'ground' }
    ]
};
// AND gate symbol
exports.andSymbol = {
    path: 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 L 0 10 Z M 25 0 L 30 0', // AND gate
    pins: [
        { name: 'in1', dx: 0, dy: -5, type: 'input' },
        { name: 'in2', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 30, dy: 0, type: 'output' }
    ]
};
// OR gate symbol
exports.orSymbol = {
    path: 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 Z M 25 0 L 30 0', // OR gate
    pins: [
        { name: 'in1', dx: 0, dy: -5, type: 'input' },
        { name: 'in2', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 30, dy: 0, type: 'output' }
    ]
};
// NOT gate (inverter) symbol
exports.notSymbol = {
    path: 'M 0 0 L 10 0 L 15 5 L 10 10 L 0 10 Z M 15 5 L 20 5 M 18 3 L 20 5 L 18 7', // NOT gate (inverter)
    pins: [
        { name: 'in', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 30, dy: 5, type: 'output' }
    ]
};
// NAND gate symbol
exports.nandSymbol = {
    path: 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 L 0 10 Z M 25 5 L 30 5 M 28 3 L 30 5 L 28 7', // NAND gate
    pins: [
        { name: 'in1', dx: 0, dy: -5, type: 'input' },
        { name: 'in2', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 32, dy: 5, type: 'output' } // Note: output is at 32 due to inversion bubble
    ]
};
// NOR gate symbol
exports.norSymbol = {
    path: 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 C 30 -7.5, 30 7.5, 25 0 Z M 25 0 L 30 0 M 30 0 L 32 -2', // NOR gate with inversion
    pins: [
        { name: 'in1', dx: 0, dy: -5, type: 'input' },
        { name: 'in2', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 32, dy: 0, type: 'output' } // Note: output is at 32 due to inversion
    ]
};
// XOR gate symbol
exports.xorSymbol = {
    path: 'M 0 0 L 10 0 C 15 -10, 25 -10, 25 0 C 25 10, 15 10, 10 0 C 5 5, 5 -5, 10 -10 C 15 -15, 25 -15, 25 0 C 30 -7.5, 30 7.5, 25 0 Z M 25 0 L 30 0', // XOR gate
    pins: [
        { name: 'in1', dx: 0, dy: -5, type: 'input' },
        { name: 'in2', dx: 0, dy: 5, type: 'input' },
        { name: 'out', dx: 30, dy: 0, type: 'output' }
    ]
};
// Ground symbol
exports.groundSymbol = {
    path: 'M 0 0 L 0 5 M -5 5 L 5 5 M -3 7 L 3 7 M -1 9 L 1 9', // Ground symbol
    pins: [
        { name: 'gnd', dx: 0, dy: 0, type: 'ground' }
    ]
};
// Battery symbol
exports.batterySymbol = {
    path: 'M 0 0 L 5 0 M 7 0 L 12 0 M 5 -5 L 5 5 M 7 -3 L 7 3', // Battery symbol
    pins: [
        { name: '+', dx: 0, dy: 0, type: 'power' },
        { name: '-', dx: 12, dy: 0, type: 'ground' }
    ]
};
// Switch symbol
exports.switchSymbol = {
    path: 'M 0 0 L 10 5 M 10 5 L 20 0', // Switch symbol
    pins: [
        { name: '1', dx: 0, dy: 0, type: 'input' },
        { name: '2', dx: 20, dy: 0, type: 'output' }
    ]
};
//# sourceMappingURL=passive.js.map