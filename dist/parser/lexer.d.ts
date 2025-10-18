export declare const TokenTypes: {
    ID: "ID";
    NUMBER: "NUMBER";
    STRING: "STRING";
    KEYWORD: "KEYWORD";
    NEWLINE: "NEWLINE";
    WHITESPACE: "WHITESPACE";
    COMMENT: "COMMENT";
    EOF: "EOF";
};
export type TokenType = typeof TokenTypes[keyof typeof TokenTypes];
export interface Token {
    type: TokenType;
    value: string;
    line: number;
    column: number;
}
export declare class Lexer {
    private input;
    private position;
    private line;
    private column;
    private currentChar;
    constructor(input: string);
    private advance;
    private skipWhitespace;
    private readNumber;
    private readIdentifier;
    private readComment;
    getNextToken(): Token;
    getAllTokens(): Token[];
}
//# sourceMappingURL=lexer.d.ts.map