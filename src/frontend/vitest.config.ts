import { getViteConfig } from 'astro/config'
import { svelteTesting } from '@testing-library/svelte/vite'

export default getViteConfig({
  resolve: {
    tsconfigPaths: true
  },
  test: {
    projects: [
      {
        plugins: [svelteTesting()],
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
        test: {
          name: 'Astro ssr',
          include: ['**/!(*.browser).test.{ts,js,tsx,jsx,astro}'],
          environment: 'node'
        }
      }
    ],
    globals: true,
    coverage: {
      include: ['src/**/*.{ts,tsx,svelte}'],
      exclude: ['**/*.test.*', '**/*.d.ts'],
      provider: 'v8',
      reporter: ['text', 'json-summary', 'json', 'cobertura']
    }
  }
} as any)
