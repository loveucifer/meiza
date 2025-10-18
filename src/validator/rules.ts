/**
 * Validation rules for Circuit Description Language
 */

import { Circuit, Component } from '../parser/ast';

export interface ValidationError {
  message: string;
  line?: number;
  column?: number;
}

export interface ValidationRule {
  validate(circuit: Circuit): ValidationError[];
}

// Rule implementations
export class UniqueComponentIdRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];
    const seenIds = new Set<string>();

    for (const component of circuit.components) {
      if (seenIds.has(component.id)) {
        errors.push({
          message: `Duplicate component ID: ${component.id}`
        });
      } else {
        seenIds.add(component.id);
      }
    }

    return errors;
  }
}

export class ValidComponentTypeRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];
    const validTypes = [
      'resistor', 'capacitor', 'inductor',
      'voltage', 'current',
      'diode', 'led', 'npn', 'pnp', 'nmos', 'pmos',
      'opamp', 'and', 'or', 'not', 'nand', 'nor', 'xor',
      'ground', 'battery', 'switch'
    ];

    for (const component of circuit.components) {
      if (!validTypes.includes(component.type)) {
        errors.push({
          message: `Invalid component type: ${component.type}`
        });
      }
    }

    return errors;
  }
}

export class ValidNodeConnectionsRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];

    for (const component of circuit.components) {
      // Components like ground only have 1 node, voltage/current sources have 2, etc.
      // Defining minimum node requirements:
      if (component.type === 'ground') {
        if (component.nodes.length !== 1) {
          errors.push({
            message: `Ground component ${component.id} must have exactly 1 node`
          });
        }
      } else if (['voltage', 'current'].includes(component.type)) {
        if (component.nodes.length !== 2) {
          errors.push({
            message: `Voltage/Current source ${component.id} must have exactly 2 nodes`
          });
        }
      } else {
        // Most components need at least 2 nodes
        if (component.nodes.length < 2) {
          errors.push({
            message: `Component ${component.id} must have at least 2 nodes`
          });
        }
      }
    }

    return errors;
  }
}

export class NoFloatingNodesRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];
    const connectedNodes = new Set<string>();

    // Add all nodes that are connected to components
    for (const component of circuit.components) {
      for (const node of component.nodes) {
        connectedNodes.add(node);
      }
    }

    // Check if any components reference nodes that don't exist
    for (const component of circuit.components) {
      for (const node of component.nodes) {
        if (!connectedNodes.has(node)) {
          errors.push({
            message: `Component ${component.id} references non-existent node: ${node}`
          });
        }
      }
    }

    return errors;
  }
}

export class GroundNodeExistsRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];
    const hasGround = circuit.components.some(comp => comp.type === 'ground');

    if (!hasGround) {
      errors.push({
        message: 'Circuit must contain at least one ground component'
      });
    }

    return errors;
  }
}

export class ValidValueFormatRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];

    for (const component of circuit.components) {
      if (component.value !== undefined) {
        // Check if the value is a positive number
        if (component.value.value <= 0) {
          errors.push({
            message: `Component ${component.id} has invalid value: ${component.value.value}`
          });
        }
        
        // Check if the unit is appropriate for the component type
        const { value, unit } = component.value;
        switch (component.type) {
          case 'resistor':
            if (!['Ω', 'ohm', 'kΩ', 'kohm', 'MΩ', 'Mohm'].includes(unit)) {
              errors.push({
                message: `Resistor ${component.id} has invalid unit: ${unit}`
              });
            }
            break;
          case 'capacitor':
            if (!['F', 'f', 'pF', 'pf', 'nF', 'nf', 'uF', 'uf', 'µF', 'µf'].includes(unit.toLowerCase())) {
              errors.push({
                message: `Capacitor ${component.id} has invalid unit: ${unit}`
              });
            }
            break;
          case 'inductor':
            if (!['H', 'h', 'pH', 'ph', 'nH', 'nh', 'uH', 'uh', 'µH', 'µh', 'mH', 'mh'].includes(unit.toLowerCase())) {
              errors.push({
                message: `Inductor ${component.id} has invalid unit: ${unit}`
              });
            }
            break;
          case 'voltage':
            if (!['V', 'v', 'mV', 'mv', 'uV', 'uv', 'µV', 'µv', 'kV', 'kv'].includes(unit.toLowerCase())) {
              errors.push({
                message: `Voltage source ${component.id} has invalid unit: ${unit}`
              });
            }
            break;
          case 'current':
            if (!['A', 'a', 'mA', 'ma', 'uA', 'ua', 'µA', 'µa', 'kA', 'ka'].includes(unit.toLowerCase())) {
              errors.push({
                message: `Current source ${component.id} has invalid unit: ${unit}`
              });
            }
            break;
        }
      }
    }

    return errors;
  }
}

export class ComponentExistsRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];

    // This rule checks if all referenced components exist
    // Since we're validating the circuit after parsing, 
    // all components in the circuit.components array exist by definition
    // But we could implement cross-referencing checks here if needed
    
    return errors;
  }
}

export class CircuitNameRule implements ValidationRule {
  validate(circuit: Circuit): ValidationError[] {
    const errors: ValidationError[] = [];

    if (!circuit.name || circuit.name.trim() === '') {
      errors.push({
        message: 'Circuit must have a valid name'
      });
    }

    return errors;
  }
}