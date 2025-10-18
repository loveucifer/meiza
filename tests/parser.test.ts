import { describe, it, expect } from 'vitest';
import { Parser } from '../src/parser/parser';
import { Lexer } from '../src/parser/lexer';

describe('Parser', () => {
  it('should correctly parse a simple circuit', () => {
    const cdl = `
      circuit test
      R1 resistor 1k in out
      C1 capacitor 100nF out gnd
      V1 voltage 5V in gnd
      GND ground gnd
    `;

    const parser = new Parser(cdl);
    const circuit = parser.parse();

    expect(circuit.name).toBe('test');
    expect(circuit.components).toHaveLength(4);

    const resistor = circuit.components.find(c => c.id === 'R1');
    expect(resistor).toBeDefined();
    expect(resistor?.type).toBe('resistor');
    expect(resistor?.value?.value).toBe(1);
    expect(resistor?.value?.unit).toBe('kΩ');
    expect(resistor?.nodes).toEqual(['in', 'out']);

    const capacitor = circuit.components.find(c => c.id === 'C1');
    expect(capacitor).toBeDefined();
    expect(capacitor?.type).toBe('capacitor');
    expect(capacitor?.value?.value).toBe(100);
    expect(capacitor?.value?.unit).toBe('nF');
    expect(capacitor?.nodes).toEqual(['out', 'gnd']);

    const voltage = circuit.components.find(c => c.id === 'V1');
    expect(voltage).toBeDefined();
    expect(voltage?.type).toBe('voltage');
    expect(voltage?.value?.value).toBe(5);
    expect(voltage?.value?.unit).toBe('V');
    expect(voltage?.nodes).toEqual(['in', 'gnd']);

    const ground = circuit.components.find(c => c.id === 'GND');
    expect(ground).toBeDefined();
    expect(ground?.type).toBe('ground');
    expect(ground?.nodes).toEqual(['gnd']);
  });

  it('should handle comments correctly', () => {
    const cdl = `
      circuit test # This is a comment
      R1 resistor 1k in out # Another comment
      # This is a full line comment
      C1 capacitor 100nF out gnd
    `;

    const parser = new Parser(cdl);
    const circuit = parser.parse();

    expect(circuit.name).toBe('test');
    expect(circuit.components).toHaveLength(2);
    expect(circuit.comments).toHaveLength(3); // Each comment line is captured
  });

  it('should throw an error for invalid syntax', () => {
    const cdl = `
      circuit test
      R1 invalid_type 1k in out
    `;

    const parser = new Parser(cdl);
    expect(() => parser.parse()).toThrow();
  });
});