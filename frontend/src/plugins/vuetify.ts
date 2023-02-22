import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

import { createVuetify } from 'vuetify'

export default createVuetify({
  theme: {
    themes: {
      light: {
        colors: {
          primary: '#016060',
          secondary: '#424242',
        },
      },
      dark: {
        colors: {
          primary: '#008e9b',
          secondary: '#424242',
        },
      }
    },
  },
})
