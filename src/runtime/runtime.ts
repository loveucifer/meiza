/**
 * Runtime interpreter for Circuit Description Language
 */

import { Circuit, Component } from '../parser/ast';

export interface Netlist {
  [nodeName: string]: Component[];
}

export interface NodeVoltages {
  [nodeName: string]: number;
}

export interface SimulationResult {
  nodeVoltages: NodeVoltages;
  componentCurrents: { [componentId: string]: number };
}

export class Runtime {
  private circuit: Circuit | null = null;

  constructor() {}

  public loadCircuit(circuit: Circuit): void {
    this.circuit = circuit;
  }

  public getNetlist(): Netlist {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    const netlist: Netlist = {};

    // Initialize netlist with all nodes
    for (const component of this.circuit.components) {
      for (const node of component.nodes) {
        if (!netlist[node]) {
          netlist[node] = [];
        }
        netlist[node].push(component);
      }
    }

    return netlist;
  }

  public getConnectedComponents(node: string): Component[] {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    const connected: Component[] = [];
    for (const component of this.circuit.components) {
      if (component.nodes.includes(node)) {
        connected.push(component);
      }
    }

    return connected;
  }

  public identifyNodes(): string[] {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    const nodes = new Set<string>();
    for (const component of this.circuit.components) {
      for (const node of component.nodes) {
        nodes.add(node);
      }
    }

    return Array.from(nodes);
  }

  public calculateNodeVoltages(): NodeVoltages {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    const nodeVoltages: NodeVoltages = {};
    const netlist = this.getNetlist();

    // Find ground node (should be 0V)
    const groundComponents = this.circuit.components.filter(c => c.type === 'ground');
    if (groundComponents.length === 0) {
      throw new Error('No ground node found in circuit');
    }
    
    // Assign 0V to ground node
    for (const ground of groundComponents) {
      for (const node of ground.nodes) {
        nodeVoltages[node] = 0;
      }
    }

    // Simple DC analysis - find voltage sources and assign voltages
    const voltageSources = this.circuit.components.filter(c => c.type === 'voltage');
    for (const vs of voltageSources) {
      if (vs.value && vs.nodes.length === 2) {
        const [posNode, negNode] = vs.nodes;
        
        // If the negative node is grounded, assign voltage to positive
        if (nodeVoltages[negNode] === 0) {
          nodeVoltages[posNode] = vs.value.value;
        } else if (nodeVoltages[negNode] !== undefined) {
          nodeVoltages[posNode] = nodeVoltages[negNode] + vs.value.value;
        }
        // In a more complete implementation, we would solve the complete system of equations
      }
    }

    return nodeVoltages;
  }

  public detectShortCircuits(): string[][] {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    const shortCircuits: string[][] = [];
    const netlist = this.getNetlist();

    // For each node, check if there are conflicting voltage sources
    for (const [node, components] of Object.entries(netlist)) {
      const voltageSources = components.filter(c => c.type === 'voltage');
      
      if (voltageSources.length > 1) {
        // Multiple voltage sources connected to the same node - possible short circuit
        const connectedIds = voltageSources.map(c => c.id);
        shortCircuits.push([node, ...connectedIds]);
      }
    }

    return shortCircuits;
  }

  public analyzeCircuit(): SimulationResult {
    if (!this.circuit) {
      throw new Error('No circuit loaded');
    }

    // For now, just return basic analysis results
    const nodeVoltages = this.calculateNodeVoltages();
    const componentCurrents: { [componentId: string]: number } = {};

    // For a more complete implementation, we would perform circuit simulation
    for (const component of this.circuit.components) {
      // Default to 0 current for all components
      componentCurrents[component.id] = 0;
    }

    return {
      nodeVoltages,
      componentCurrents
    };
  }
}