/**
 * Transform AST to JSON IR (Intermediate Representation)
 */
import { Circuit } from '../parser/ast';
export interface BoundingBox {
    x: number;
    y: number;
    width: number;
    height: number;
}
export interface Pin {
    name: string;
    position: [number, number];
}
export interface ComponentIR {
    id: string;
    type: string;
    symbol: string;
    position: [number, number];
    rotation: number;
    pins: Pin[];
    label: string;
    value?: string;
}
export interface WireIR {
    from: [number, number];
    to: [number, number];
    path: [number, number][];
}
export interface CircuitIR {
    name: string;
    components: ComponentIR[];
    wires: WireIR[];
    bounds: BoundingBox;
}
export declare class IRTransformer {
    transform(circuit: Circuit): CircuitIR;
    private getSymbolForComponent;
    private generatePins;
    private transformWires;
}
//# sourceMappingURL=transformer.d.ts.map