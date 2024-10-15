/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/main/kotlin/**/*.kt", "./src/main/kotlin/*.kt"],
  theme: {
    colors: {
      // dark
      primary: "#1f1f1f",
      // light
      secondary: "#ede6e3",
    },
    fontFamily: {
      sans: ["Inter", "sans-serif"],
      serif: ["Libre Baskerville", "serif"],
      mono: ["Fira Code", "monospace"],
    },
    extend: {},
  },
  plugins: [],
};
