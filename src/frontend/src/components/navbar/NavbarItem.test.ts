import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import NavbarItem from './NavbarItem.astro'

test('Navbar Item internal', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(NavbarItem, {
    props: {
      id: 'home',
      href: '/',
      external: false
    },
    slots: {
      default: 'Home'
    }
  })

  expect(result).toContain('Home')
  expect(result).toContain('href="/')
  expect(result).toContain('_self')
})

test('Navbar Item internal', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(NavbarItem, {
    props: {
      id: 'home',
      href: '/',
      external: true
    },
    slots: {
      default: 'Home'
    }
  })

  expect(result).toContain('Home')
  expect(result).toContain('href="/')
  expect(result).toContain('_blank')
})
