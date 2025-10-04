import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
// @eslint-disable-next-line import/no-unresolved
import ssr from '@astrojs/react/server.js'

import { ProtectedIsland } from './protectedIsland.tsx'
import type { AstroComponentFactory } from 'astro/runtime/server/index.js'

test('ProtectedIsland Astro Container test', async () => {
  const container = await AstroContainer.create()
  container.addServerRenderer({
    name: '@astrojs/react',
    renderer: ssr
  })
  container.addClientRenderer({
    name: '@astrojs/react',
    entrypoint: '@astrojs/react/client-v17.js'
  })
  const result = await container.renderToString(
    ProtectedIsland as unknown as AstroComponentFactory
  )
  expect(result).toContain('Protected React Island')
  expect(result).toContain('Loading protected data')
})
