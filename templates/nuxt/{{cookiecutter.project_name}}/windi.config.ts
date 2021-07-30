import { defineConfig } from 'windicss/helpers'
import defaultTheme from 'windicss/defaultTheme'
import colors from 'windicss/colors'

export default defineConfig({
  extract: {
    include: ['**/*.{vue,html,jsx,tsx}'],
    exclude: ['node_modules', '.git'],
  },
  darkMode: 'class',
  presets: [require('frontend-commons/tailwind/preset')],
  theme: {
    fontFamily: {
      banner: ['Nexa Regular Bold', 'serif'],
      heading: ['Fira Sans', ...defaultTheme.fontFamily.sans],
      sans: ['Inter', ...defaultTheme.fontFamily.sans],
    },
    extend: {
      animation: {
        levitate: 'levitate 2s ease-in-out infinite',
        'color-pulse': 'color-pulse 5s infinite',
      },
      backgroundOpacity: { 98: '0.98' },
      colors: {
        flamingo: '#EE4037',
        'old-rose': '#C08472',
        glacier: '#74B4BE',
        'cod-gray': '#111011',
        'vista-white': '#FAF2F1',
        'warm-gray': colors.warmGray,
        'cool-gray': colors.coolGray,
        'blood-red': '#bb0a1e',
      },
      keyframes: {
        levitate: {
          '0%, 100%': { transform: 'translateY(2px)' },
          '50%': { transform: 'translateY(-2px)' },
        },
        'color-pulse': {
          '0%, 100%': { color: '#EE4037' },
          '50%': { color: '#2563EB' },
          '75%': { color: '#279B48' },
        },
      },
      scale: {
        25: '.25',
      },
      transitionProperty: {
        width: 'width',
        spacing: 'margin, padding',
      },
    },
  },
})
