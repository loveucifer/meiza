"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Parser = void 0;
exports.parseCircuit = parseCircuit;
const lexer_1 = require("./lexer");
class Parser {
    constructor(input) {
        this.position = 0;
        this.lexer = new lexer_1.Lexer(input);
        this.tokens = this.lexer.getAllTokens();
        this.currentToken = this.tokens[0];
    }
    advance() {
        this.position++;
        if (this.position < this.tokens.length) {
            this.currentToken = this.tokens[this.position];
        }
        else {
            this.currentToken = this.tokens[this.tokens.length - 1]; // Keep the EOF token
        }
    }
    match(tokenType) {
        return this.currentToken?.type === tokenType;
    }
    consume(tokenType) {
        if (!this.currentToken || this.currentToken.type !== tokenType) {
            throw new Error(`Expected token ${tokenType}, but got ${this.currentToken?.type} at line ${this.currentToken?.line}, column ${this.currentToken?.column}`);
        }
        const token = this.currentToken;
        this.advance();
        return token;
    }
    parseValue() {
        if (this.match(lexer_1.TokenTypes.NUMBER)) {
            const token = this.consume(lexer_1.TokenTypes.NUMBER);
            const valueStr = token.value;
            // Extract number and unit
            const match = valueStr.match(/^([\d.]+)\s*([a-zA-Z]*)$/);
            if (match) {
                const [, numStr, unit] = match;
                return {
                    value: parseFloat(numStr),
                    unit: unit || undefined
                };
            }
            else {
                // If no unit, just return the number
                return {
                    value: parseFloat(valueStr),
                };
            }
        }
        return undefined;
    }
    parseComponent() {
        const id = this.consume(lexer_1.TokenTypes.ID).value;
        // Accept any identifier as component type, even if it's not a valid one
        // Validation will catch invalid component types later
        let componentType;
        if (this.match(lexer_1.TokenTypes.KEYWORD)) {
            componentType = this.consume(lexer_1.TokenTypes.KEYWORD).value;
        }
        else if (this.match(lexer_1.TokenTypes.ID)) {
            componentType = this.consume(lexer_1.TokenTypes.ID).value;
        }
        else {
            throw new Error(`Expected component type but got ${this.currentToken?.type} at line ${this.currentToken?.line}, column ${this.currentToken?.column}`);
        }
        // Check if component has a value
        let value;
        if (this.match(lexer_1.TokenTypes.NUMBER)) {
            value = this.parseValue();
        }
        // Parse nodes (connections) - continue until we hit newline or EOF
        // For nodes, we need to accept both IDs and KEYWORDS (as node names may match keywords)
        const nodes = [];
        while (this.position < this.tokens.length &&
            this.currentToken &&
            this.currentToken.type !== lexer_1.TokenTypes.NEWLINE &&
            this.currentToken.type !== lexer_1.TokenTypes.EOF) {
            if (this.currentToken.type === lexer_1.TokenTypes.ID || this.currentToken.type === lexer_1.TokenTypes.KEYWORD) {
                // For nodes, treat both IDs and KEYWORDS as identifiers
                const token = this.currentToken;
                this.advance(); // Advance before consuming to work with our token tracking
                nodes.push(token.value);
            }
            else {
                // Skip other token types until we get to newline
                this.advance();
            }
        }
        // Consume newline if present
        if (this.match(lexer_1.TokenTypes.NEWLINE)) {
            this.advance();
        }
        return {
            id,
            type: componentType, // Type assertion since we'll validate later
            value,
            nodes
        };
    }
    parse() {
        // Expect 'circuit' keyword followed by circuit name
        this.consume(lexer_1.TokenTypes.KEYWORD); // 'circuit'
        const name = this.consume(lexer_1.TokenTypes.ID).value;
        // Consume newline after circuit name
        if (this.match(lexer_1.TokenTypes.NEWLINE)) {
            this.advance();
        }
        const components = [];
        const wires = []; // Wires are implicitly defined by connections between components
        // Parse components
        while (!this.match(lexer_1.TokenTypes.EOF)) {
            if (this.match(lexer_1.TokenTypes.ID)) {
                components.push(this.parseComponent());
            }
            else if (this.match(lexer_1.TokenTypes.NEWLINE)) {
                this.advance();
            }
            else if (this.match(lexer_1.TokenTypes.COMMENT)) {
                this.advance(); // Skip comments
            }
            else if (this.match(lexer_1.TokenTypes.KEYWORD)) {
                // Check if it's the 'circuit' keyword for sub-circuits or similar
                // For now, just advance to skip unexpected keywords
                this.advance();
            }
            else {
                // If we encounter an unexpected token, skip it
                this.advance();
            }
        }
        // Generate wires implicitly from component connections
        for (const component of components) {
            for (let i = 0; i < component.nodes.length - 1; i++) {
                wires.push({
                    from: component.nodes[i],
                    to: component.nodes[i + 1]
                });
            }
        }
        return {
            name,
            components,
            wires
        };
    }
}
exports.Parser = Parser;
function parseCircuit(input) {
    const parser = new Parser(input);
    return parser.parse();
}
//# sourceMappingURL=parser.js.map