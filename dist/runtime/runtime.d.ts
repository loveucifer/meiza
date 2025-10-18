import { Circuit, Component } from '../parser/ast';
export interface Netlist {
    components: Component[];
    nets: {
        [netName: string]: string[];
    };
    connections: {
        [componentId: string]: {
            [pin: number]: string;
        };
    };
}
export declare class CircuitRuntime {
    private circuit;
    private netlist;
    constructor(circuit: Circuit);
    buildNetlist(): Netlist;
    findConnectedComponents(node: string): Component[];
    getNodes(): string[];
    detectShortCircuits(): string[][];
    calculateNodeVoltages(): {
        [node: string]: number | null;
    };
    private hasGround;
}
export declare function runCircuit(circuit: Circuit): CircuitRuntime;
//# sourceMappingURL=runtime.d.ts.map