/*============================
 * Globals
 *===========================*/

const ROCKET_SIZE: number = 80
const POWER: number = 0.2
const PARTICLE_DECAY: number = 0.08
const PARTICLE_SIZE: number = 14

/*============================
 * Canvas
 *===========================*/

export default class Canvas {
  public canvas: HTMLCanvasElement
  public dpr: number
  public ctx: CanvasRenderingContext2D
  public upEl: HTMLElement
  public downEl: HTMLElement
  public leftEl: HTMLElement
  public rightEl: HTMLElement
  public keys!: {
    space: boolean
    up: boolean
    down: boolean
    right: boolean
    left: boolean
  }
  public particles!: Particle[]
  public rocket!: Rocket
  public demo1!: boolean
  public timerCanvas: ReturnType<typeof setTimeout>
  public demo2!: boolean

  constructor() {
    // setup a canvas
    this.canvas = document.getElementById('canvas') as HTMLCanvasElement
    this.dpr = window.devicePixelRatio || 1
    this.ctx = this.canvas?.getContext('2d') as CanvasRenderingContext2D
    this.ctx?.scale(this.dpr, this.dpr)

    this.upEl = document.getElementById('up')!
    this.downEl = document.getElementById('down')!
    this.leftEl = document.getElementById('left')!
    this.rightEl = document.getElementById('right')!

    this.setCanvasSize = this.setCanvasSize.bind(this)
    this.handleKeydown = this.handleKeydown.bind(this)
    this.handleKeyup = this.handleKeyup.bind(this)
    this.loop = this.loop.bind(this)
    this.timerCanvas = setTimeout(() => {}, 0)

    this.init()
  }

  init() {
    if (this.canvas) {
      this.setCanvasSize()
      this.setupListeners()
      this.setupKeys()
      this.particles = []
      this.rocket = new Rocket(
        this.canvas?.width / 2,
        this.canvas?.height / 2,
        ROCKET_SIZE
      )

      this.loop()
    }
  }

  demo() {
    this.demo1 = true
    this.demo2 = false
    this.upEl.className = 'key up is-down'
    this.rightEl.className = 'key right is-down'
    this.timerCanvas = setTimeout(() => {
      this.demo1 = false
      this.demo2 = true
      this.rightEl.className = 'key right'
      this.leftEl.className = 'key left is-down'

      this.timerCanvas = setTimeout(() => {
        this.demo2 = false
        this.upEl.className = 'key up'
        this.leftEl.className = 'key left'
      }, 1000)
    }, 1300)
  }

  setCanvasSize() {
    this.canvas.width = window.innerWidth * this.dpr
    this.canvas.height = 500 * this.dpr
    this.canvas.style.width = window.innerWidth + 'px'
    this.canvas.style.height = window.innerHeight + 'px'
  }

  setupListeners() {
    window.addEventListener('resize', this.setCanvasSize)
    window.addEventListener('keydown', this.handleKeydown)
    window.addEventListener('keyup', this.handleKeyup)
  }

  stopListeners() {
    window.removeEventListener('resize', this.setCanvasSize)
    window.removeEventListener('keydown', this.handleKeydown)
    window.removeEventListener('keyup', this.handleKeyup)
  }

  setupKeys() {
    this.keys = {
      space: false,
      up: false,
      down: false,
      right: false,
      left: false
    }
  }

  handleKeydown(event: KeyboardEvent) {
    event.preventDefault()
    // key event
    switch (event.key) {
      case ' ':
        this.keys.space = true
        break
      case 'ArrowDown':
        this.keys.down = true
        break
      case 'ArrowRight':
        this.keys.right = true
        break
      case 'ArrowUp':
        this.keys.up = true
        break
      case 'ArrowLeft':
        this.keys.left = true
        break
    }
  }

  handleKeyup(event: KeyboardEvent) {
    event.preventDefault()
    this.leftEl.className = 'key left'
    this.rightEl.className = 'key right'
    this.upEl.className = 'key up'
    this.downEl.className = 'key down'

    // key events
    switch (event.key) {
      case ' ':
        this.keys.space = false
        break
      case 'ArrowDown':
        this.keys.down = false
        break
      case 'ArrowRight':
        this.keys.right = false
        break
      case 'ArrowUp':
        this.keys.up = false
        break
      case 'ArrowLeft':
        this.keys.left = false
        break
    }
  }

  updateRocket() {
    if (this.keys.right) {
      this.rightEl.className = 'key right is-down'
      this.rocket.rotateClockwise()
    }
    if (this.keys.left) {
      this.leftEl.className = 'key left is-down'
      this.rocket.rotateCounterClockwise()
    }
    if (this.keys.up) {
      this.upEl.className = 'key up is-down'
      this.addParticles(this.rocket)
      this.rocket.accelerate()
    }
    if (this.keys.down) {
      this.downEl.className = 'key down is-down'
      this.addParticles(this.rocket)
      this.rocket.decelerate()
    }
    this.rocket.updatePosition()
    this.positionCheck(this.rocket)
    this.rocket.draw()
  }

  positionCheck(object: {
    canvasSize?: number
    position: { x: number; y: number }
  }) {
    const size = object.canvasSize || this.rocket.canvasSize
    const { position } = object

    if (position.x < -size) {
      object.position.x = this.canvas.width
    }
    if (position.y < -size) {
      object.position.y = this.canvas.height
    }
    if (position.x > this.canvas.width) {
      object.position.x = -size
    }
    if (position.y > this.canvas.height) {
      object.position.y = -size
    }
  }

