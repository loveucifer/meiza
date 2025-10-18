import { CircuitIR } from '../ir/types';
import { BaseRenderer, RendererOptions } from './renderer';
export declare class SVGRenderer extends BaseRenderer {
    render(circuit: CircuitIR, options: RendererOptions): string;
    private renderWire;
    private renderComponent;
}
export declare function renderSVG(circuit: CircuitIR, options: RendererOptions): string;
//# sourceMappingURL=svg-renderer.d.ts.map