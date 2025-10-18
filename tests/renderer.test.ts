import { describe, it, expect } from 'vitest';
import { SVGRenderer } from '../src/renderer/svg-renderer';
import { CircuitIR } from '../src/ir/types';

describe('Renderer', () => {
  it('should generate valid SVG for a circuit', () => {
    const circuitIR: CircuitIR = {
      name: 'test',
      components: [
        {
          id: 'R1',
          type: 'resistor',
          symbol: 'M 0,0 L 100,0',
          position: [100, 100],
          rotation: 0,
          pins: [
            { name: '1', position: [100, 100] },
            { name: '2', position: [200, 100] }
          ],
          label: 'R1',
          value: '1kΩ'
        }
      ],
      wires: [],
      bounds: { x: 0, y: 0, width: 800, height: 600 }
    };

    const renderer = new SVGRenderer();
    const svg = renderer.render(circuitIR);

    expect(svg).toContain('<svg');
    expect(svg).toContain('M 0,0 L 20,0 L 25,5 L 35,-5 L 45,5 L 55,-5 L 65,5 L 75,-5 L 80,0 L 100,0');
    expect(svg).toContain('R1');
    expect(svg).toContain('1kΩ');
  });

  it('should handle different themes', () => {
    const circuitIR: CircuitIR = {
      name: 'test',
      components: [
        {
          id: 'R1',
          type: 'resistor',
          symbol: 'M 0,0 L 100,0',
          position: [100, 100],
          rotation: 0,
          pins: [
            { name: '1', position: [100, 100] },
            { name: '2', position: [200, 100] }
          ],
          label: 'R1',
          value: '1kΩ'
        }
      ],
      wires: [],
      bounds: { x: 0, y: 0, width: 800, height: 600 }
    };

    const lightRenderer = new SVGRenderer({ theme: 'light' });
    const lightSvg = lightRenderer.render(circuitIR);

    const darkRenderer = new SVGRenderer({ theme: 'dark' });
    const darkSvg = darkRenderer.render(circuitIR);

    // Dark theme should have different color values than light theme
    expect(lightSvg).toContain('#ffffff'); // light background
    expect(darkSvg).toContain('#1e1e1e'); // dark background
  });

  it('should handle options correctly', () => {
    const circuitIR: CircuitIR = {
      name: 'test',
      components: [
        {
          id: 'R1',
          type: 'resistor',
          symbol: 'M 0,0 L 100,0',
          position: [100, 100],
          rotation: 0,
          pins: [
            { name: '1', position: [100, 100] },
            { name: '2', position: [200, 100] }
          ],
          label: 'R1',
          value: '1kΩ'
        }
      ],
      wires: [],
      bounds: { x: 0, y: 0, width: 800, height: 600 }
    };

    // Test with labels and values disabled
    const renderer = new SVGRenderer({ showLabels: false, showValues: false });
    const svg = renderer.render(circuitIR);

    expect(svg).not.toContain('R1'); // Label should not be present
    expect(svg).not.toContain('1kΩ'); // Value should not be present
  });
});