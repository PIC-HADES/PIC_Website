/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        hades: {
          50:  "#f8f6ef",
          100: "#ede6d4",
          200: "#dbce9f",
          300: "#c6b36b",
          400: "#ad9347",
          500: "#8f7636",
          600: "#715c29",
          700: "#54431e",
          800: "#392d13",
          900: "#221b0c",
          950: "#141005",
        },
        naval: {
          50:  "#fcf4f3",
          100: "#f7e4e1",
          200: "#eec6be",
          300: "#e29e91",
          400: "#d47465",
          500: "#b55141",
          600: "#933a2d",
          700: "#762e22",
          800: "#5d231a",
          900: "#441811",
          950: "#2b0e0a",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
      animation: {
        "fade-in": "fadeIn 0.8s ease-out forwards",
        "slide-up": "slideUp 0.8s ease-out forwards",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        slideUp: {
          "0%": { opacity: "0", transform: "translateY(30px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
      },
    },
  },
  plugins: [],
};
