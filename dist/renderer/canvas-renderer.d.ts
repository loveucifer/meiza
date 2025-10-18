import { CircuitIR } from '../ir/types';
import { BaseRenderer, RendererOptions } from './renderer';
export declare class CanvasRenderer extends BaseRenderer {
    render(circuit: CircuitIR, options: RendererOptions): void;
    renderToContext(circuit: CircuitIR, context: CanvasRenderingContext2D, options: RendererOptions): void;
    private renderWireToContext;
    private renderComponentToContext;
    private drawPathData;
}
export declare function renderCanvas(circuit: CircuitIR, canvas: HTMLCanvasElement, options?: RendererOptions): void;
//# sourceMappingURL=canvas-renderer.d.ts.map