// spaceGame.test.ts
import { describe, it, expect, beforeAll, afterAll, vi } from 'vitest'
import Canvas from './spaceGame'
import { JSDOM } from 'jsdom'

describe('Canvas Class', () => {
  let canvasInstance: Canvas
  let dom: JSDOM

  beforeAll(() => {
    // Set up a mock HTML environment
    dom = new JSDOM(`
      <html>
        <body>
          <canvas id="canvas"></canvas>
          <div id="up" class="key up"></div>
          <div id="down" class="key down"></div>
          <div id="left" class="key left"></div>
          <div id="right" class="key right"></div>
        </body>
      </html>
    `)

    global.document = dom.window.document
    global.window = dom.window
    global.window.requestAnimationFrame = (callback) => {
      return setTimeout(callback, 0)
    }
    global.HTMLCanvasElement = dom.window.HTMLCanvasElement
    global.CanvasRenderingContext2D = dom.window.CanvasRenderingContext2D

    // Create an instance of the Canvas class
    canvasInstance = new Canvas()
  })

  afterAll(() => {
    // Clean up the mock HTML environment
    dom.window.close()
  })

  it('should initialize canvas correctly', () => {
    expect(canvasInstance.canvas).toBeInstanceOf(HTMLCanvasElement)
  })

  it('should set canvas size correctly', () => {
    canvasInstance.setCanvasSize()
    expect(canvasInstance.canvas.width).toBe(
      window.innerWidth * canvasInstance.dpr
    )
    expect(canvasInstance.canvas.height).toBe(500 * canvasInstance.dpr)
  })

  it('should handle arrow up events correctly', () => {
    const event = new dom.window.KeyboardEvent('keydown', { key: 'ArrowUp' })
    canvasInstance.handleKeydown(event)
    expect(canvasInstance.keys.up).toBe(true)
  })

  it('should handle arrow down events correctly', () => {
    const eventAcc = new dom.window.KeyboardEvent('keydown', {
      key: 'ArrowDown'
    })
    const eventDcc = new dom.window.KeyboardEvent('keydown', {
      key: 'ArrowUp'
    })

    const eventUp = new dom.window.KeyboardEvent('keyup', { key: 'ArrowDown' })
    canvasInstance.handleKeydown(eventAcc)
    expect(canvasInstance.keys.down).toBe(true)
    canvasInstance.updateRocket()
    canvasInstance.handleKeyup(eventUp)
    expect(canvasInstance.keys.down).toBe(false)
    canvasInstance.handleKeydown(eventDcc)
    canvasInstance.updateRocket()
    expect(canvasInstance.keys.up).toBe(true)
  })

  it('should handle arrow left events correctly', () => {
    const event = new dom.window.KeyboardEvent('keydown', { key: 'ArrowLeft' })
    canvasInstance.handleKeydown(event)
    expect(canvasInstance.keys.left).toBe(true)
    canvasInstance.updateRocket()
  })

  it('should handle arrow right events correctly', () => {
    const event = new dom.window.KeyboardEvent('keydown', { key: 'ArrowRight' })
    canvasInstance.handleKeydown(event)
    expect(canvasInstance.keys.right).toBe(true)
    canvasInstance.updateRocket()
  })

  it('should handle space event correctly', () => {
    const event = new dom.window.KeyboardEvent('keydown', { key: ' ' })
    canvasInstance.handleKeydown(event)
    expect(canvasInstance.keys.space).toBe(true)
  })

  it('should handle keyup events correctly for ArrowUp', () => {
    const event = new dom.window.KeyboardEvent('keyup', { key: 'ArrowUp' })
    canvasInstance.handleKeyup(event)
    expect(canvasInstance.keys.up).toBe(false)
  })

  it('should handle keyup events correctly for ArrowDown', () => {
    const event = new dom.window.KeyboardEvent('keyup', { key: 'ArrowDown' })
    canvasInstance.handleKeyup(event)
    expect(canvasInstance.keys.down).toBe(false)
  })

  it('should handle keyup events correctly for ArrowLeft', () => {
    const event = new dom.window.KeyboardEvent('keyup', { key: 'ArrowLeft' })
    canvasInstance.handleKeyup(event)
    expect(canvasInstance.keys.left).toBe(false)
  })

  it('should handle keyup events correctly for ArrowRight', () => {
    const event = new dom.window.KeyboardEvent('keyup', { key: 'ArrowRight' })
    canvasInstance.handleKeyup(event)
    expect(canvasInstance.keys.right).toBe(false)
  })

  it('should handle keyup events correctly for space', () => {
    const event = new dom.window.KeyboardEvent('keyup', { key: ' ' })
    canvasInstance.handleKeyup(event)
    expect(canvasInstance.keys.space).toBe(false)
  })

  it('should update rocket position correctly', () => {
    canvasInstance.keys.up = true
    canvasInstance.updateRocket()
    expect(canvasInstance.rocket.position.y).toBeLessThan(
      canvasInstance.canvas.height / 2 + 5
    )
  })

  it('should add particles correctly', () => {
    const initialParticleCount = canvasInstance.particles.length
    canvasInstance.addParticles(canvasInstance.rocket)
    expect(canvasInstance.particles.length).toBe(initialParticleCount + 1)
  })

  it('should draw background correctly', () => {
    const spy = vi.spyOn(canvasInstance.ctx, 'fillRect')
    canvasInstance.drawBG()
    expect(spy).toHaveBeenCalled()
  })

  it('should run demo correctly', () => {
    canvasInstance.demo1 = true
    canvasInstance.runDemo()
    expect(canvasInstance.rocket.velocity.x).not.toBe(0)
  })

  it('should run demo correctly', () => {
    canvasInstance.demo()
    expect(canvasInstance.rocket.velocity.x).not.toBe(0)
  })
})
