/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  darkMode: "class",
  content: ["./**/*.tsx"],
  plugins: [],
  theme: {
    extend: {
      boxShadow: {
        around: "0 0 8px 2px rgba(0,0,0,0.4)"
      }
    }
  }
}
