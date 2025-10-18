"use strict";
/**
 * Component symbol definitions for passive components
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.passiveComponents = exports.inductorSymbol = exports.capacitorSymbol = exports.resistorSymbol = void 0;
exports.resistorSymbol = {
    type: 'resistor',
    symbol: 'M 0,0 L 20,0 L 25,5 L 35,-5 L 45,5 L 55,-5 L 65,5 L 75,-5 L 80,0 L 100,0',
    pins: [
        { name: '1', offset: [0, 0] },
        { name: '2', offset: [100, 0] }
    ]
};
exports.capacitorSymbol = {
    type: 'capacitor',
    symbol: 'M 0,0 L 40,0 M 40,-15 L 40,15 M 60,-15 L 60,15 M 60,0 L 100,0',
    pins: [
        { name: '1', offset: [0, 0] },
        { name: '2', offset: [100, 0] }
    ]
};
exports.inductorSymbol = {
    type: 'inductor',
    symbol: 'M 0,0 L 15,0 C 20,-15 30,-15 35,0 C 40,15 50,15 55,0 C 60,-15 70,-15 75,0 C 80,15 90,15 95,0 L 100,0',
    pins: [
        { name: '1', offset: [0, 0] },
        { name: '2', offset: [100, 0] }
    ]
};
exports.passiveComponents = {
    resistor: exports.resistorSymbol,
    capacitor: exports.capacitorSymbol,
    inductor: exports.inductorSymbol
};
//# sourceMappingURL=passive.js.map