/**
 * Component symbol definitions for integrated circuits and logic gates
 */

import { ComponentSymbol } from './passive';

export const opampSymbol: ComponentSymbol = {
  type: 'opamp',
  symbol: 'M 0,-40 L 0,40 L 60,0 Z M -20,-15 L 0,-15 M -20,15 L 0,15 M 60,0 L 80,0',
  pins: [
    { name: '+', offset: [-20, 15] },
    { name: '-', offset: [-20, -15] },
    { name: 'out', offset: [80, 0] },
    { name: 'V+', offset: [30, -40] },
    { name: 'V-', offset: [30, 40] }
  ]
};

export const andGateSymbol: ComponentSymbol = {
  type: 'and',
  symbol: 'M 0,-30 L 0,30 L 40,30 Q 60,0 40,-30 Z M -20,-15 L 0,-15 M -20,15 L 0,15 M 60,0 L 80,0',
  pins: [
    { name: 'A', offset: [-20, -15] },
    { name: 'B', offset: [-20, 15] },
    { name: 'Y', offset: [80, 0] }
  ]
};

export const orGateSymbol: ComponentSymbol = {
  type: 'or',
  symbol: 'M 0,-30 L 40,0 L 0,30 Q 20,15 40,0 Q 20,-15 0,-30 Z M -20,-15 L 0,-15 M -20,15 L 0,15 M 60,0 L 80,0',
  pins: [
    { name: 'A', offset: [-20, -15] },
    { name: 'B', offset: [-20, 15] },
    { name: 'Y', offset: [80, 0] }
  ]
};

export const notGateSymbol: ComponentSymbol = {
  type: 'not',
  symbol: 'M 0,-20 L 0,20 L 40,0 Z M 40,0 L 60,0 M 60,-5 A 5,5 0 1,0 60,5 A 5,5 0 1,0 60,-5 Z',
  pins: [
    { name: 'A', offset: [-20, 0] },
    { name: 'Y', offset: [70, 0] }
  ]
};

export const nandGateSymbol: ComponentSymbol = {
  type: 'nand',
  symbol: 'M 0,-30 L 0,30 L 40,30 Q 60,0 40,-30 Z M 60,0 L 80,0 M 80,-5 A 5,5 0 1,0 80,5 A 5,5 0 1,0 80,-5 Z M -20,-15 L 0,-15 M -20,15 L 0,15',
  pins: [
    { name: 'A', offset: [-20, -15] },
    { name: 'B', offset: [-20, 15] },
    { name: 'Y', offset: [90, 0] }
  ]
};

export const norGateSymbol: ComponentSymbol = {
  type: 'nor',
  symbol: 'M 0,-30 L 40,0 L 0,30 Q 20,15 40,0 Q 20,-15 0,-30 Z M 60,0 L 80,0 M 80,-5 A 5,5 0 1,0 80,5 A 5,5 0 1,0 80,-5 Z M -20,-15 L 0,-15 M -20,15 L 0,15',
  pins: [
    { name: 'A', offset: [-20, -15] },
    { name: 'B', offset: [-20, 15] },
    { name: 'Y', offset: [90, 0] }
  ]
};

export const xorGateSymbol: ComponentSymbol = {
  type: 'xor',
  symbol: 'M 0,-30 L 40,0 L 0,30 Q 20,15 40,0 Q 20,-15 0,-30 Z M -10,-30 Q 10,0 -10,30 M -20,-15 L -10,-15 M -20,15 L -10,15 M 60,0 L 80,0',
  pins: [
    { name: 'A', offset: [-20, -15] },
    { name: 'B', offset: [-20, 15] },
    { name: 'Y', offset: [80, 0] }
  ]
};

export const ics = {
  opamp: opampSymbol,
  and: andGateSymbol,
  or: orGateSymbol,
  not: notGateSymbol,
  nand: nandGateSymbol,
  nor: norGateSymbol,
  xor: xorGateSymbol
};