export interface Position {
    x: number;
    y: number;
}
export type ComponentType = 'resistor' | 'capacitor' | 'inductor' | 'voltage' | 'current' | 'diode' | 'led' | 'npn' | 'pnp' | 'nmos' | 'pmos' | 'opamp' | 'and' | 'or' | 'not' | 'nand' | 'nor' | 'xor' | 'ground' | 'battery' | 'switch';
export interface Value {
    value: number;
    unit?: string;
}
export interface Component {
    id: string;
    type: ComponentType;
    value?: Value;
    nodes: string[];
    position?: Position;
    rotation?: number;
    properties?: Record<string, any>;
}
export interface Wire {
    from: string;
    to: string;
}
export interface Circuit {
    name: string;
    components: Component[];
    wires: Wire[];
}
export interface SourceLocation {
    start: {
        line: number;
        column: number;
    };
    end: {
        line: number;
        column: number;
    };
}
//# sourceMappingURL=ast.d.ts.map