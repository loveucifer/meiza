/**
 * Runtime interpreter for Circuit Description Language
 */
import { Circuit, Component } from '../parser/ast';
export interface Netlist {
    [nodeName: string]: Component[];
}
export interface NodeVoltages {
    [nodeName: string]: number;
}
export interface SimulationResult {
    nodeVoltages: NodeVoltages;
    componentCurrents: {
        [componentId: string]: number;
    };
}
export declare class Runtime {
    private circuit;
    constructor();
    loadCircuit(circuit: Circuit): void;
    getNetlist(): Netlist;
    getConnectedComponents(node: string): Component[];
    identifyNodes(): string[];
    calculateNodeVoltages(): NodeVoltages;
    detectShortCircuits(): string[][];
    analyzeCircuit(): SimulationResult;
}
//# sourceMappingURL=runtime.d.ts.map