import { ComponentType } from '../../parser/ast';
export interface ComponentSymbol {
    path: string;
    pins: Array<{
        name: string;
        dx: number;
        dy: number;
        type: string;
    }>;
}
export declare const ComponentSymbols: {
    [key in ComponentType]: ComponentSymbol;
};
//# sourceMappingURL=index.d.ts.map