import { ComponentIR, WireIR } from '../ir/types';
export interface LayoutResult {
    components: ComponentIR[];
    wires: WireIR[];
}
export declare class GridLayout {
    private gridSize;
    private margin;
    constructor(gridSize?: number, margin?: number);
    layout(components: ComponentIR[], wires: WireIR[]): LayoutResult;
    private positionComponents;
    private routeWires;
}
export declare function layoutCircuit(components: ComponentIR[], wires: WireIR[]): LayoutResult;
//# sourceMappingURL=layout.d.ts.map