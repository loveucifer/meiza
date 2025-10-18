/**
 * Validation error types for the Circuit Description Language
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
//# sourceMappingURL=errors.d.ts.map