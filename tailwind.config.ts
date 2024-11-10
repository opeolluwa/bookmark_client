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

      "app-secondary": {
        DEFAULT: "#EC8305",
        50: "#FDD8AB",
        100: "#FDCF97",
        200: "#FCBC6F",
        300: "#FBA947",
        400: "#FA9720",
        500: "#EC8305",
        600: "#B56504",
        700: "#7E4603",
        800: "#472802",
        900: "#100900",
        950: "#000000",
      },
      app: {
        DEFAULT: "#488AF3",
        50: "#F4F8FE",
        100: "#E1ECFD",
        200: "#BBD3FB",
        300: "#95BBF8",
        400: "#6EA2F6",
        500: "#488AF3",
        600: "#1368F0",
        700: "#0C51BE",
        800: "#093B8A",
        900: "#062455",
        950: "#04193B",
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
  plugins: [require("daisyui")],
};
export default config;
