"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CircuitRuntime = void 0;
exports.runCircuit = runCircuit;
class CircuitRuntime {
    constructor(circuit) {
        this.netlist = null;
        this.circuit = circuit;
    }
    buildNetlist() {
        const nets = {};
        const connections = {};
        // Initialize nets and connections
        for (const component of this.circuit.components) {
            connections[component.id] = {};
            for (let i = 0; i < component.nodes.length; i++) {
                const node = component.nodes[i];
                if (!nets[node]) {
                    nets[node] = [];
                }
                // Record this component's connection to this node
                nets[node].push(`${component.id}:${i}`);
                connections[component.id][i] = node;
            }
        }
        this.netlist = {
            components: this.circuit.components,
            nets,
            connections
        };
        return this.netlist;
    }
    findConnectedComponents(node) {
        if (!this.netlist) {
            this.buildNetlist();
        }
        const connectedComponents = [];
        const nodeConnections = this.netlist.nets[node] || [];
        for (const connection of nodeConnections) {
            const [componentId] = connection.split(':');
            const component = this.circuit.components.find(c => c.id === componentId);
            if (component) {
                connectedComponents.push(component);
            }
        }
        return connectedComponents;
    }
    getNodes() {
        if (!this.netlist) {
            this.buildNetlist();
        }
        return Object.keys(this.netlist.nets);
    }
    detectShortCircuits() {
        if (!this.netlist) {
            this.buildNetlist();
        }
        const shortCircuits = [];
        for (const [netName, connections] of Object.entries(this.netlist.nets)) {
            // A short circuit could occur if there are voltage sources with different values
            // connected directly to the same net
            const voltageSources = connections.filter(conn => {
                const parts = conn.split(':');
                if (parts.length === 0)
                    return false; // Guard against empty strings
                const compId = parts[0];
                const comp = this.circuit.components.find(c => c.id === compId);
                return comp && comp.type === 'voltage';
            });
            if (voltageSources.length > 1) {
                // Multiple voltage sources on the same net - potential short circuit
                const componentIds = voltageSources.map(conn => {
                    const parts = conn.split(':');
                    return parts[0] || ''; // Default to empty string if split doesn't work
                });
                shortCircuits.push(componentIds);
            }
        }
        return shortCircuits;
    }
    calculateNodeVoltages() {
        if (!this.netlist) {
            this.buildNetlist();
        }
        // Initialize all node voltages to null
        const nodeVoltages = {};
        for (const node of Object.keys(this.netlist.nets)) {
            nodeVoltages[node] = null;
        }
        // Find ground node first (voltage = 0)
        const groundComponent = this.circuit.components.find(c => c.type === 'ground');
        if (groundComponent) {
            // Ground connects to a node, set that node's voltage to 0
            for (const node of groundComponent.nodes) {
                nodeVoltages[node] = 0;
            }
        }
        // Simple DC analysis: find voltage sources and propagate their voltages
        // This is a basic implementation - a full simulator would be much more complex
        const voltageComponents = this.circuit.components.filter(c => c.type === 'voltage' && c.value !== undefined);
        for (const voltageComp of voltageComponents) {
            if (voltageComp.nodes.length >= 2) {
                const posNode = voltageComp.nodes[0]; // Assume first node is positive
                const negNode = voltageComp.nodes[1]; // Assume second node is negative
                // If we know the voltage of one node, we can calculate the other
                if (nodeVoltages[posNode] !== null) {
                    nodeVoltages[negNode] = nodeVoltages[posNode] - (voltageComp.value?.value || 0);
                }
                else if (nodeVoltages[negNode] !== null) {
                    nodeVoltages[posNode] = nodeVoltages[negNode] + (voltageComp.value?.value || 0);
                }
                else {
                    // If neither is known, and we have a ground reference, make an assumption
                    if (nodeVoltages[posNode] === null && voltageComp.value) {
                        // For now, set the positive node to the voltage value if ground is at 0
                        // This is a simplification for DC analysis
                        if (this.hasGround()) {
                            nodeVoltages[posNode] = voltageComp.value.value;
                        }
                    }
                }
            }
        }
        return nodeVoltages;
    }
    hasGround() {
        return this.circuit.components.some(c => c.type === 'ground');
    }
}
exports.CircuitRuntime = CircuitRuntime;
function runCircuit(circuit) {
    return new CircuitRuntime(circuit);
}
//# sourceMappingURL=runtime.js.map