import { Circuit } from './ast';
export declare class Parser {
    private lexer;
    private currentToken;
    private tokens;
    private position;
    constructor(input: string);
    private advance;
    private match;
    private consume;
    private parseValue;
    private parseComponent;
    parse(): Circuit;
}
export declare function parseCircuit(input: string): Circuit;
//# sourceMappingURL=parser.d.ts.map