/**
 * Component symbol definitions for semiconductors
 */

import { ComponentSymbol } from './passive';

export const diodeSymbol: ComponentSymbol = {
  type: 'diode',
  symbol: 'M 0,0 L 30,0 L 50,-15 L 50,15 L 30,0 L 70,0',
  pins: [
    { name: 'A', offset: [0, 0] },
    { name: 'C', offset: [70, 0] }
  ]
};

export const ledSymbol: ComponentSymbol = {
  type: 'led',
  symbol: 'M 0,0 L 30,0 L 50,-15 L 50,15 L 30,0 L 70,0 M 65,-20 L 60,-25 M 70,-25 L 65,-30 M 70,-25 L 75,-30 M 70,-25 L 70,-35',
  pins: [
    { name: 'A', offset: [0, 0] },
    { name: 'C', offset: [70, 0] }
  ]
};

export const npnSymbol: ComponentSymbol = {
  type: 'npn',
  symbol: 'M 0,30 L 0,-30 M 30,0 L 0,0 M 30,-15 L 50,-25 M 30,15 L 50,25 M 50,-25 L 50,25 L 70,0 L 50,-25 M 70,0 L 100,0 M 35,-20 L 30,-25 M 40,-25 L 35,-30 M 40,-25 L 45,-30 M 40,-25 L 40,-35',
  pins: [
    { name: 'B', offset: [0, 0] },
    { name: 'C', offset: [100, 0] },
    { name: 'E', offset: [30, 30] }
  ]
};

export const pnpSymbol: ComponentSymbol = {
  type: 'pnp',
  symbol: 'M 0,30 L 0,-30 M 30,0 L 0,0 M 30,-15 L 50,-25 M 30,15 L 50,25 M 50,-25 L 50,25 L 70,0 L 50,-25 M 70,0 L 100,0 M 70,0 L 75,-5 M 70,0 L 75,5',
  pins: [
    { name: 'B', offset: [0, 0] },
    { name: 'C', offset: [100, 0] },
    { name: 'E', offset: [30, 30] }
  ]
};

export const nmosSymbol: ComponentSymbol = {
  type: 'nmos',
  symbol: 'M 0,30 L 0,-30 M 30,0 L 0,0 M 30,-15 L 50,-25 M 30,15 L 50,25 M 50,-25 L 50,25 L 70,0 L 50,-25 M 70,0 L 100,0 M 50,-35 L 50,-50 M 40,-50 L 60,-50 M 50,-50 L 50,-60',
  pins: [
    { name: 'G', offset: [50, -60] },
    { name: 'D', offset: [100, 0] },
    { name: 'S', offset: [30, 30] }
  ]
};

export const pmosSymbol: ComponentSymbol = {
  type: 'pmos',
  symbol: 'M 0,30 L 0,-30 M 30,0 L 0,0 M 30,-15 L 50,-25 M 30,15 L 50,25 M 50,-25 L 50,25 L 70,0 L 50,-25 M 70,0 L 100,0 M 50,-35 L 50,-50 M 40,-50 L 60,-50 M 50,-50 L 50,-60 M 50,-60 L 55,-55 M 50,-60 L 45,-55',
  pins: [
    { name: 'G', offset: [50, -60] },
    { name: 'D', offset: [100, 0] },
    { name: 'S', offset: [30, 30] }
  ]
};

export const semiconductors = {
  diode: diodeSymbol,
  led: ledSymbol,
  npn: npnSymbol,
  pnp: pnpSymbol,
  nmos: nmosSymbol,
  pmos: pmosSymbol
};