/**
 * IR (Intermediate Representation) type definitions for Circuit Description Language
 */

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