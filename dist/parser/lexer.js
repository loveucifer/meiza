"use strict";
// CDL Lexer - Tokenizes input string into tokens
Object.defineProperty(exports, "__esModule", { value: true });
exports.Lexer = exports.TokenTypes = void 0;
exports.TokenTypes = {
    ID: 'ID',
    NUMBER: 'NUMBER',
    STRING: 'STRING',
    KEYWORD: 'KEYWORD',
    NEWLINE: 'NEWLINE',
    WHITESPACE: 'WHITESPACE',
    COMMENT: 'COMMENT',
    EOF: 'EOF'
};
const KEYWORDS = new Set([
    'circuit',
    'resistor', 'capacitor', 'inductor',
    'voltage', 'current',
    'diode', 'led', 'npn', 'pnp', 'nmos', 'pmos',
    'opamp', 'and', 'or', 'not', 'nand', 'nor', 'xor',
    'ground', 'battery', 'switch'
]);
class Lexer {
    constructor(input) {
        this.position = 0;
        this.line = 1;
        this.column = 1;
        this.input = input;
        this.currentChar = this.input[this.position];
    }
    advance() {
        if (this.currentChar === '\n') {
            this.line++;
            this.column = 1;
        }
        else {
            this.column++;
        }
        this.position++;
        this.currentChar = this.position < this.input.length ? this.input[this.position] : undefined;
    }
    skipWhitespace() {
        while (this.currentChar && /\s/.test(this.currentChar) && this.currentChar !== '\n') {
            this.advance();
        }
    }
    readNumber() {
        let numStr = '';
        const startLine = this.line;
        const startColumn = this.column;
        while (this.currentChar && (/\d/.test(this.currentChar) || this.currentChar === '.')) {
            numStr += this.currentChar;
            this.advance();
        }
        // Check for unit suffix (k, M, u, n, p, etc.)
        let unit = '';
        if (this.currentChar && /[kMunp]/.test(this.currentChar)) {
            unit += this.currentChar;
            this.advance();
        }
        // Check for unit type (F, H, V, A, W, etc.)
        if (this.currentChar && /[FHAVWOhm]/.test(this.currentChar)) {
            unit += this.currentChar;
            this.advance();
        }
        return {
            type: exports.TokenTypes.NUMBER,
            value: numStr + (unit ? ' ' + unit : ''),
            line: startLine,
            column: startColumn
        };
    }
    readIdentifier() {
        let idStr = '';
        const startLine = this.line;
        const startColumn = this.column;
        while (this.currentChar && (/[a-zA-Z0-9_]/.test(this.currentChar))) {
            idStr += this.currentChar;
            this.advance();
        }
        const type = KEYWORDS.has(idStr.toLowerCase()) ? exports.TokenTypes.KEYWORD : exports.TokenTypes.ID;
        return {
            type,
            value: idStr,
            line: startLine,
            column: startColumn
        };
    }
    readComment() {
        let comment = '#';
        const startLine = this.line;
        const startColumn = this.column;
        this.advance(); // skip the '#'
        while (this.currentChar && this.currentChar !== '\n') {
            comment += this.currentChar;
            this.advance();
        }
        return {
            type: exports.TokenTypes.COMMENT,
            value: comment,
            line: startLine,
            column: startColumn
        };
    }
    getNextToken() {
        while (this.currentChar !== undefined) {
            if (this.currentChar === '#') {
                return this.readComment();
            }
            if (/\s/.test(this.currentChar)) {
                if (this.currentChar === '\n') {
                    const token = {
                        type: exports.TokenTypes.NEWLINE,
                        value: '\n',
                        line: this.line,
                        column: this.column
                    };
                    this.advance();
                    return token;
                }
                else {
                    this.skipWhitespace();
                    continue;
                }
            }
            if (/\d/.test(this.currentChar)) {
                return this.readNumber();
            }
            if (/[a-zA-Z_]/.test(this.currentChar)) {
                return this.readIdentifier();
            }
            // Handle special characters
            const char = this.currentChar;
            this.advance();
            return {
                type: exports.TokenTypes.ID, // Treat everything else as ID for now
                value: char,
                line: this.line,
                column: this.column
            };
        }
        return {
            type: exports.TokenTypes.EOF,
            value: '',
            line: this.line,
            column: this.column
        };
    }
    getAllTokens() {
        const tokens = [];
        let token = this.getNextToken();
        while (token.type !== exports.TokenTypes.EOF) {
            tokens.push(token);
            token = this.getNextToken();
        }
        tokens.push(token); // Add EOF token
        return tokens;
    }
}
exports.Lexer = Lexer;
//# sourceMappingURL=lexer.js.map