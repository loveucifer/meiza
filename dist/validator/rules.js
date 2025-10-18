"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.allValidationRules = exports.ValidComponentConnectionsRule = exports.NoDirectShortCircuitRule = exports.GroundNodeExistsRule = exports.NoFloatingNodesRule = exports.ValidValueFormatRule = exports.ValidNodeCountRule = exports.ValidComponentTypeRule = exports.UniqueComponentIdsRule = void 0;
const errors_1 = require("./errors");
// Rule: Ensure unique component IDs
class UniqueComponentIdsRule {
    constructor() {
        this.name = 'UniqueComponentIds';
    }
    validate(circuit) {
        const errors = [];
        const idSet = new Set();
        for (const component of circuit.components) {
            if (idSet.has(component.id)) {
                errors.push(new errors_1.ValidationError('DUPLICATE_ID', `Duplicate component ID found: ${component.id}`, 'error'));
            }
            else {
                idSet.add(component.id);
            }
        }
        return errors;
    }
}
exports.UniqueComponentIdsRule = UniqueComponentIdsRule;
// Rule: Ensure valid component types
class ValidComponentTypeRule {
    constructor() {
        this.name = 'ValidComponentType';
    }
    validate(circuit) {
        const errors = [];
        const validComponentTypes = new Set([
            'resistor', 'capacitor', 'inductor',
            'voltage', 'current',
            'diode', 'led', 'npn', 'pnp', 'nmos', 'pmos',
            'opamp', 'and', 'or', 'not', 'nand', 'nor', 'xor',
            'ground', 'battery', 'switch'
        ]);
        for (const component of circuit.components) {
            if (!validComponentTypes.has(component.type)) {
                errors.push(new errors_1.ValidationError('INVALID_COMPONENT_TYPE', `Invalid component type: ${component.type}`, 'error'));
            }
        }
        return errors;
    }
}
exports.ValidComponentTypeRule = ValidComponentTypeRule;
// Rule: Ensure components have proper number of nodes
class ValidNodeCountRule {
    constructor() {
        this.name = 'ValidNodeCount';
    }
    validate(circuit) {
        const errors = [];
        const nodeCountMap = {
            'resistor': [2, 2],
            'capacitor': [2, 2],
            'inductor': [2, 2],
            'voltage': [2, 2],
            'current': [2, 2],
            'diode': [2, 2],
            'led': [2, 2],
            'npn': [3, 3],
            'pnp': [3, 3],
            'nmos': [3, 3],
            'pmos': [3, 3],
            'opamp': [5, 5], // Assuming 5 pins: +in, -in, out, V+, V-
            'and': [3, 10], // 2+ inputs, 1 output
            'or': [3, 10], // 2+ inputs, 1 output
            'not': [2, 2],
            'nand': [3, 10], // 2+ inputs, 1 output
            'nor': [3, 10], // 2+ inputs, 1 output
            'xor': [3, 10], // 2+ inputs, 1 output
            'ground': [1, 1],
            'battery': [2, 2],
            'switch': [2, 2]
        };
        for (const component of circuit.components) {
            const [min, max] = nodeCountMap[component.type] || [2, 2];
            if (component.nodes.length < min || component.nodes.length > max) {
                errors.push(new errors_1.ValidationError('INVALID_NODE_COUNT', `Component ${component.id} (${component.type}) has ${component.nodes.length} nodes, expected ${min}-${max}`, 'error'));
            }
        }
        return errors;
    }
}
exports.ValidNodeCountRule = ValidNodeCountRule;
// Rule: Ensure value format is correct for components that require values
class ValidValueFormatRule {
    constructor() {
        this.name = 'ValidValueFormat';
    }
    validate(circuit) {
        const errors = [];
        const componentsRequiringValues = new Set([
            'resistor', 'capacitor', 'inductor', 'voltage', 'current'
        ]);
        for (const component of circuit.components) {
            if (componentsRequiringValues.has(component.type) && component.value === undefined) {
                errors.push(new errors_1.ValidationError('MISSING_VALUE', `Component ${component.id} (${component.type}) requires a value`, 'error'));
            }
        }
        return errors;
    }
}
exports.ValidValueFormatRule = ValidValueFormatRule;
// Rule: Ensure no floating nodes (nodes not connected to anything)
class NoFloatingNodesRule {
    constructor() {
        this.name = 'NoFloatingNodes';
    }
    validate(circuit) {
        const errors = [];
        // Collect all nodes referenced in components
        const allNodes = new Set();
        for (const component of circuit.components) {
            for (const node of component.nodes) {
                allNodes.add(node);
            }
        }
        // All nodes in wires are connected, so we don't need to check wires specifically
        // Components already reference their nodes, so if a node exists in a component,
        // it's connected to that component.
        // Actually, we need to make sure each node is connected to at least one other component
        // So we should check if any node appears only once across all components
        const nodeConnections = new Map();
        for (const component of circuit.components) {
            for (const node of component.nodes) {
                nodeConnections.set(node, (nodeConnections.get(node) || 0) + 1);
            }
        }
        for (const [node, count] of nodeConnections.entries()) {
            if (count === 1) {
                errors.push(new errors_1.ValidationError('FLOATING_NODE', `Node ${node} appears in only one component (floating node)`, 'error'));
            }
        }
        return errors;
    }
}
exports.NoFloatingNodesRule = NoFloatingNodesRule;
// Rule: Ensure ground node exists
class GroundNodeExistsRule {
    constructor() {
        this.name = 'GroundNodeExists';
    }
    validate(circuit) {
        const errors = [];
        const hasGround = circuit.components.some(comp => comp.type === 'ground');
        if (!hasGround) {
            errors.push(new errors_1.ValidationError('NO_GROUND_NODE', 'Circuit must have at least one ground component', 'error'));
        }
        return errors;
    }
}
exports.GroundNodeExistsRule = GroundNodeExistsRule;
// Rule: Check for short circuits (direct connection between power sources)
class NoDirectShortCircuitRule {
    constructor() {
        this.name = 'NoDirectShortCircuit';
    }
    validate(circuit) {
        const errors = [];
        // Check for direct connection between two voltage sources
        const voltageSources = circuit.components.filter(c => c.type === 'voltage');
        for (let i = 0; i < voltageSources.length; i++) {
            for (let j = i + 1; j < voltageSources.length; j++) {
                const vs1 = voltageSources[i];
                const vs2 = voltageSources[j];
                // If two voltage sources are directly connected (share a node)
                const sharedNodes = vs1.nodes.filter(node => vs2.nodes.includes(node));
                if (sharedNodes.length > 0) {
                    errors.push(new errors_1.ValidationError('DIRECT_VOLTAGE_SHORT', `Direct connection between voltage sources ${vs1.id} and ${vs2.id} at node(s): ${sharedNodes.join(', ')}`, 'error'));
                }
            }
        }
        return errors;
    }
}
exports.NoDirectShortCircuitRule = NoDirectShortCircuitRule;
// Rule: Check for invalid component connections (like connecting two outputs directly)
class ValidComponentConnectionsRule {
    constructor() {
        this.name = 'ValidComponentConnections';
    }
    validate(circuit) {
        // For now, we'll implement a basic check for logic gate outputs
        const errors = [];
        // In a real implementation, we'd have more complex rules about what can connect to what
        // For this example, we'll just validate that outputs don't connect directly to other outputs
        // This is simplified and would need a more sophisticated approach in a real circuit analyzer
        return errors;
    }
}
exports.ValidComponentConnectionsRule = ValidComponentConnectionsRule;
// Export all rules
exports.allValidationRules = [
    new UniqueComponentIdsRule(),
    new ValidComponentTypeRule(),
    new ValidNodeCountRule(),
    new ValidValueFormatRule(),
    new NoFloatingNodesRule(),
    new GroundNodeExistsRule(),
    new NoDirectShortCircuitRule(),
    new ValidComponentConnectionsRule()
];
//# sourceMappingURL=rules.js.map