  addParticles(object: {
    position: { x: number; y: number }
    velocity: { x: number; y: number }
    size: number
  }) {
    let { x, y } = object.position
    x = x + object.size / 2
    y = y + object.size / 2
    let { x: vx, y: vy } = object.velocity
    // eslint-disable-next-line no-self-assign
    vx = vx
    // eslint-disable-next-line no-self-assign
    vy = vy

    const particle = new Particle(x, y, vx, vy, PARTICLE_SIZE)
    this.particles.push(particle)
  }

  drawParticles() {
    if (!this.particles.length) return
    this.particles.forEach((particle, i) => {
      this.positionCheck(particle)
      particle.updatePosition()
      particle.updateSize()
      particle.draw(this.ctx)
      if (particle.dead) this.particles.splice(i, 1)
    })
  }

  drawRocket() {
    this.ctx?.drawImage(
      this.rocket.canvas,
      this.rocket.position.x,
      this.rocket.position.y,
      this.rocket.width,
      this.rocket.height
    )
  }

  drawBG() {
    const gradient = this.ctx.createLinearGradient(
      0,
      this.canvas.height / 2,
      this.canvas.width,
      this.canvas.height / 2
    )
    gradient.addColorStop(0, '#efefef')
    gradient.addColorStop(1, '#e7e7e7')
    this.ctx.fillStyle = gradient
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height)
  }

  runDemo() {
    if (this.demo1) {
      this.addParticles(this.rocket)
      this.rocket.rotateClockwise()
      this.rocket.accelerate()
    }

    if (this.demo2) {
      this.addParticles(this.rocket)
      this.rocket.rotateCounterClockwise()
      this.rocket.accelerate()
    }
  }

  loop() {
    window.requestAnimationFrame(this.loop)
    this.runDemo()
    this.drawBG()
    this.updateRocket()
    this.drawParticles()
    this.drawRocket()
  }
}

/*============================
 * Particle Class
 *===========================*/

class Particle {
  public velocity: { x: number; y: number }
  public position: { x: number; y: number }
  public size: number
  public width: number
  public height: number
  public decay: number
  public dead: boolean

  constructor(x: number, y: number, vx: number, vy: number, size: number) {
    this.velocity = {
      x: vx,
      y: vy
    }
    this.position = {
      x,
      y
    }
    this.size = size
    this.width = size
    this.height = size
    this.decay = PARTICLE_DECAY
    this.dead = false
  }

  updateSize() {
    this.size -= this.decay
    if (this.size <= 0) {
      this.dead = true
    }
  }

  updatePosition() {
    this.position.x += this.velocity.x
    this.position.y += this.velocity.y
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = '#262626'
    ctx.fillRect(
      this.position.x - this.size / 2,
      this.position.y - this.size / 2,
      this.size,
      this.size
    )
  }
}

/*============================
 * Rocket Class
 *===========================*/

class Rocket {
  public power: number
  public brake: number
  public velocity: { x: number; y: number }
  public position: { x: number; y: number }
  public size: number
  public width: number
  public height: number
  public rotation: number
  public canvasSize: number
  public center: number
  public ctx: CanvasRenderingContext2D
  public canvas: HTMLCanvasElement

  constructor(x: number, y: number, size: number) {
    this.power = POWER
    this.brake = -POWER
    this.velocity = {
      x: 0,
      y: 0
    }
    this.position = {
      x,
      y
    }
    this.size = size
    this.width = size
    this.height = size
    this.rotation = 0

    this.canvasSize = this.size * 2
    this.center = this.canvasSize / 2

    this.ctx = document.createElement('canvas').getContext('2d')!
    this.canvas = this.ctx.canvas

    this.canvas.width = this.canvasSize
    this.canvas.height = this.canvasSize

    this.draw()
  }

  accelerate() {
    const degOffset = Math.PI / 2
    this.velocity = movePointAtAngle(
      this.velocity,
      (this.rotation / 180) * Math.PI + degOffset,
      this.power
    )
  }

  decelerate() {
    const degOffset = Math.PI / 2
    this.velocity = movePointAtAngle(
      this.velocity,
      (this.rotation / 180) * Math.PI + degOffset,
      this.brake
    )
  }

  updatePosition() {
    this.position.x += this.velocity.x
    this.position.y += this.velocity.y
  }

  rotateCounterClockwise() {
    if (this.rotation <= 0) {
      this.rotation = 360
    } else {
      this.rotation -= 5
    }
  }

  rotateClockwise() {
    if (this.rotation >= 360) {
      this.rotation = 0
    } else {
      this.rotation += 5
    }
  }

  drawShip() {
    const offset = this.center / 2
    this.ctx.strokeStyle = '#262626'
    this.ctx.lineWidth = this.size / 9
    this.ctx.beginPath()
    this.ctx.moveTo(this.size / 2 - offset, -offset * 1.7)
    this.ctx.lineTo(this.size - offset, this.size - offset)
    this.ctx.lineTo(0 - offset, this.size - offset)
    this.ctx.fill()
    this.ctx.closePath()
    this.ctx.stroke()
  }

  draw() {
    this.ctx.fillStyle = '#262626'
    this.ctx.clearRect(0, 0, this.canvasSize, this.canvasSize)

    // ctx transforms
    this.ctx.save()
    this.ctx.translate(this.center, this.center)
    this.ctx.rotate((this.rotation / 180) * Math.PI)

    this.drawShip()

    // reset translate/rotation
    this.ctx.restore()
  }
}

function movePointAtAngle(
  point: { x: number; y: number },
  angle: number,
  distance: number
) {
  return {
    x: point.x - Math.cos(angle) * distance,
    y: point.y - Math.sin(angle) * distance
  }
}
