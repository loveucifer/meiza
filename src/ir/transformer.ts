/**
 * Transform AST to JSON IR (Intermediate Representation)
 */

import { Circuit, Component, Wire, Position } from '../parser/ast';

export interface BoundingBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface Pin {
  name: string;
  position: [number, number];
}

export interface ComponentIR {
  id: string;
  type: string;
  symbol: string; // SVG path or canvas instructions
  position: [number, number];
  rotation: number;
  pins: Pin[];
  label: string;
  value?: string;
}

export interface WireIR {
  from: [number, number];
  to: [number, number];
  path: [number, number][]; // Points for routing
}

export interface CircuitIR {
  name: string;
  components: ComponentIR[];
  wires: WireIR[];
  bounds: BoundingBox;
}

export class IRTransformer {
  transform(circuit: Circuit): CircuitIR {
    // Calculate bounding box
    const bounds: BoundingBox = {
      x: 0,
      y: 0,
      width: Math.max(800, circuit.components.length * 100), // Dynamic width based on components
      height: Math.max(600, circuit.components.length * 50)  // Dynamic height based on components
    };

    // Transform components
    const components: ComponentIR[] = circuit.components.map((component, index) => {
      // Calculate position if not provided
      const position: [number, number] = component.position 
        ? [component.position.x, component.position.y] 
        : [100 + (index % 5) * 150, 100 + Math.floor(index / 5) * 150];

      // Assign default rotation if not provided
      const rotation = component.rotation || 0;

      // Determine symbol based on component type
      const symbol = this.getSymbolForComponent(component.type);

      // Generate pins based on component type
      const pins = this.generatePins(component.type, position);

      // Format value if present
      const value = component.value 
        ? `${component.value.value}${component.value.unit}` 
        : undefined;

      return {
        id: component.id,
        type: component.type,
        symbol,
        position,
        rotation,
        pins,
        label: component.id,
        value
      };
    });

    // Transform wires
    const wires: WireIR[] = this.transformWires(circuit, components);

    return {
      name: circuit.name,
      components,
      wires,
      bounds
    };
  }

  private getSymbolForComponent(type: string): string {
    // Map component types to SVG path strings
    switch (type) {
      case 'resistor':
        return 'M 0,0 L 20,0 L 25,5 L 35,-5 L 45,5 L 55,-5 L 65,5 L 75,-5 L 80,0 L 100,0';
      case 'capacitor':
        return 'M 0,0 L 40,0 M 40,-15 L 40,15 M 60,-15 L 60,15 M 60,0 L 100,0';
      case 'voltage':
        return 'M 0,0 L 45,0 M 45,-20 A 25,25 0 1,0 45,20 A 25,25 0 1,0 45,-20 M 55,-20 A 25,25 0 1,1 55,20 A 25,25 0 1,1 55,-20 M 55,0 L 100,0';
      case 'ground':
        return 'M 0,0 L 0,10 M -10,10 L 10,10 M -5,15 L 5,15 M -2.5,20 L 2.5,20';
      default:
        return `M 0,0 L 100,0`; // Default line for unknown components
    }
  }

  private generatePins(type: string, position: [number, number]): Pin[] {
    // Generate pin positions based on component type
    switch (type) {
      case 'resistor':
      case 'capacitor':
      case 'inductor':
      case 'voltage':
      case 'current':
        // Two-terminal components
        return [
          { name: '1', position: [position[0], position[1]] },
          { name: '2', position: [position[0] + 100, position[1]] }
        ];
      case 'diode':
      case 'led':
        // Two-terminal components with direction
        return [
          { name: 'A', position: [position[0], position[1]] },
          { name: 'C', position: [position[0] + 100, position[1]] }
        ];
      case 'npn':
      case 'pnp':
        // Three-terminal transistor
        return [
          { name: 'B', position: [position[0] + 30, position[1] - 30] }, // Base
          { name: 'C', position: [position[0], position[1] + 30] },     // Collector
          { name: 'E', position: [position[0] + 60, position[1] + 30] } // Emitter
        ];
      case 'ground':
        // One-terminal component
        return [
          { name: 'GND', position: [position[0], position[1]] }
        ];
      default:
        // Default to two-terminal
        return [
          { name: '1', position: [position[0], position[1]] },
          { name: '2', position: [position[0] + 100, position[1]] }
        ];
    }
  }

  private transformWires(circuit: Circuit, components: ComponentIR[]): WireIR[] {
    // This method transforms the logical connections in the AST
    // to physical wire paths in the IR
    // Note: Actual wire routing is handled by the layout engine
    // The IR just connects components that share nodes
    const wires: WireIR[] = [];

    // This method is intentionally minimal as wire routing is 
    // handled by the layout engine after IR generation
    return wires;
  }
}