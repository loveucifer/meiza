export interface UnitPrefix {
    symbol: string;
    multiplier: number;
    name: string;
}
export declare function parseValueWithUnit(input: string): number;
export declare function convertToUnit(value: number, targetUnit: string): number;
export declare function formatValueWithUnit(value: number, baseUnit?: string): string;
//# sourceMappingURL=units.d.ts.map