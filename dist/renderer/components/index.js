"use strict";
/**
 * Component registry for Circuit Description Language
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.isComponentType = exports.getComponentSymbol = exports.componentRegistry = void 0;
const passive_1 = require("./passive");
const sources_1 = require("./sources");
const semiconductors_1 = require("./semiconductors");
const ics_1 = require("./ics");
// Combine all component symbols into a single registry
exports.componentRegistry = {
    ...passive_1.passiveComponents,
    ...sources_1.sources,
    ...semiconductors_1.semiconductors,
    ...ics_1.ics
};
function getComponentSymbol(type) {
    return exports.componentRegistry[type];
}
exports.getComponentSymbol = getComponentSymbol;
function isComponentType(type) {
    return type in exports.componentRegistry;
}
exports.isComponentType = isComponentType;
//# sourceMappingURL=index.js.map