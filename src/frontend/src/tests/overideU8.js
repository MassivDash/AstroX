// Only overwrite this if we are in test mode. This is an env var we happen to have in our repo
if (process.env.VITE_IS_TEST === 'true') {
  class ESBuildAndJSDOMCompatibleTextEncoder extends TextEncoder {
    constructor() {
      super()
    }

    encode(input) {
      if (typeof input !== 'string') {
        throw new TypeError('`input` must be a string')
      }

      const decodedURI = decodeURIComponent(encodeURIComponent(input))
      const arr = new Uint8Array(decodedURI.length)
      const chars = decodedURI.split('')
      for (let i = 0; i < chars.length; i++) {
        arr[i] = decodedURI[i].charCodeAt(0)
      }
      return arr
    }
  }

  Object.defineProperty(global, 'TextEncoder', {
    value: ESBuildAndJSDOMCompatibleTextEncoder,
    writable: true
  })
}
