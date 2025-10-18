"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Mieza = exports.renderCanvas = exports.renderSVG = exports.toIR = exports.validateCircuit = exports.parseCircuit = void 0;
const parser_1 = require("./parser/parser");
Object.defineProperty(exports, "parseCircuit", { enumerable: true, get: function () { return parser_1.parseCircuit; } });
const validator_1 = require("./validator/validator");
Object.defineProperty(exports, "validateCircuit", { enumerable: true, get: function () { return validator_1.validateCircuit; } });
const transformer_1 = require("./ir/transformer");
Object.defineProperty(exports, "toIR", { enumerable: true, get: function () { return transformer_1.toIR; } });
const svg_renderer_1 = require("./renderer/svg-renderer");
Object.defineProperty(exports, "renderSVG", { enumerable: true, get: function () { return svg_renderer_1.renderSVG; } });
const canvas_renderer_1 = require("./renderer/canvas-renderer");
Object.defineProperty(exports, "renderCanvas", { enumerable: true, get: function () { return canvas_renderer_1.renderCanvas; } });
class Mieza {
    parse(cdl) {
        return (0, parser_1.parseCircuit)(cdl);
    }
    validate(circuit) {
        return (0, validator_1.validateCircuit)(circuit);
    }
    toIR(circuit) {
        return (0, transformer_1.toIR)(circuit);
    }
    renderSVG(ir, options) {
        return (0, svg_renderer_1.renderSVG)(ir, options);
    }
    renderCanvas(ir, canvas) {
        return (0, canvas_renderer_1.renderCanvas)(ir, canvas);
    }
}
exports.Mieza = Mieza;
exports.default = Mieza;
//# sourceMappingURL=index.js.map