/**
 * SVG renderer for Circuit Description Language
 */
import { CircuitIR } from '../ir/types';
export interface SVGRendererOptions {
    width?: number;
    height?: number;
    theme?: 'light' | 'dark';
    showLabels?: boolean;
    showValues?: boolean;
}
export declare class SVGRenderer {
    private options;
    constructor(options?: SVGRendererOptions);
    render(circuitIR: CircuitIR): string;
    private getStyles;
    private renderComponent;
    private renderWire;
    updateOptions(options: SVGRendererOptions): void;
}
//# sourceMappingURL=svg-renderer.d.ts.map