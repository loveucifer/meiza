import { Circuit } from '../parser/ast';
import { ValidationError } from './errors';
export interface ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class UniqueComponentIdsRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidComponentTypeRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidNodeCountRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidValueFormatRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class NoFloatingNodesRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class GroundNodeExistsRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class NoDirectShortCircuitRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare class ValidComponentConnectionsRule implements ValidationRule {
    name: string;
    validate(circuit: Circuit): ValidationError[];
}
export declare const allValidationRules: ValidationRule[];
//# sourceMappingURL=rules.d.ts.map