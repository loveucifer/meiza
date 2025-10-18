import { Position } from '../parser/ast';
export declare function rotatePoint(point: Position, center: Position, angleRad: number): Position;
export declare function distance(p1: Position, p2: Position): number;
export declare function midpoint(p1: Position, p2: Position): Position;
export declare function translatePoint(point: Position, dx: number, dy: number): Position;
export declare function angleBetween(p1: Position, p2: Position): number;
export declare function angleBetweenDegrees(p1: Position, p2: Position): number;
export declare function arePointsClose(p1: Position, p2: Position, tolerance?: number): boolean;
export declare function scalePoint(point: Position, center: Position, factor: number): Position;
//# sourceMappingURL=geometry.d.ts.map