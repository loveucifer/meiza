/**
 * Component registry for Circuit Description Language
 */

import { passiveComponents } from './passive';
import { sources } from './sources';
import { semiconductors } from './semiconductors';
import { ics } from './ics';

// Combine all component symbols into a single registry
export const componentRegistry = {
  ...passiveComponents,
  ...sources,
  ...semiconductors,
  ...ics
};

export type ComponentType = keyof typeof componentRegistry;

export function getComponentSymbol(type: ComponentType) {
  return componentRegistry[type];
}

export function isComponentType(type: string): type is ComponentType {
  return type in componentRegistry;
}