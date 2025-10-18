/**
 * AST (Abstract Syntax Tree) definitions for the Circuit Description Language
 */
export interface Position {
    x: number;
    y: number;
}
export interface Circuit {
    name: string;
    components: Component[];
    wires: Wire[];
    comments: Comment[];
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
export interface Value {
    value: number;
    unit: string;
}
export interface Comment {
    position: Position;
    text: string;
}
export type ComponentType = 'resistor' | 'capacitor' | 'inductor' | 'voltage' | 'current' | 'diode' | 'led' | 'npn' | 'pnp' | 'nmos' | 'pmos' | 'opamp' | 'and' | 'or' | 'not' | 'nand' | 'nor' | 'xor' | 'ground' | 'battery' | 'switch';
//# sourceMappingURL=ast.d.ts.map