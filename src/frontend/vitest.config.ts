import { getViteConfig } from 'astro/config'
import { svelteTesting } from '@testing-library/svelte/vite'
import tsconfigPaths from 'vite-tsconfig-paths'

export default getViteConfig({
  //@ts-ignore
  test: {
    projects: [
      {
        plugins: [svelteTesting(), tsconfigPaths()],
        extends: true,
        test: {
          setupFiles: './setup.vitest.ts',
          name: 'browser',
          include: ['**/*.browser.test.{ts,tsx,jsx,svelte}'],
          environment: 'js-dom'
        }
      },
      {
        extends: true,
        plugins: [tsconfigPaths()],
        test: {
          name: 'Astro ssr',
          include: ['**/!(*.browser).test.{ts,js,tsx,jsx,astro}'],
          environment: 'node'
        }
      }
    ],
    globals: true,
    coverage: {
      include: ['.astro', '.ts', '.tsx', '.svelte'],
      provider: 'v8',
      reporter: ['text', 'json-summary', 'json', 'cobertura']
    }
  }
})
