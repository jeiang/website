/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    colors: {
      primary: "#1f1f1f",
      secondary: "#ede6e3",
    },
    fontFamily: {
      sans: ["Inter", "sans-serif"],
      serif: ["Merriweather", "serif"],
      mono: ["Fira Code", "monospace"],
    },
    extend: {},
  },
  plugins: [],
};
