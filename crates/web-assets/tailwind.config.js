/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../web-pages/**/*.rs"],
  /** We need this because we use daisy-rsx library */
  safelist: [
    {
      pattern: /avatar*|alert*|modal*|btn*|menu*|dropdown*|badge*|card*|input*|select*|textarea*|label*|tab*|tooltip*|flex*|text*|overflow*/
    }
  ],
  theme: {},
  plugins: [
    require("daisyui"),
    require('@tailwindcss/typography')
  ]
}