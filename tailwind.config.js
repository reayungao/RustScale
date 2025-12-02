/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                rust: {
                    DEFAULT: '#E05D44', // Rust Orange
                    950: '#3F1D18',
                },
                burnt: {
                    DEFAULT: '#EA580C', // Burnt Orange
                }
            }
        },
    },
    plugins: [],
}
