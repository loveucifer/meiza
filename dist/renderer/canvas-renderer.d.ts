/**
 * Canvas renderer for Circuit Description Language
 */
import { CircuitIR } from '../ir/types';
export interface CanvasRendererOptions {
    theme?: 'light' | 'dark';
    showLabels?: boolean;
    showValues?: boolean;
}
export declare class CanvasRenderer {
    private options;
    constructor(options?: CanvasRendererOptions);
    render(circuitIR: CircuitIR, canvas: HTMLCanvasElement): void;
    private clearCanvas;
    private renderComponent;
    private drawSymbol;
    private renderWire;
    updateOptions(options: CanvasRendererOptions): void;
}
//# sourceMappingURL=canvas-renderer.d.ts.map