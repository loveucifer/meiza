"use strict";
// Unit conversion utilities for circuit values
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseValueWithUnit = parseValueWithUnit;
exports.convertToUnit = convertToUnit;
exports.formatValueWithUnit = formatValueWithUnit;
// Common unit prefixes
const UNIT_PREFIXES = [
    { symbol: 'p', multiplier: 1e-12, name: 'pico' },
    { symbol: 'n', multiplier: 1e-9, name: 'nano' },
    { symbol: 'u', multiplier: 1e-6, name: 'micro' },
    { symbol: 'm', multiplier: 1e-3, name: 'milli' },
    { symbol: 'k', multiplier: 1e3, name: 'kilo' },
    { symbol: 'M', multiplier: 1e6, name: 'mega' },
    { symbol: 'G', multiplier: 1e9, name: 'giga' },
    { symbol: 'T', multiplier: 1e12, name: 'tera' }
];
// Function to parse a value with unit and return the numeric value in base units
function parseValueWithUnit(input) {
    const regex = /^([\d.]+)\s*([a-zA-Z]*)$/;
    const match = input.match(regex);
    if (!match) {
        throw new Error(`Invalid value format: ${input}`);
    }
    const [, numericStr, unit] = match;
    const numericValue = parseFloat(numericStr);
    if (isNaN(numericValue)) {
        throw new Error(`Invalid numeric value: ${numericStr}`);
    }
    // If no unit is specified, return the numeric value as is
    if (!unit) {
        return numericValue;
    }
    // Find the matching prefix
    const prefix = UNIT_PREFIXES.find(p => p.symbol === unit);
    if (!prefix) {
        throw new Error(`Unknown unit prefix: ${unit}`);
    }
    return numericValue * prefix.multiplier;
}
// Function to convert a base value to a specified unit
function convertToUnit(value, targetUnit) {
    const prefix = UNIT_PREFIXES.find(p => p.symbol === targetUnit);
    if (!prefix) {
        throw new Error(`Unknown unit prefix: ${targetUnit}`);
    }
    return value / prefix.multiplier;
}
// Function to format a value with an appropriate unit prefix
function formatValueWithUnit(value, baseUnit = '') {
    if (value === 0) {
        return `0${baseUnit}`;
    }
    // Find the most appropriate prefix based on the magnitude of the value
    let absValue = Math.abs(value);
    let selectedPrefix = null;
    // Find the prefix that gives a value between 1 and 1000 (or between 0.1 and 0.999 for the next smaller prefix)
    for (let i = UNIT_PREFIXES.length - 1; i >= 0; i--) {
        const prefix = UNIT_PREFIXES[i];
        const scaledValue = absValue / prefix.multiplier;
        if (scaledValue >= 1 || i === 0) { // Use this prefix if value >= 1, or if it's the smallest prefix
            selectedPrefix = prefix;
            absValue = scaledValue;
            break;
        }
    }
    // If no prefix was applicable (value is between 1 and 1000), use base units
    if (!selectedPrefix) {
        return `${value}${baseUnit}`;
    }
    // Adjust the sign back
    const finalValue = value < 0 ? -absValue : absValue;
    return `${finalValue}${selectedPrefix.symbol}${baseUnit}`;
}
//# sourceMappingURL=units.js.map