import { axiosBackendInstance } from '../axiosInstance/axiosBackendInstance'
import { describe, it, expect } from 'vitest'
describe('axiosBackendInstance', () => {
  it('should have the correct baseURL', () => {
    expect(axiosBackendInstance.defaults.baseURL).toBe(
      import.meta.env.PUBLIC_API_URL
    )
  })
})
