import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import IndexPage from '../pages/index.astro'
import Page404 from '../pages/404.astro'
import CliPage from '../pages/cli.astro'
import ActixPage from '../pages/actix.astro'
import ProtectedPage from '../pages/auth/protected.astro'
import AstroPage from '../pages/astro.astro'
import { getContainerRenderer } from '@astrojs/svelte'
import ssr from '@astrojs/svelte/server.js'

console.log(ssr)
test('Index Page', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(IndexPage)
  expect(result).toContain('🦀 Rust orientated monolithic')
})

test('404 Page', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(Page404)
  expect(result).toContain('404')
})

test('Cli Page', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(CliPage)
  expect(result).toContain('Runner')
})

test('Actix Page', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(ActixPage)
  expect(result).toContain('ActiX')
})

test('Astro Page', async () => {
  const container = await AstroContainer.create()
  container.addServerRenderer({
    name: '@astrojs/svelte',
    renderer: ssr //wrong types, but this combo works
  })
  container.addClientRenderer({
    name: '@astrojs/svelte',
    entrypoint: '@astrojs/svelte/client-v5.js'
  })
  const result = await container.renderToString(AstroPage)
  expect(result).toContain('Astro framework frontend')
})

test('Protected Page', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(ProtectedPage)
  expect(result).toContain('Protected')
})
