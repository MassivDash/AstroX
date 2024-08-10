import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import Navbar from './Navbar.astro'

test('Navbar', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(Navbar)

  expect(result).toContain('astro')
  expect(result).toContain('X')
  expect(result).toContain('Actix')
})
