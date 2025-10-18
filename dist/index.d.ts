import { parseCircuit } from './parser/parser';
import { validateCircuit } from './validator/validator';
import { toIR } from './ir/transformer';
import { renderSVG } from './renderer/svg-renderer';
import { renderCanvas } from './renderer/canvas-renderer';
export { parseCircuit, validateCircuit, toIR, renderSVG, renderCanvas };
export declare class Mieza {
    parse(cdl: string): import("./parser/ast").Circuit;
    validate(circuit: any): import("./validator/errors").ValidationError[];
    toIR(circuit: any): import("./ir/types").CircuitIR;
    renderSVG(ir: any, options: {
        width: number;
        height: number;
    }): string;
    renderCanvas(ir: any, canvas: HTMLCanvasElement): void;
}
export default Mieza;
//# sourceMappingURL=index.d.ts.map