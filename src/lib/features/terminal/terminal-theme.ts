import type { ITheme } from '@xterm/xterm';

/** Paletas de la terminal, alineadas con los tokens de `app.css` (colores suaves, sin negro puro). */
export function terminalTheme(dark: boolean): ITheme {
    return dark
        ? {
              background: '#0f1115',
              foreground: '#d3d8e0',
              cursor: '#8c95a5',
              cursorAccent: '#0f1115',
              selectionBackground: 'rgba(106,165,247,0.28)',
              black: '#1a1e25',
              red: '#e8706a',
              green: '#55b78c',
              yellow: '#e3ad4a',
              blue: '#6aa5f7',
              magenta: '#ad94f2',
              cyan: '#4fb8cf',
              white: '#d3d8e0',
              brightBlack: '#5d6676',
              brightRed: '#f08a85',
              brightGreen: '#74cca4',
              brightYellow: '#ecc072',
              brightBlue: '#8bbcf9',
              brightMagenta: '#c2aef6',
              brightCyan: '#74cbdd',
              brightWhite: '#eef1f5',
          }
        : {
              background: '#f4f5f7',
              foreground: '#1d2432',
              cursor: '#566175',
              cursorAccent: '#f4f5f7',
              selectionBackground: 'rgba(47,111,214,0.22)',
              black: '#1d2432',
              red: '#c4433b',
              green: '#2b8a62',
              yellow: '#9a6408',
              blue: '#2f6fd6',
              magenta: '#7650d6',
              cyan: '#1f8aa0',
              white: '#d9dde4',
              brightBlack: '#8791a3',
              brightRed: '#d9574e',
              brightGreen: '#37a276',
              brightYellow: '#b4730a',
              brightBlue: '#4a86e8',
              brightMagenta: '#8d68e6',
              brightCyan: '#2ba0b8',
              brightWhite: '#ffffff',
          };
}
