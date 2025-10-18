"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CanvasRenderer = void 0;
exports.renderCanvas = renderCanvas;
const renderer_1 = require("./renderer");
const layout_1 = require("./layout");
class CanvasRenderer extends renderer_1.BaseRenderer {
    render(circuit, options) {
        // This method would render directly to a canvas, but since we can't access
        // the DOM in this context, we'll provide the implementation
        throw new Error('Canvas rendering requires DOM access. Use renderCanvasToContext instead.');
    }
    renderToContext(circuit, context, options) {
        const { width, height, theme = 'light', showLabels = true, showValues = true } = options;
        const colors = this.applyTheme(theme);
        // Apply layout to get final positions
        const layoutResult = (0, layout_1.layoutCircuit)(circuit.components, circuit.wires);
        // Clear canvas
        context.fillStyle = colors.background;
        context.fillRect(0, 0, width, height);
        // Render wires first (so components appear on top)
        for (const wire of layoutResult.wires) {
            this.renderWireToContext(wire, context, colors.wire);
        }
        // Render components
        for (const component of layoutResult.components) {
            this.renderComponentToContext(component, context, colors.component, showLabels, showValues);
        }
    }
    renderWireToContext(wire, context, color) {
        context.strokeStyle = color;
        context.lineWidth = 2;
        context.beginPath();
        if (wire.path && wire.path.length > 0) {
            // Multi-segment wire following the path
            const firstPoint = wire.path[0];
            context.moveTo(firstPoint.x, firstPoint.y);
            for (let i = 1; i < wire.path.length; i++) {
                const point = wire.path[i];
                context.lineTo(point.x, point.y);
            }
        }
        else {
            // Simple straight line between from and to
            context.moveTo(wire.from.x, wire.from.y);
            context.lineTo(wire.to.x, wire.to.y);
        }
        context.stroke();
    }
    renderComponentToContext(component, context, color, showLabels, showValues) {
        const [x, y] = component.position;
        // Save the current context state
        context.save();
        // Apply rotation if needed
        if (component.rotation !== 0) {
            context.translate(x, y);
            context.rotate(component.rotation * Math.PI / 180); // Convert degrees to radians
            context.translate(-x, -y);
        }
        // Draw the component symbol using the path data
        context.strokeStyle = color;
        context.lineWidth = 2;
        context.beginPath();
        // Parse and draw the SVG path data using Canvas API
        this.drawPathData(context, component.symbol);
        context.stroke();
        // Draw label if enabled
        if (showLabels) {
            context.fillStyle = color;
            context.font = '12px Arial';
            context.textAlign = 'center';
            context.textBaseline = 'bottom';
            context.fillText(component.label, x, y - 15);
        }
        // Draw value if enabled
        if (showValues && component.value) {
            context.fillStyle = color;
            context.font = '10px Arial';
            context.textAlign = 'center';
            context.textBaseline = 'top';
            context.fillText(component.value, x, y + 15);
        }
        // Restore the context state
        context.restore();
    }
    drawPathData(context, pathData) {
        // This is a simplified path parser for SVG path data
        // A full implementation would be more complex
        const commands = pathData.trim().split(/(?=[A-Za-z])/);
        let currentX = 0;
        let currentY = 0;
        for (const command of commands) {
            if (!command)
                continue;
            const type = command.charAt(0);
            const paramsStr = command.substring(1).trim();
            const params = paramsStr ? paramsStr.split(/[\s,]+/).map(p => parseFloat(p)) : [];
            switch (type) {
                case 'M': // Move to
                    if (params.length >= 2) {
                        currentX = params[0];
                        currentY = params[1];
                        context.moveTo(currentX, currentY);
                    }
                    break;
                case 'L': // Line to
                    if (params.length >= 2) {
                        currentX = params[0];
                        currentY = params[1];
                        context.lineTo(currentX, currentY);
                    }
                    break;
                case 'H': // Horizontal line to
                    if (params.length >= 1) {
                        currentX = params[0];
                        context.lineTo(currentX, currentY);
                    }
                    break;
                case 'V': // Vertical line to
                    if (params.length >= 1) {
                        currentY = params[0];
                        context.lineTo(currentX, currentY);
                    }
                    break;
                case 'Z': // Close path
                    context.closePath();
                    break;
                case 'C': // Cubic Bézier curve
                    if (params.length >= 6) {
                        const x1 = params[0], y1 = params[1];
                        const x2 = params[2], y2 = params[3];
                        const x = params[4], y = params[5];
                        context.bezierCurveTo(x1, y1, x2, y2, x, y);
                        currentX = x;
                        currentY = y;
                    }
                    break;
                case 'S': // Smooth cubic Bézier curve
                    if (params.length >= 4) {
                        // Simplified: treat as cubic Bézier with inferred control point
                        const x2 = params[0], y2 = params[1];
                        const x = params[2], y = params[3];
                        // For simplicity, we'll approximate with a line
                        context.lineTo(x, y);
                        currentX = x;
                        currentY = y;
                    }
                    break;
                case 'Q': // Quadratic Bézier curve
                    if (params.length >= 4) {
                        const x1 = params[0], y1 = params[1];
                        const x = params[2], y = params[3];
                        context.quadraticCurveTo(x1, y1, x, y);
                        currentX = x;
                        currentY = y;
                    }
                    break;
                case 'T': // Smooth quadratic Bézier curve
                    if (params.length >= 2) {
                        const x = params[0], y = params[1];
                        // For simplicity, we'll approximate with a line
                        context.lineTo(x, y);
                        currentX = x;
                        currentY = y;
                    }
                    break;
                case 'A': // Elliptical arc
                    if (params.length >= 7) {
                        // For simplicity, we'll approximate with a line
                        const x = params[5], y = params[6];
                        context.lineTo(x, y);
                        currentX = x;
                        currentY = y;
                    }
                    break;
            }
        }
    }
}
exports.CanvasRenderer = CanvasRenderer;
function renderCanvas(circuit, canvas, options = { width: 800, height: 600 }) {
    const renderer = new CanvasRenderer();
    const context = canvas.getContext('2d');
    if (!context) {
        throw new Error('Could not get 2D context from canvas element');
    }
    // Set canvas dimensions
    canvas.width = options.width;
    canvas.height = options.height;
    renderer.renderToContext(circuit, context, options);
}
//# sourceMappingURL=canvas-renderer.js.map