"use strict";
/**
 * Layout engine for Circuit Description Language renderer
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.LayoutEngine = void 0;
// Grid-based layout with auto-routing
class LayoutEngine {
    constructor() {
        this.gridSize = 20; // pixels
        this.margin = 50; // pixels
    }
    layout(circuitIR) {
        // Create a new circuit IR with updated positions
        const layoutIR = {
            name: circuitIR.name,
            components: [],
            wires: [],
            bounds: circuitIR.bounds
        };
        // Calculate positions for components
        const positionedComponents = this.positionComponents(circuitIR.components, circuitIR.bounds);
        layoutIR.components = positionedComponents;
        // Calculate wire paths
        const wirePaths = this.routeWires(positionedComponents, circuitIR.wires);
        layoutIR.wires = wirePaths;
        return layoutIR;
    }
    positionComponents(components, bounds) {
        // Calculate grid dimensions based on number of components and available space
        const cols = Math.min(components.length, Math.floor((bounds.width - 2 * this.margin) / 120));
        const rows = Math.ceil(components.length / cols);
        // Calculate cell size
        const cellWidth = (bounds.width - 2 * this.margin) / cols;
        const cellHeight = (bounds.height - 2 * this.margin) / rows;
        // Position components in a grid
        return components.map((comp, index) => {
            const row = Math.floor(index / cols);
            const col = index % cols;
            const x = this.margin + col * cellWidth + cellWidth / 2 - 50; // 50 is half the component width
            const y = this.margin + row * cellHeight + cellHeight / 2; // Adjust for component height
            // Update component position and pin positions
            const updatedPins = comp.pins.map((pin) => ({
                name: pin.name,
                position: [pin.position[0] + x - comp.position[0], pin.position[1] + y - comp.position[1]]
            }));
            return {
                ...comp,
                position: [x, y],
                pins: updatedPins
            };
        });
    }
    routeWires(components, originalWires) {
        // This is a simplified wire routing algorithm
        // In a more sophisticated implementation, we would:
        // 1. Identify all connected nodes in the circuit
        // 2. Find components that connect to each node
        // 3. Route wires between the appropriate pins of those components
        const wires = [];
        // For now, we'll create a simplified implementation that connects
        // components that share nodes (based on assumptions about the circuit structure)
        // Since we don't have the original node connectivity information in the IR, 
        // we'll just connect components in a simple pattern
        if (components.length < 2) {
            return wires; // Not enough components to connect
        }
        // Connect components sequentially as a simple demonstration
        for (let i = 0; i < components.length - 1; i++) {
            const fromComp = components[i];
            const toComp = components[i + 1];
            // Use the first available pin of each component to connect
            if (fromComp.pins.length > 0 && toComp.pins.length > 0) {
                const fromPin = fromComp.pins[fromComp.pins.length - 1]; // Last pin of first component
                const toPin = toComp.pins[0]; // First pin of next component
                wires.push({
                    from: fromPin.position,
                    to: toPin.position,
                    path: this.calculateWirePath(fromPin.position, toPin.position)
                });
            }
        }
        return wires;
    }
    calculateWirePath(from, to) {
        // Create a simple path between two points using grid-based routing
        // This is a simplified implementation with one bend point
        // Calculate midpoint for the bend
        const midX = (from[0] + to[0]) / 2;
        // Create path with one bend
        return [
            from,
            [midX, from[1]],
            [midX, to[1]],
            to
        ];
    }
    // Additional utility methods for layout operations
    snapToGrid(position) {
        const x = Math.round(position[0] / this.gridSize) * this.gridSize;
        const y = Math.round(position[1] / this.gridSize) * this.gridSize;
        return [x, y];
    }
    getGridSize() {
        return this.gridSize;
    }
    setGridSize(size) {
        if (size > 0) {
            this.gridSize = size;
        }
    }
}
exports.LayoutEngine = LayoutEngine;
//# sourceMappingURL=layout.js.map