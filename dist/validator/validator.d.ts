/**
 * Validation system for Circuit Description Language
 */
import { Circuit } from '../parser/ast';
export interface ValidationError {
    message: string;
    line?: number;
    column?: number;
}
export interface ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class Validator {
    private rules;
    constructor();
    addRule(rule: ValidationRule): void;
    validate(circuit: Circuit): ValidationError[];
}
//# sourceMappingURL=validator.d.ts.map