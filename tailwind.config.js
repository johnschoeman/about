const production = !process.env.ROLLUP_WATCH

module.exports = {
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      brightness: ["hover", "focus"],
    },
  },
  plugins: [],
  purge: {
    content: ["./src/**/*.rs"],
    enabled: production, // disable purge in dev
  },
}
