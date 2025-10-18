export interface ValidationError {
    type: string;
    message: string;
    severity: 'error' | 'warning';
}
export declare class ValidationError extends Error {
    type: string;
    severity: 'error' | 'warning';
    constructor(type: string, message: string, severity?: 'error' | 'warning');
}
//# sourceMappingURL=errors.d.ts.map