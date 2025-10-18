"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ComponentSymbols = void 0;
const passive_1 = require("./passive");
// Registry of component symbols
exports.ComponentSymbols = {
    resistor: passive_1.resistorSymbol,
    capacitor: passive_1.capacitorSymbol,
    inductor: passive_1.inductorSymbol,
    voltage: passive_1.voltageSymbol,
    current: passive_1.currentSymbol,
    diode: passive_1.diodeSymbol,
    led: passive_1.ledSymbol,
    npn: passive_1.npnSymbol,
    pnp: passive_1.pnpSymbol,
    nmos: passive_1.nmosSymbol,
    pmos: passive_1.pmosSymbol,
    opamp: passive_1.opampSymbol,
    and: passive_1.andSymbol,
    or: passive_1.orSymbol,
    not: passive_1.notSymbol,
    nand: passive_1.nandSymbol,
    nor: passive_1.norSymbol,
    xor: passive_1.xorSymbol,
    ground: passive_1.groundSymbol,
    battery: passive_1.batterySymbol,
    switch: passive_1.switchSymbol
};
//# sourceMappingURL=index.js.map