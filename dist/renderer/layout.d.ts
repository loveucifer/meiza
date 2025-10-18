/**
 * Layout engine for Circuit Description Language renderer
 */
import { CircuitIR } from '../ir/types';
export declare class LayoutEngine {
    private gridSize;
    private margin;
    layout(circuitIR: CircuitIR): CircuitIR;
    private positionComponents;
    private routeWires;
    private calculateWirePath;
    snapToGrid(position: [number, number]): [number, number];
    getGridSize(): number;
    setGridSize(size: number): void;
}
//# sourceMappingURL=layout.d.ts.map