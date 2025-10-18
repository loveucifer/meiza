import { CircuitIR } from '../ir/types';
export interface RendererOptions {
    width: number;
    height: number;
    theme?: 'light' | 'dark';
    showLabels?: boolean;
    showValues?: boolean;
}
export interface Renderer {
    render(circuit: CircuitIR, options: RendererOptions): string | void;
}
export declare abstract class BaseRenderer implements Renderer {
    abstract render(circuit: CircuitIR, options: RendererOptions): string | void;
    protected applyTheme(theme: 'light' | 'dark'): {
        [key: string]: string;
    };
}
//# sourceMappingURL=renderer.d.ts.map