"use strict";
/**
 * Runtime interpreter for Circuit Description Language
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.Runtime = void 0;
class Runtime {
    constructor() {
        this.circuit = null;
    }
    loadCircuit(circuit) {
        this.circuit = circuit;
    }
    getNetlist() {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        const netlist = {};
        // Initialize netlist with all nodes
        for (const component of this.circuit.components) {
            for (const node of component.nodes) {
                if (!netlist[node]) {
                    netlist[node] = [];
                }
                netlist[node].push(component);
            }
        }
        return netlist;
    }
    getConnectedComponents(node) {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        const connected = [];
        for (const component of this.circuit.components) {
            if (component.nodes.includes(node)) {
                connected.push(component);
            }
        }
        return connected;
    }
    identifyNodes() {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        const nodes = new Set();
        for (const component of this.circuit.components) {
            for (const node of component.nodes) {
                nodes.add(node);
            }
        }
        return Array.from(nodes);
    }
    calculateNodeVoltages() {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        const nodeVoltages = {};
        const netlist = this.getNetlist();
        // Find ground node (should be 0V)
        const groundComponents = this.circuit.components.filter(c => c.type === 'ground');
        if (groundComponents.length === 0) {
            throw new Error('No ground node found in circuit');
        }
        // Assign 0V to ground node
        for (const ground of groundComponents) {
            for (const node of ground.nodes) {
                nodeVoltages[node] = 0;
            }
        }
        // Simple DC analysis - find voltage sources and assign voltages
        const voltageSources = this.circuit.components.filter(c => c.type === 'voltage');
        for (const vs of voltageSources) {
            if (vs.value && vs.nodes.length === 2) {
                const [posNode, negNode] = vs.nodes;
                // If the negative node is grounded, assign voltage to positive
                if (nodeVoltages[negNode] === 0) {
                    nodeVoltages[posNode] = vs.value.value;
                }
                else if (nodeVoltages[negNode] !== undefined) {
                    nodeVoltages[posNode] = nodeVoltages[negNode] + vs.value.value;
                }
                // In a more complete implementation, we would solve the complete system of equations
            }
        }
        return nodeVoltages;
    }
    detectShortCircuits() {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        const shortCircuits = [];
        const netlist = this.getNetlist();
        // For each node, check if there are conflicting voltage sources
        for (const [node, components] of Object.entries(netlist)) {
            const voltageSources = components.filter(c => c.type === 'voltage');
            if (voltageSources.length > 1) {
                // Multiple voltage sources connected to the same node - possible short circuit
                const connectedIds = voltageSources.map(c => c.id);
                shortCircuits.push([node, ...connectedIds]);
            }
        }
        return shortCircuits;
    }
    analyzeCircuit() {
        if (!this.circuit) {
            throw new Error('No circuit loaded');
        }
        // For now, just return basic analysis results
        const nodeVoltages = this.calculateNodeVoltages();
        const componentCurrents = {};
        // For a more complete implementation, we would perform circuit simulation
        for (const component of this.circuit.components) {
            // Default to 0 current for all components
            componentCurrents[component.id] = 0;
        }
        return {
            nodeVoltages,
            componentCurrents
        };
    }
}
exports.Runtime = Runtime;
//# sourceMappingURL=runtime.js.map