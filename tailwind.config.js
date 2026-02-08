/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        main: 'var(--bg-main)',
        sidebar: 'var(--bg-sidebar)',
        accent: 'var(--accent)',
        text: 'var(--text-main)',
      },
      borderColor: {
        DEFAULT: 'var(--border)',
      }
    },
  },
  plugins: [],
}