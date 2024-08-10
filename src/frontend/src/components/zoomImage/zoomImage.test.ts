import { experimental_AstroContainer as AstroContainer } from 'astro/container'
import { expect, test } from 'vitest'
import AstroImage from '../../sections/imgs/astro.jpeg'
import ZoomImage from './zoomImage.astro'

test('ZoomImage', async () => {
  const container = await AstroContainer.create()
  const result = await container.renderToString(ZoomImage, {
    props: {
      src: AstroImage,
      alt: 'An example image',
      borderFlat: 'left'
    }
  })

  expect(result).toContain('An example image')
})
