"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SVGRenderer = void 0;
exports.renderSVG = renderSVG;
const renderer_1 = require("./renderer");
const layout_1 = require("./layout");
class SVGRenderer extends renderer_1.BaseRenderer {
    render(circuit, options) {
        const { width, height, theme = 'light', showLabels = true, showValues = true } = options;
        const colors = this.applyTheme(theme);
        // Apply layout to get final positions
        const layoutResult = (0, layout_1.layoutCircuit)(circuit.components, circuit.wires);
        let svg = `<svg width="${width}" height="${height}" xmlns="http://www.w3.org/2000/svg">\n`;
        // Add background
        svg += `<rect width="100%" height="100%" fill="${colors.background}" />\n`;
        // Render wires first (so components appear on top)
        for (const wire of layoutResult.wires) {
            svg += this.renderWire(wire, colors.wire);
        }
        // Render components
        for (const component of layoutResult.components) {
            svg += this.renderComponent(component, colors.component, showLabels, showValues);
        }
        svg += '</svg>';
        return svg;
    }
    renderWire(wire, color) {
        // For simplicity, we'll render a straight line between two points
        // In a real implementation, we would follow the path defined in the WireIR
        if (wire.path && wire.path.length > 0) {
            // Multi-segment wire following the path
            let path = `<polyline points="`;
            for (let i = 0; i < wire.path.length; i++) {
                const point = wire.path[i];
                path += `${point.x},${point.y} `;
            }
            path += `" stroke="${color}" stroke-width="2" fill="none" />\n`;
            return path;
        }
        else {
            // Simple straight line between from and to
            return `<line x1="${wire.from.x}" y1="${wire.from.y}" x2="${wire.to.x}" y2="${wire.to.y}" 
               stroke="${color}" stroke-width="2" />\n`;
        }
    }
    renderComponent(component, color, showLabels, showValues) {
        const [x, y] = component.position;
        let componentSVG = '';
        // Apply transform for rotation
        const transform = component.rotation !== 0
            ? `transform="rotate(${component.rotation}, ${x}, ${y})"`
            : '';
        // Render the component symbol
        componentSVG += `<g ${transform}>\n`;
        componentSVG += `<path d="${component.symbol}" stroke="${color}" stroke-width="2" fill="none" />\n`;
        // Add label
        if (showLabels) {
            componentSVG += `<text x="${x}" y="${y - 15}" font-size="12" fill="${color}" text-anchor="middle">${component.label}</text>\n`;
        }
        // Add value if available
        if (showValues && component.value) {
            componentSVG += `<text x="${x}" y="${y + 15}" font-size="10" fill="${color}" text-anchor="middle">${component.value}</text>\n`;
        }
        componentSVG += '</g>\n';
        return componentSVG;
    }
}
exports.SVGRenderer = SVGRenderer;
function renderSVG(circuit, options) {
    const renderer = new SVGRenderer();
    return renderer.render(circuit, options);
}
//# sourceMappingURL=svg-renderer.js.map