import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import Layout from './Layout.astro'

test('Card with slots', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(Layout, {
    props: {
      title: 'Layout Title'
    },
    slots: {
      default: 'Layout content'
    }
  })

  expect(result).toContain('Layout content')
  expect(result).toContain('Layout Title')
})
