/**
 * Component registry for Circuit Description Language
 */
export declare const componentRegistry: {
    opamp: import("./passive").ComponentSymbol;
    and: import("./passive").ComponentSymbol;
    or: import("./passive").ComponentSymbol;
    not: import("./passive").ComponentSymbol;
    nand: import("./passive").ComponentSymbol;
    nor: import("./passive").ComponentSymbol;
    xor: import("./passive").ComponentSymbol;
    diode: import("./passive").ComponentSymbol;
    led: import("./passive").ComponentSymbol;
    npn: import("./passive").ComponentSymbol;
    pnp: import("./passive").ComponentSymbol;
    nmos: import("./passive").ComponentSymbol;
    pmos: import("./passive").ComponentSymbol;
    voltage: import("./passive").ComponentSymbol;
    current: import("./passive").ComponentSymbol;
    ground: import("./passive").ComponentSymbol;
    battery: import("./passive").ComponentSymbol;
    resistor: import("./passive").ComponentSymbol;
    capacitor: import("./passive").ComponentSymbol;
    inductor: import("./passive").ComponentSymbol;
};
export type ComponentType = keyof typeof componentRegistry;
export declare function getComponentSymbol(type: ComponentType): import("./passive").ComponentSymbol;
export declare function isComponentType(type: string): type is ComponentType;
//# sourceMappingURL=index.d.ts.map