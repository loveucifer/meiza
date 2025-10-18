/**
 * SVG renderer for Circuit Description Language
 */

import { CircuitIR, ComponentIR, WireIR } from '../ir/types';
import { getComponentSymbol } from './components';

export interface SVGRendererOptions {
  width?: number;
  height?: number;
  theme?: 'light' | 'dark';
  showLabels?: boolean;
  showValues?: boolean;
}

export class SVGRenderer {
  private options: SVGRendererOptions;

  constructor(options?: SVGRendererOptions) {
    this.options = {
      width: 800,
      height: 600,
      theme: 'light',
      showLabels: true,
      showValues: true,
      ...options
    };
  }

  public render(circuitIR: CircuitIR): string {
    const { width, height } = this.options;
    
    // Start SVG element
    let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">`;
    
    // Add theme-based styles
    svg += this.getStyles();
    
    // Render wires first (so components appear on top)
    for (const wire of circuitIR.wires) {
      svg += this.renderWire(wire);
    }
    
    // Render components
    for (const component of circuitIR.components) {
      svg += this.renderComponent(component);
    }
    
    // Close SVG element
    svg += '</svg>';
    
    return svg;
  }

  private getStyles(): string {
    const isDark = this.options.theme === 'dark';
    const bgColor = isDark ? '#1e1e1e' : '#ffffff';
    const textColor = isDark ? '#ffffff' : '#000000';
    const wireColor = isDark ? '#a0a0a0' : '#000000';
    const componentColor = isDark ? '#8080ff' : '#000000';
    
    return `
      <style>
        .background { fill: ${bgColor}; }
        .component { stroke: ${componentColor}; stroke-width: 2; fill: none; }
        .wire { stroke: ${wireColor}; stroke-width: 1.5; fill: none; }
        .label { fill: ${textColor}; font-family: Arial, sans-serif; font-size: 12px; }
        .value { fill: ${textColor}; font-family: Arial, sans-serif; font-size: 10px; }
      </style>
    `;
  }

  private renderComponent(component: ComponentIR): string {
    const symbol = getComponentSymbol(component.type as any);
    if (!symbol) {
      console.warn(`Unknown component type: ${component.type}`);
      return '';
    }

    const [x, y] = component.position;
    const transform = `translate(${x}, ${y}) rotate(${component.rotation}, 50, 0)`;
    
    let componentSVG = `<g transform="${transform}">`;
    
    // Render the component symbol
    componentSVG += `<path class="component" d="${symbol.symbol}" />`;
    
    // Render label if enabled
    if (this.options.showLabels) {
      componentSVG += `<text class="label" x="50" y="-10" text-anchor="middle">${component.label}</text>`;
    }
    
    // Render value if present and enabled
    if (component.value && this.options.showValues) {
      componentSVG += `<text class="value" x="50" y="15" text-anchor="middle">${component.value}</text>`;
    }
    
    componentSVG += '</g>';
    
    return componentSVG;
  }

  private renderWire(wire: WireIR): string {
    // For now, draw a simple line - in a more advanced implementation
    // we could use the path property to draw the routed wire
    const [x1, y1] = wire.from;
    const [x2, y2] = wire.to;
    
    return `<line class="wire" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" />`;
  }
  
  public updateOptions(options: SVGRendererOptions): void {
    this.options = { ...this.options, ...options };
  }
}