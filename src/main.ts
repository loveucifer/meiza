/**
 * Main entry point for Mieza - Circuit Description Language System
 */

import { Parser } from './parser/parser';
import { Validator } from './validator/validator';
import { Runtime } from './runtime/runtime';
import { IRTransformer } from './ir/transformer';
import { LayoutEngine } from './renderer/layout';
import { SVGRenderer } from './renderer/svg-renderer';
import { CanvasRenderer } from './renderer/canvas-renderer';
import { Circuit } from './parser/ast';
import { CircuitIR } from './ir/types';

export class Mieza {
  private parser: Parser;
  private validator: Validator;
  private runtime: Runtime;
  private irTransformer: IRTransformer;
  private layoutEngine: LayoutEngine;

  constructor() {
    this.parser = new Parser('');
    this.validator = new Validator();
    this.runtime = new Runtime();
    this.irTransformer = new IRTransformer();
    this.layoutEngine = new LayoutEngine();
  }

  public parse(cdl: string): Circuit {
    this.parser = new Parser(cdl);
    return this.parser.parse();
  }

  public validate(circuit: Circuit): import('./validator/validator').ValidationError[] {
    return this.validator.validate(circuit);
  }

  public toIR(circuit: Circuit): CircuitIR {
    let ir = this.irTransformer.transform(circuit);
    // Apply layout to the IR
    ir = this.layoutEngine.layout(ir);
    return ir;
  }

  public renderSVG(circuitIR: CircuitIR, options?: import('./renderer/svg-renderer').SVGRendererOptions): string {
    const renderer = new SVGRenderer(options);
    return renderer.render(circuitIR);
  }

  public renderCanvas(circuitIR: CircuitIR, canvas: HTMLCanvasElement, options?: import('./renderer/canvas-renderer').CanvasRendererOptions): void {
    const renderer = new CanvasRenderer(options);
    renderer.render(circuitIR, canvas);
  }

  public analyze(circuit: Circuit): import('./runtime/runtime').SimulationResult {
    this.runtime.loadCircuit(circuit);
    return this.runtime.analyzeCircuit();
  }
}