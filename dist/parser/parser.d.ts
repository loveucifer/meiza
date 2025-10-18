/**
 * Recursive descent parser for the Circuit Description Language
 */
import { Circuit } from './ast';
export declare class Parser {
    private tokens;
    private position;
    constructor(input: string);
    parse(): Circuit;
    private parseComponent;
    private isComponentDeclaration;
    private isComponentTypeToken;
    private isValueToken;
    private isUnitToken;
    private getDefaultUnit;
    private currentToken;
    private advance;
    private consume;
}
//# sourceMappingURL=parser.d.ts.map