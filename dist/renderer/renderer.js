"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BaseRenderer = void 0;
class BaseRenderer {
    applyTheme(theme) {
        if (theme === 'dark') {
            return {
                background: '#1e1e1e',
                component: '#d4d4d4',
                wire: '#8bc34a',
                text: '#ffffff',
                grid: '#444444'
            };
        }
        else {
            return {
                background: '#ffffff',
                component: '#000000',
                wire: '#000000',
                text: '#000000',
                grid: '#e0e0e0'
            };
        }
    }
}
exports.BaseRenderer = BaseRenderer;
//# sourceMappingURL=renderer.js.map