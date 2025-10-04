/**
 * @vitest-environment jsdom
 */

import { ProtectedIsland } from './protectedIsland.tsx'
import { expect, test, vi } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import { axiosBackendInstance } from '@axios/axiosBackendInstance'

vi.mock('@axios/axiosBackendInstance', () => ({
  axiosBackendInstance: {
    get: vi.fn()
  }
}))

test('ProtectedIsland - successful authentication', async () => {
  vi.mocked(axiosBackendInstance.get).mockResolvedValueOnce({
    data: {
      message: 'Welcome! This is protected data.',
      status: 'authenticated'
    }
  } as any)

  render(<ProtectedIsland />)

  // Check if the loading text is displayed initially
  expect(screen.getByText('Loading protected data...')).toBeTruthy()

  // Wait for the data to be fetched and displayed
  await waitFor(() => {
    expect(screen.getByText('Welcome! This is protected data.')).toBeTruthy()
    expect(screen.getByText('authenticated')).toBeTruthy()
  })
})

test('ProtectedIsland - unauthorized access', async () => {
  vi.mocked(axiosBackendInstance.get).mockRejectedValueOnce({
    response: {
      data: {
        error: 'Unauthorized',
        message: 'You must be logged in to access this resource'
      }
    }
  })

  render(<ProtectedIsland />)

  // Check if the loading text is displayed initially
  expect(screen.getByText('Loading protected data...')).toBeTruthy()

  // Wait for the error to be displayed
  await waitFor(() => {
    expect(screen.getByText('Unauthorized')).toBeTruthy()
    expect(
      screen.getByText('You must be logged in to access this resource')
    ).toBeTruthy()
    expect(
      screen.getByText('Please login to access this resource.')
    ).toBeTruthy()
  })
})
