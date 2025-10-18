/**
 * Validation rules for Circuit Description Language
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
export declare class UniqueComponentIdRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidComponentTypeRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidNodeConnectionsRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class NoFloatingNodesRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class GroundNodeExistsRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidValueFormatRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class ComponentExistsRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
export declare class CircuitNameRule implements ValidationRule {
    validate(circuit: Circuit): ValidationError[];
}
//# sourceMappingURL=rules.d.ts.map