"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.validateCircuit = validateCircuit;
const errors_1 = require("./errors");
const rules_1 = require("./rules");
function validateCircuit(circuit, rules = rules_1.allValidationRules) {
    const errors = [];
    for (const rule of rules) {
        try {
            const ruleErrors = rule.validate(circuit);
            errors.push(...ruleErrors);
        }
        catch (e) {
            // If a rule fails, add it as an error
            errors.push(new errors_1.ValidationError('VALIDATION_RULE_ERROR', `Validation rule ${rule.name} failed: ${e.message}`, 'error'));
        }
    }
    return errors;
}
//# sourceMappingURL=validator.js.map