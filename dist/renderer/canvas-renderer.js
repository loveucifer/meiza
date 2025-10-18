"use strict";
/**
 * Canvas renderer for Circuit Description Language
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.CanvasRenderer = void 0;
const components_1 = require("./components");
class CanvasRenderer {
    constructor(options) {
        this.options = {
            theme: 'light',
            showLabels: true,
            showValues: true,
            ...options
        };
    }
    render(circuitIR, canvas) {
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2D context from canvas');
        }
        // Set canvas dimensions
        canvas.width = circuitIR.bounds.width;
        canvas.height = circuitIR.bounds.height;
        // Clear canvas
        this.clearCanvas(ctx, circuitIR.bounds);
        // Render wires first (so components appear on top)
        for (const wire of circuitIR.wires) {
            this.renderWire(ctx, wire);
        }
        // Render components
        for (const component of circuitIR.components) {
            this.renderComponent(ctx, component);
        }
    }
    clearCanvas(ctx, bounds) {
        const bgColor = this.options.theme === 'dark' ? '#1e1e1e' : '#ffffff';
        ctx.fillStyle = bgColor;
        ctx.fillRect(0, 0, bounds.width, bounds.height);
    }
    renderComponent(ctx, component) {
        const symbol = (0, components_1.getComponentSymbol)(component.type);
        if (!symbol) {
            console.warn(`Unknown component type: ${component.type}`);
            return;
        }
        // Save the current context state
        ctx.save();
        // Apply transformations (position and rotation)
        const [x, y] = component.position;
        ctx.translate(x, y);
        ctx.rotate(component.rotation * Math.PI / 180); // Convert degrees to radians
        // Draw the component symbol
        this.drawSymbol(ctx, symbol.symbol);
        // Draw label if enabled
        if (this.options.showLabels) {
            ctx.fillStyle = this.options.theme === 'dark' ? '#ffffff' : '#000000';
            ctx.font = '12px Arial';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(component.label, 50, -10);
        }
        // Draw value if present and enabled
        if (component.value && this.options.showValues) {
            ctx.fillStyle = this.options.theme === 'dark' ? '#ffffff' : '#000000';
            ctx.font = '10px Arial';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(component.value, 50, 15);
        }
        // Restore the context state
        ctx.restore();
    }
    drawSymbol(ctx, symbol) {
        // Set drawing style
        ctx.strokeStyle = this.options.theme === 'dark' ? '#8080ff' : '#000000';
        ctx.lineWidth = 2;
        ctx.lineCap = 'round';
        ctx.lineJoin = 'round';
        // Create a path from the SVG path string
        const path = new Path2D(symbol);
        ctx.stroke(path);
    }
    renderWire(ctx, wire) {
        // Set wire drawing style
        ctx.strokeStyle = this.options.theme === 'dark' ? '#a0a0a0' : '#000000';
        ctx.lineWidth = 1.5;
        ctx.lineCap = 'round';
        // For now, draw a simple line - in a more advanced implementation
        // we could use the path property to draw the routed wire
        const [x1, y1] = wire.from;
        const [x2, y2] = wire.to;
        ctx.beginPath();
        ctx.moveTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.stroke();
    }
    updateOptions(options) {
        this.options = { ...this.options, ...options };
    }
}
exports.CanvasRenderer = CanvasRenderer;
//# sourceMappingURL=canvas-renderer.js.map