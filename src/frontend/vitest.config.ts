import { getViteConfig } from 'astro/config'

export default getViteConfig({
  test: {
    coverage: {
      extension: ['.astro', '.ts', '.tsx', '.svelte'],
      provider: 'v8',
      reporter: ['text', 'json-summary', 'json', 'cobertura']
    }
  }
})
