"use strict";
// Geometry utilities for positioning and transformations
Object.defineProperty(exports, "__esModule", { value: true });
exports.rotatePoint = rotatePoint;
exports.distance = distance;
exports.midpoint = midpoint;
exports.translatePoint = translatePoint;
exports.angleBetween = angleBetween;
exports.angleBetweenDegrees = angleBetweenDegrees;
exports.arePointsClose = arePointsClose;
exports.scalePoint = scalePoint;
// Rotate a point around a center by a given angle (in radians)
function rotatePoint(point, center, angleRad) {
    // Translate point to origin
    const translatedX = point.x - center.x;
    const translatedY = point.y - center.y;
    // Apply rotation
    const rotatedX = translatedX * Math.cos(angleRad) - translatedY * Math.sin(angleRad);
    const rotatedY = translatedX * Math.sin(angleRad) + translatedY * Math.cos(angleRad);
    // Translate back
    return {
        x: rotatedX + center.x,
        y: rotatedY + center.y
    };
}
// Calculate the distance between two points
function distance(p1, p2) {
    return Math.sqrt(Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2));
}
// Calculate the midpoint between two points
function midpoint(p1, p2) {
    return {
        x: (p1.x + p2.x) / 2,
        y: (p1.y + p2.y) / 2
    };
}
// Translate a point by dx, dy
function translatePoint(point, dx, dy) {
    return {
        x: point.x + dx,
        y: point.y + dy
    };
}
// Calculate the angle between two points in radians
function angleBetween(p1, p2) {
    return Math.atan2(p2.y - p1.y, p2.x - p1.x);
}
// Calculate the angle between two points in degrees
function angleBetweenDegrees(p1, p2) {
    return angleBetween(p1, p2) * 180 / Math.PI;
}
// Check if two points are close to each other within a tolerance
function arePointsClose(p1, p2, tolerance = 5) {
    return distance(p1, p2) <= tolerance;
}
// Scale a point by a factor relative to a center
function scalePoint(point, center, factor) {
    const dx = point.x - center.x;
    const dy = point.y - center.y;
    return {
        x: center.x + dx * factor,
        y: center.y + dy * factor
    };
}
//# sourceMappingURL=geometry.js.map