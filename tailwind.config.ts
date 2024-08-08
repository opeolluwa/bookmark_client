import type { Config } from "tailwindcss";
import colors, { slate } from "tailwindcss/colors"; 

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/Layouts/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      black: colors.black,
      white: colors.white,
      gray: colors.gray,
      emerald: colors.emerald,
      indigo: colors.indigo,
      yellow: colors.yellow,
      dark: colors.neutral,
      red: colors.red,
      accent: "rgba(226,233,252,255)",
      card: "#f9fbfe",
      slate: colors.slate,
      app: {
        DEFAULT: "#75CE8E",
        50: "#FCFEFD",
        100: "#EDF9F1",
        200: "#CFEED8",
        300: "#B1E3BF",
        400: "#93D9A7",
        500: "#75CE8E",
        600: "#4CBF6C",
        700: "#379C53",
        800: "#29723D",
        900: "#1A4927",
        950: "#12341C",
      },
    },
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
    },
  },
  plugins: [],
};
export default config;


