/**
 * Lexer for the Circuit Description Language
 */

export interface Token {
  type: TokenType;
  value: string;
  line: number;
  column: number;
}

export enum TokenType {
  // Keywords
  CIRCUIT = 'CIRCUIT',
  RESISTOR = 'RESISTOR',
  CAPACITOR = 'CAPACITOR',
  INDUCTOR = 'INDUCTOR',
  VOLTAGE = 'VOLTAGE',
  CURRENT = 'CURRENT',
  DIODE = 'DIODE',
  LED = 'LED',
  NPN = 'NPN',
  PNP = 'PNP',
  NMOS = 'NMOS',
  PMOS = 'PMOS',
  OPAMP = 'OPAMP',
  AND = 'AND',
  OR = 'OR',
  NOT = 'NOT',
  NAND = 'NAND',
  NOR = 'NOR',
  XOR = 'XOR',
  GROUND = 'GROUND',
  BATTERY = 'BATTERY',
  SWITCH = 'SWITCH',
  
  // Identifiers and literals
  ID = 'ID',
  NUMBER = 'NUMBER',
  UNIT = 'UNIT',
  
  // Punctuation
  NEWLINE = 'NEWLINE',
  EOF = 'EOF',
  
  // Comments
  COMMENT = 'COMMENT',
}

export class Lexer {
  private input: string;
  private position: number = 0;
  private currentLine: number = 1;
  private currentColumn: number = 0;
  
  constructor(input: string) {
    this.input = input;
  }
  
  public tokenize(): Token[] {
    const tokens: Token[] = [];
    
    while (this.position < this.input.length) {
      const char = this.input[this.position];
      
      // Skip whitespace except newlines
      if (/\s/.test(char) && char !== '\n') {
        this.advance();
        continue;
      }
      
      // Handle newlines
      if (char === '\n') {
        tokens.push({
          type: TokenType.NEWLINE,
          value: '\n',
          line: this.currentLine,
          column: this.currentColumn
        });
        this.advance();
        continue;
      }
      
      // Handle comments
      if (char === '#') {
        const comment = this.readComment();
        tokens.push({
          type: TokenType.COMMENT,
          value: comment,
          line: this.currentLine,
          column: this.currentColumn
        });
        continue;
      }
      
      // Handle numbers
      if (/\d/.test(char) || char === '.') {
        const number = this.readNumber();
        tokens.push({
          type: TokenType.NUMBER,
          value: number,
          line: this.currentLine,
          column: this.currentColumn
        });
        continue;
      }
      
      // Handle identifiers/keywords
      if (/[a-zA-Z_]/.test(char)) {
        const identifier = this.readIdentifier();
        const type = this.getKeywordType(identifier) || TokenType.ID;
        tokens.push({
          type,
          value: identifier,
          line: this.currentLine,
          column: this.currentColumn
        });
        continue;
      }
      
      // Handle units
      if (/[kMugnpf]/.test(char)) {
        const unit = this.readUnit();
        tokens.push({
          type: TokenType.UNIT,
          value: unit,
          line: this.currentLine,
          column: this.currentColumn
        });
        continue;
      }
      
      // If we get here, we have an unexpected character
      throw new Error(`Unexpected character: ${char} at line ${this.currentLine}, column ${this.currentColumn}`);
    }
    
    // Add EOF token
    tokens.push({
      type: TokenType.EOF,
      value: '',
      line: this.currentLine,
      column: this.currentColumn
    });
    
    return tokens;
  }
  
  private advance(): void {
    if (this.input[this.position] === '\n') {
      this.currentLine++;
      this.currentColumn = 0;
    } else {
      this.currentColumn++;
    }
    this.position++;
  }
  
  private readComment(): string {
    let comment = '#';
    this.advance(); // skip the '#'
    
    while (this.position < this.input.length && this.input[this.position] !== '\n') {
      comment += this.input[this.position];
      this.advance();
    }
    
    return comment;
  }
  
  private readNumber(): string {
    let number = '';
    
    // Handle decimal point at the beginning
    if (this.input[this.position] === '.') {
      number += this.input[this.position];
      this.advance();
    }
    
    // Read digits
    while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
      number += this.input[this.position];
      this.advance();
    }
    
    // Check for decimal point
    if (this.position < this.input.length && this.input[this.position] === '.') {
      number += this.input[this.position];
      this.advance();
      
      // Read more digits after decimal point
      while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
        number += this.input[this.position];
        this.advance();
      }
    }
    
    return number;
  }
  
  private readIdentifier(): string {
    let identifier = '';
    
    while (this.position < this.input.length && /[a-zA-Z_0-9]/.test(this.input[this.position])) {
      identifier += this.input[this.position];
      this.advance();
    }
    
    return identifier;
  }
  
  private readUnit(): string {
    let unit = this.input[this.position];
    this.advance();
    
    // Units can be single characters like k, M, u, n, p, f
    // Or combinations like Ohm, Hz, etc.
    if (this.position < this.input.length && /[OHhz]/.test(this.input[this.position])) {
      unit += this.input[this.position];
      this.advance();
    }
    
    return unit;
  }
  
  private getKeywordType(keyword: string): TokenType | undefined {
    switch (keyword.toLowerCase()) {
      case 'circuit':
        return TokenType.CIRCUIT;
      case 'resistor':
        return TokenType.RESISTOR;
      case 'capacitor':
        return TokenType.CAPACITOR;
      case 'inductor':
        return TokenType.INDUCTOR;
      case 'voltage':
        return TokenType.VOLTAGE;
      case 'current':
        return TokenType.CURRENT;
      case 'diode':
        return TokenType.DIODE;
      case 'led':
        return TokenType.LED;
      case 'npn':
        return TokenType.NPN;
      case 'pnp':
        return TokenType.PNP;
      case 'nmos':
        return TokenType.NMOS;
      case 'pmos':
        return TokenType.PMOS;
      case 'opamp':
        return TokenType.OPAMP;
      case 'and':
        return TokenType.AND;
      case 'or':
        return TokenType.OR;
      case 'not':
        return TokenType.NOT;
      case 'nand':
        return TokenType.NAND;
      case 'nor':
        return TokenType.NOR;
      case 'xor':
        return TokenType.XOR;
      case 'ground':
        return TokenType.GROUND;
      case 'battery':
        return TokenType.BATTERY;
      case 'switch':
        return TokenType.SWITCH;
      default:
        return undefined;
    }
  }
}