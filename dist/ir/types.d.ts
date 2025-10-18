import { Position } from '../parser/ast';
export interface BoundingBox {
    x: number;
    y: number;
    width: number;
    height: number;
}
export interface Pin {
    name: string;
    position: Position;
    type: 'input' | 'output' | 'power' | 'ground';
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
    from: Position;
    to: Position;
    path: Position[];
}
export interface CircuitIR {
    name: string;
    components: ComponentIR[];
    wires: WireIR[];
    bounds: BoundingBox;
    metadata: {
        width: number;
        height: number;
    };
}
//# sourceMappingURL=types.d.ts.map