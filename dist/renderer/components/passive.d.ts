/**
 * Component symbol definitions for passive components
 */
export interface ComponentSymbol {
    type: string;
    symbol: string;
    pins: {
        name: string;
        offset: [number, number];
    }[];
}
export declare const resistorSymbol: ComponentSymbol;
export declare const capacitorSymbol: ComponentSymbol;
export declare const inductorSymbol: ComponentSymbol;
export declare const passiveComponents: {
    resistor: ComponentSymbol;
    capacitor: ComponentSymbol;
    inductor: ComponentSymbol;
};
//# sourceMappingURL=passive.d.ts.map