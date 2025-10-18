/**
 * Lexer for the Circuit Description Language
 */
export interface Token {
    type: TokenType;
    value: string;
    line: number;
    column: number;
}
export declare enum TokenType {
    CIRCUIT = "CIRCUIT",
    RESISTOR = "RESISTOR",
    CAPACITOR = "CAPACITOR",
    INDUCTOR = "INDUCTOR",
    VOLTAGE = "VOLTAGE",
    CURRENT = "CURRENT",
    DIODE = "DIODE",
    LED = "LED",
    NPN = "NPN",
    PNP = "PNP",
    NMOS = "NMOS",
    PMOS = "PMOS",
    OPAMP = "OPAMP",
    AND = "AND",
    OR = "OR",
    NOT = "NOT",
    NAND = "NAND",
    NOR = "NOR",
    XOR = "XOR",
    GROUND = "GROUND",
    BATTERY = "BATTERY",
    SWITCH = "SWITCH",
    ID = "ID",
    NUMBER = "NUMBER",
    UNIT = "UNIT",
    NEWLINE = "NEWLINE",
    EOF = "EOF",
    COMMENT = "COMMENT"
}
export declare class Lexer {
    private input;
    private position;
    private currentLine;
    private currentColumn;
    constructor(input: string);
    tokenize(): Token[];
    private advance;
    private readComment;
    private readNumber;
    private readIdentifier;
    private readUnit;
    private getKeywordType;
}
//# sourceMappingURL=lexer.d.ts.map