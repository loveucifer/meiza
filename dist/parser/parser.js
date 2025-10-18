"use strict";
/**
 * Recursive descent parser for the Circuit Description Language
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.Parser = void 0;
const lexer_1 = require("./lexer");
class Parser {
    constructor(input) {
        this.position = 0;
        const lexer = new lexer_1.Lexer(input);
        this.tokens = lexer.tokenize();
    }
    parse() {
        const circuit = {
            name: '',
            components: [],
            wires: [],
            comments: []
        };
        // Skip any leading newlines, whitespace, or comments
        while (this.currentToken().type === lexer_1.TokenType.NEWLINE || this.currentToken().type === lexer_1.TokenType.COMMENT) {
            this.advance();
        }
        // Expect the circuit declaration
        this.consume(lexer_1.TokenType.CIRCUIT, 'Expected "circuit" keyword');
        // Get circuit name
        const circuitNameToken = this.consume(lexer_1.TokenType.ID, 'Expected circuit name');
        circuit.name = circuitNameToken.value;
        // Parse components and wires until EOF
        while (this.currentToken().type !== lexer_1.TokenType.EOF) {
            // Skip newlines
            if (this.currentToken().type === lexer_1.TokenType.NEWLINE) {
                this.advance();
                continue;
            }
            // Handle comments
            if (this.currentToken().type === lexer_1.TokenType.COMMENT) {
                const commentToken = this.currentToken();
                circuit.comments.push({
                    position: { x: commentToken.column, y: commentToken.line },
                    text: commentToken.value.substring(1) // Remove the '#' character
                });
                this.advance();
                continue;
            }
            // Handle components or wires
            if (this.isComponentDeclaration()) {
                const component = this.parseComponent();
                circuit.components.push(component);
            }
            else {
                // If it's not a component, it might be a wire or an error
                // For now, let's check for possible wire format: node1 node2
                // But since our grammar doesn't explicitly define wire syntax like that,
                // we'll treat everything as component declarations for now
                const component = this.parseComponent();
                circuit.components.push(component);
            }
        }
        return circuit;
    }
    parseComponent() {
        // Parse the component ID
        const idToken = this.consume(lexer_1.TokenType.ID, 'Expected component ID');
        const id = idToken.value;
        // Parse the component type
        const typeToken = this.advance();
        let type;
        switch (typeToken.type) {
            case lexer_1.TokenType.RESISTOR:
                type = 'resistor';
                break;
            case lexer_1.TokenType.CAPACITOR:
                type = 'capacitor';
                break;
            case lexer_1.TokenType.INDUCTOR:
                type = 'inductor';
                break;
            case lexer_1.TokenType.VOLTAGE:
                type = 'voltage';
                break;
            case lexer_1.TokenType.CURRENT:
                type = 'current';
                break;
            case lexer_1.TokenType.DIODE:
                type = 'diode';
                break;
            case lexer_1.TokenType.LED:
                type = 'led';
                break;
            case lexer_1.TokenType.NPN:
                type = 'npn';
                break;
            case lexer_1.TokenType.PNP:
                type = 'pnp';
                break;
            case lexer_1.TokenType.NMOS:
                type = 'nmos';
                break;
            case lexer_1.TokenType.PMOS:
                type = 'pmos';
                break;
            case lexer_1.TokenType.OPAMP:
                type = 'opamp';
                break;
            case lexer_1.TokenType.AND:
                type = 'and';
                break;
            case lexer_1.TokenType.OR:
                type = 'or';
                break;
            case lexer_1.TokenType.NOT:
                type = 'not';
                break;
            case lexer_1.TokenType.NAND:
                type = 'nand';
                break;
            case lexer_1.TokenType.NOR:
                type = 'nor';
                break;
            case lexer_1.TokenType.XOR:
                type = 'xor';
                break;
            case lexer_1.TokenType.GROUND:
                type = 'ground';
                break;
            case lexer_1.TokenType.BATTERY:
                type = 'battery';
                break;
            case lexer_1.TokenType.SWITCH:
                type = 'switch';
                break;
            default:
                throw new Error(`Unexpected component type: ${typeToken.value} at line ${typeToken.line}, column ${typeToken.column}`);
        }
        // Optionally parse value with unit
        let value;
        if (this.isValueToken(this.currentToken())) {
            const valueToken = this.consume(lexer_1.TokenType.NUMBER, 'Expected numeric value');
            let unitValue;
            // Check if the number token already contains unit information (like "100n" for 100nF)
            const valueStr = valueToken.value;
            const unitMatch = valueStr.match(/([0-9.]+)([a-zA-Z]+)/);
            if (unitMatch) {
                // The number token contains both the value and unit prefix
                const numericValue = parseFloat(unitMatch[1]);
                const unitPrefix = unitMatch[2];
                // Look ahead to see if there's a following unit token (like "F" after "100n")
                if (this.isUnitToken(this.currentToken())) {
                    const unitToken = this.advance();
                    // Combine the prefix and base unit (e.g., "n" + "F" = "nF")
                    unitValue = unitPrefix + unitToken.value;
                }
                else {
                    // Use just the prefix (e.g., for "1k" it would be just "k" until we handle it properly)
                    unitValue = unitPrefix;
                }
                // Handle common unit abbreviations for different component types
                if (type === 'resistor' && unitValue === 'k') {
                    unitValue = 'kΩ';
                }
                else if (type === 'resistor' && unitValue === 'M') {
                    unitValue = 'MΩ';
                }
                else if (type === 'capacitor' && unitValue === 'u') {
                    unitValue = 'µF'; // microfarad
                }
                else if (type === 'capacitor' && unitValue === 'n') {
                    unitValue = 'nF'; // nanofarad
                }
                else if (type === 'capacitor' && unitValue === 'p') {
                    unitValue = 'pF'; // picofarad
                }
                else if (type === 'inductor' && unitValue === 'u') {
                    unitValue = 'µH'; // microhenry
                }
                else if (type === 'inductor' && unitValue === 'm') {
                    unitValue = 'mH'; // millihenry
                }
                else if (type === 'voltage' && unitValue === 'm') {
                    unitValue = 'mV'; // millivolt
                }
                else if (type === 'voltage' && unitValue === 'u') {
                    unitValue = 'µV'; // microvolt
                }
                else if (type === 'current' && unitValue === 'm') {
                    unitValue = 'mA'; // milliamp
                }
                else if (type === 'current' && unitValue === 'u') {
                    unitValue = 'µA'; // microamp
                }
                value = {
                    value: numericValue,
                    unit: unitValue
                };
            }
            else {
                // Standard case: numeric value followed by separate unit token or default unit
                const numericValue = parseFloat(valueToken.value);
                // Check if the next token is a unit token (like UNIT("k"))
                if (this.isUnitToken(this.currentToken())) {
                    const unitToken = this.advance();
                    unitValue = unitToken.value;
                }
                // Also check if the next token is an ID that could be a unit or unit prefix
                else if (this.currentToken().type === lexer_1.TokenType.ID) {
                    const nextTokenValue = this.currentToken().value;
                    // Check if this ID is a full compound unit (like "nF", "pF", "mV", etc.)
                    const compoundUnits = {
                        'resistor': ['Ω', 'ohm', 'kΩ', 'kohm', 'MΩ', 'Mohm'],
                        'capacitor': ['F', 'f', 'pF', 'pf', 'nF', 'nf', 'uF', 'uf', 'µF', 'µf'],
                        'inductor': ['H', 'h', 'pH', 'ph', 'nH', 'nh', 'uH', 'uh', 'µH', 'µh', 'mH', 'mh'],
                        'voltage': ['V', 'v', 'mV', 'mv', 'uV', 'uv', 'µV', 'µv', 'kV', 'kv'],
                        'current': ['A', 'a', 'mA', 'ma', 'uA', 'ua', 'µA', 'µa', 'kA', 'ka']
                    };
                    const validUnits = compoundUnits[type] || [];
                    if (validUnits.map(u => u.toLowerCase()).includes(nextTokenValue.toLowerCase())) {
                        // This is a valid compound unit, use it
                        unitValue = this.advance().value;
                    }
                    // Check if this ID is a common unit prefix (like "k", "m", "u", "n", "p", "f")
                    else {
                        const potentialUnit = nextTokenValue.toLowerCase();
                        if (potentialUnit === 'k' || potentialUnit === 'm' || potentialUnit === 'u' ||
                            potentialUnit === 'n' || potentialUnit === 'p' || potentialUnit === 'f' || potentialUnit === 'mega') {
                            // This is likely a unit prefix, consume it
                            unitValue = this.advance().value;
                        }
                        else {
                            // This doesn't look like a unit, use default
                            unitValue = this.getDefaultUnit(type);
                        }
                    }
                }
                else {
                    // Default unit could be assumed based on component type
                    unitValue = this.getDefaultUnit(type);
                }
                // Handle common unit abbreviations for different component types
                if (type === 'resistor' && unitValue === 'k') {
                    unitValue = 'kΩ';
                }
                else if (type === 'resistor' && unitValue === 'M' || unitValue === 'mega') {
                    unitValue = 'MΩ';
                }
                else if (type === 'capacitor' && unitValue === 'u') {
                    unitValue = 'µF'; // microfarad
                }
                else if (type === 'capacitor' && unitValue === 'n') {
                    unitValue = 'nF'; // nanofarad
                }
                else if (type === 'capacitor' && unitValue === 'p') {
                    unitValue = 'pF'; // picofarad
                }
                else if (type === 'inductor' && unitValue === 'u') {
                    unitValue = 'µH'; // microhenry
                }
                else if (type === 'inductor' && unitValue === 'm') {
                    unitValue = 'mH'; // millihenry
                }
                else if (type === 'voltage' && unitValue === 'm') {
                    unitValue = 'mV'; // millivolt
                }
                else if (type === 'voltage' && unitValue === 'u') {
                    unitValue = 'µV'; // microvolt
                }
                else if (type === 'current' && unitValue === 'm') {
                    unitValue = 'mA'; // milliamp
                }
                else if (type === 'current' && unitValue === 'u') {
                    unitValue = 'µA'; // microamp
                }
                else if (type === 'capacitor' && (unitValue === 'F' || unitValue === 'f')) {
                    // For cases like "100 F" where space separates value and unit
                    unitValue = 'F';
                }
                else if (type === 'inductor' && (unitValue === 'H' || unitValue === 'h')) {
                    unitValue = 'H';
                }
                else if (type === 'voltage' && (unitValue === 'V' || unitValue === 'v')) {
                    unitValue = 'V';
                }
                else if (type === 'current' && (unitValue === 'A' || unitValue === 'a')) {
                    unitValue = 'A';
                }
                value = {
                    value: numericValue,
                    unit: unitValue
                };
            }
        }
        // Parse node connections
        const nodes = [];
        while (this.currentToken().type !== lexer_1.TokenType.NEWLINE && this.currentToken().type !== lexer_1.TokenType.EOF && this.currentToken().type !== lexer_1.TokenType.COMMENT) {
            if (this.currentToken().type === lexer_1.TokenType.ID) {
                nodes.push(this.consume(lexer_1.TokenType.ID, 'Expected node name').value);
            }
            else {
                // If it's not an ID, advance to newline
                this.advance();
            }
        }
        // Consume the newline (but check if there's a comment first)
        if (this.currentToken().type === lexer_1.TokenType.NEWLINE) {
            this.advance();
        }
        // If there was a comment, return without consuming it so the main loop can handle it
        else if (this.currentToken().type === lexer_1.TokenType.COMMENT) {
            // Return the component but leave the comment token for the main loop
        }
        return {
            id,
            type,
            value,
            nodes,
            // Position and rotation would be parsed if they follow a specific syntax
            // For now, we're not including that in the basic grammar
        };
    }
    isComponentDeclaration() {
        // A component declaration starts with an ID followed by a component type keyword
        if (this.position >= this.tokens.length - 1) {
            return false;
        }
        const idToken = this.tokens[this.position];
        const typeToken = this.tokens[this.position + 1];
        return idToken.type === lexer_1.TokenType.ID && this.isComponentTypeToken(typeToken);
    }
    isComponentTypeToken(token) {
        return [
            lexer_1.TokenType.RESISTOR, lexer_1.TokenType.CAPACITOR, lexer_1.TokenType.INDUCTOR,
            lexer_1.TokenType.VOLTAGE, lexer_1.TokenType.CURRENT,
            lexer_1.TokenType.DIODE, lexer_1.TokenType.LED,
            lexer_1.TokenType.NPN, lexer_1.TokenType.PNP, lexer_1.TokenType.NMOS, lexer_1.TokenType.PMOS,
            lexer_1.TokenType.OPAMP,
            lexer_1.TokenType.AND, lexer_1.TokenType.OR, lexer_1.TokenType.NOT, lexer_1.TokenType.NAND, lexer_1.TokenType.NOR, lexer_1.TokenType.XOR,
            lexer_1.TokenType.GROUND, lexer_1.TokenType.BATTERY, lexer_1.TokenType.SWITCH
        ].includes(token.type);
    }
    isValueToken(token) {
        // Value token is a number
        return token.type === lexer_1.TokenType.NUMBER;
    }
    isUnitToken(token) {
        // Unit token is one of the recognized units
        return token.type === lexer_1.TokenType.UNIT;
    }
    getDefaultUnit(componentType) {
        switch (componentType) {
            case 'resistor':
                return 'Ω';
            case 'capacitor':
                return 'F';
            case 'inductor':
                return 'H';
            case 'voltage':
                return 'V';
            case 'current':
                return 'A';
            default:
                return '';
        }
    }
    currentToken() {
        if (this.position < this.tokens.length) {
            return this.tokens[this.position];
        }
        // Return EOF token if we've reached the end
        return { type: lexer_1.TokenType.EOF, value: '', line: 0, column: 0 };
    }
    advance() {
        if (this.position < this.tokens.length) {
            return this.tokens[this.position++];
        }
        // Return EOF token if we've reached the end
        return { type: lexer_1.TokenType.EOF, value: '', line: 0, column: 0 };
    }
    consume(tokenType, errorMessage) {
        const token = this.currentToken();
        if (token.type !== tokenType) {
            throw new Error(`${errorMessage}. Got ${token.type} (${token.value}) at line ${token.line}, column ${token.column}`);
        }
        return this.advance();
    }
}
exports.Parser = Parser;
//# sourceMappingURL=parser.js.map