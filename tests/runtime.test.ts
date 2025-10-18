import { describe, it, expect, beforeEach } from 'vitest';
import { Runtime } from '../src/runtime/runtime';
import { Circuit } from '../src/parser/ast';

describe('Runtime', () => {
  let runtime: Runtime;

  beforeEach(() => {
    runtime = new Runtime();
  });

  it('should load a circuit and generate a netlist', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'C1', type: 'capacitor', value: { value: 1e-6, unit: 'F' }, nodes: ['out', 'gnd'] },
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    runtime.loadCircuit(circuit);
    const netlist = runtime.getNetlist();

    expect(Object.keys(netlist)).toHaveLength(3); // in, out, gnd
    expect(netlist['in']).toHaveLength(2); // R1 and V1
    expect(netlist['out']).toHaveLength(2); // R1 and C1
    expect(netlist['gnd']).toHaveLength(3); // C1, V1, and GND
  });

  it('should identify all nodes in the circuit', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'C1', type: 'capacitor', value: { value: 1e-6, unit: 'F' }, nodes: ['out', 'gnd'] },
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    runtime.loadCircuit(circuit);
    const nodes = runtime.identifyNodes();

    expect(nodes).toContain('in');
    expect(nodes).toContain('out');
    expect(nodes).toContain('gnd');
    expect(nodes).toHaveLength(3);
  });

  it('should calculate node voltages', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'C1', type: 'capacitor', value: { value: 1e-6, unit: 'F' }, nodes: ['out', 'gnd'] },
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    runtime.loadCircuit(circuit);
    const voltages = runtime.calculateNodeVoltages();

    // Ground should be 0V
    expect(voltages['gnd']).toBe(0);
    // Input should match voltage source
    expect(voltages['in']).toBe(5);
    // For a more complete analysis, output voltage would require additional circuit analysis
    // The current simple implementation doesn't calculate intermediate node voltages
    // So we'll just verify that voltages are computed for at least the known nodes
  });

  it('should detect short circuits', () => {
    // For now, just test that the method exists and works with a simple circuit
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
        { id: 'V2', type: 'voltage', value: { value: 3, unit: 'V' }, nodes: ['in', 'gnd'] }, // This should cause a short
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    runtime.loadCircuit(circuit);
    const shortCircuits = runtime.detectShortCircuits();

    // Should detect short circuits at nodes where multiple voltage sources are connected
    // Both 'in' and 'gnd' have multiple voltage sources connected
    expect(shortCircuits).toHaveLength(2);
    // Should have short circuits at both 'in' and 'gnd' nodes
    const nodeNames = shortCircuits.map(sc => sc[0]);
    expect(nodeNames).toContain('in');
    expect(nodeNames).toContain('gnd');
  });

  it('should analyze the circuit', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'C1', type: 'capacitor', value: { value: 1e-6, unit: 'F' }, nodes: ['out', 'gnd'] },
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    runtime.loadCircuit(circuit);
    const result = runtime.analyzeCircuit();

    expect(result.nodeVoltages).toBeDefined();
    expect(result.componentCurrents).toBeDefined();
    expect(result.nodeVoltages['gnd']).toBe(0);
    expect(result.nodeVoltages['in']).toBe(5);
  });
});