import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import ssr from '@astrojs/svelte/server.js'

import SpaceX from './spacex.svelte'
import type { AstroComponentFactory } from 'astro/runtime/server/index.js'

test('Svelte Island', async () => {
  const container = await AstroContainer.create()
  container.addServerRenderer({
    name: '@astrojs/svelte',
    renderer: ssr
  })
  container.addClientRenderer({
    name: '@astrojs/svelte',
    entrypoint: '@astrojs/svelte/client-v5.js'
  })
  const result = await container.renderToString(
    SpaceX as unknown as AstroComponentFactory
  ) // This works but astro still have not pinned down the types correctly
  expect(result).toContain('Svelte Component (Client side call example)')
})
