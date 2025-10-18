/**
 * Component symbol definitions for sources
 */

import { ComponentSymbol } from './passive';

export const voltageSourceSymbol: ComponentSymbol = {
  type: 'voltage',
  symbol: 'M 0,0 L 45,0 M 45,-20 A 25,25 0 1,0 45,20 A 25,25 0 1,0 45,-20 M 55,-20 A 25,25 0 1,1 55,20 A 25,25 0 1,1 55,-20 M 55,0 L 100,0',
  pins: [
    { name: '+', offset: [0, 0] },
    { name: '-', offset: [100, 0] }
  ]
};

export const currentSourceSymbol: ComponentSymbol = {
  type: 'current',
  symbol: 'M 0,0 L 45,0 M 45,-25 A 25,25 0 1,0 45,25 A 25,25 0 1,0 45,-25 M 55,0 L 100,0 M 65,-10 L 75,0 L 65,10',
  pins: [
    { name: '+', offset: [0, 0] },
    { name: '-', offset: [100, 0] }
  ]
};

export const groundSymbol: ComponentSymbol = {
  type: 'ground',
  symbol: 'M 0,0 L 0,10 M -15,10 L 15,10 M -10,15 L 10,15 M -5,20 L 5,20',
  pins: [
    { name: 'GND', offset: [0, 0] }
  ]
};

export const batterySymbol: ComponentSymbol = {
  type: 'battery',
  symbol: 'M 0,0 L 30,0 M 30,-10 L 30,10 M 40,-5 L 40,5 M 40,0 L 100,0',
  pins: [
    { name: '+', offset: [0, 0] },
    { name: '-', offset: [100, 0] }
  ]
};

export const sources = {
  voltage: voltageSourceSymbol,
  current: currentSourceSymbol,
  ground: groundSymbol,
  battery: batterySymbol
};