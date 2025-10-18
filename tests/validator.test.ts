import { describe, it, expect } from 'vitest';
import { Validator } from '../src/validator/validator';
import { Circuit } from '../src/parser/ast';

describe('Validator', () => {
  it('should detect duplicate component IDs', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'R1', type: 'resistor', value: { value: 2000, unit: 'Ω' }, nodes: ['out', 'gnd'] }, // Duplicate ID
        { id: 'GND', type: 'ground', nodes: ['gnd'] }, // Include ground to avoid that error
      ],
      wires: [],
      comments: []
    };

    const validator = new Validator();
    const errors = validator.validate(circuit);

    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain('Duplicate component ID');
  });

  it('should detect invalid component types', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'invalid_type' as any, value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'GND', type: 'ground', nodes: ['out'] }, // Include ground to avoid that error
      ],
      wires: [],
      comments: []
    };

    const validator = new Validator();
    const errors = validator.validate(circuit);

    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain('Invalid component type');
  });

  it('should detect circuits without ground', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: 1000, unit: 'Ω' }, nodes: ['in', 'out'] },
        { id: 'V1', type: 'voltage', value: { value: 5, unit: 'V' }, nodes: ['in', 'gnd'] },
      ],
      wires: [],
      comments: []
    };

    const validator = new Validator();
    const errors = validator.validate(circuit);

    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain('must contain at least one ground component');
  });

  it('should pass validation for a valid circuit', () => {
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

    const validator = new Validator();
    const errors = validator.validate(circuit);

    expect(errors).toHaveLength(0);
  });

  it('should detect invalid values', () => {
    const circuit: Circuit = {
      name: 'test',
      components: [
        { id: 'R1', type: 'resistor', value: { value: -1000, unit: 'Ω' }, nodes: ['in', 'out'] }, // Negative value
        { id: 'GND', type: 'ground', nodes: ['gnd'] },
      ],
      wires: [],
      comments: []
    };

    const validator = new Validator();
    const errors = validator.validate(circuit);

    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain('has invalid value');
  });
});