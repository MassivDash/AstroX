import { getViteConfig } from 'astro/config'

export default getViteConfig({
  test: {
    coverage: {
      reporter: ['text', 'json-summary', 'json', 'cobertura']
    }
  }
})
