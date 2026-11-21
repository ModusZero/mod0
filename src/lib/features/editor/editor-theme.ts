import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { EditorView } from '@codemirror/view';
import { tags } from '@lezer/highlight';

/** Tema del editor basado en los tokens de `app.css`, así cambia solo con el tema de la app. */
export const editorTheme = EditorView.theme({
    '&': {
        height: '100%',
        backgroundColor: 'var(--bg-main)',
        color: 'var(--text-main)',
        fontSize: '0.8125rem',
    },
    '&.cm-focused': { outline: 'none' },
    '.cm-scroller': { fontFamily: 'var(--font-mono)', lineHeight: '1.6', overflow: 'auto' },
    '.cm-content': { caretColor: 'var(--text-main)', padding: '8px 0' },
    '.cm-gutters': { backgroundColor: 'var(--bg-main)', color: 'var(--text-faint)', border: 'none' },
    '.cm-activeLine': { backgroundColor: 'var(--bg-hover)' },
    '.cm-activeLineGutter': { backgroundColor: 'transparent', color: 'var(--text-muted)' },
    '.cm-cursor': { borderLeftColor: 'var(--text-main)' },
    '.cm-selectionBackground, &.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground': {
        backgroundColor: 'var(--accent-soft)',
    },
    '.cm-matchingBracket': { backgroundColor: 'var(--bg-active)', outline: 'none' },
    '.cm-panels': { backgroundColor: 'var(--bg-sidebar)', color: 'var(--text-main)' },
    '.cm-tooltip': {
        backgroundColor: 'var(--bg-raised)',
        border: '1px solid var(--border-subtle)',
        color: 'var(--text-main)',
    },
    '.cm-searchMatch': { backgroundColor: 'var(--accent-soft)' },
});

export const highlightTheme = syntaxHighlighting(
    HighlightStyle.define([
        { tag: [tags.keyword, tags.operatorKeyword, tags.modifier], color: 'var(--syn-keyword)' },
        { tag: [tags.string, tags.special(tags.string), tags.regexp], color: 'var(--syn-string)' },
        { tag: [tags.number, tags.bool, tags.null, tags.atom], color: 'var(--syn-number)' },
        {
            tag: [tags.comment, tags.lineComment, tags.blockComment],
            color: 'var(--syn-comment)',
            fontStyle: 'italic',
        },
        {
            tag: [tags.function(tags.variableName), tags.function(tags.propertyName)],
            color: 'var(--syn-function)',
        },
        { tag: [tags.typeName, tags.className, tags.namespace], color: 'var(--syn-type)' },
        { tag: [tags.propertyName, tags.attributeName], color: 'var(--syn-function)' },
        { tag: [tags.punctuation, tags.bracket, tags.operator], color: 'var(--syn-punct)' },
        { tag: tags.heading, color: 'var(--syn-function)', fontWeight: '600' },
        { tag: tags.link, color: 'var(--accent)', textDecoration: 'underline' },
    ]),
);
