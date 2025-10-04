import { useEffect, useMemo, useState } from 'react'
import { axiosBackendInstance } from '@axios/axiosBackendInstance'

interface ProtectedData {
  message: string
  status: string
}

interface ErrorData {
  error: string
  message: string
}

export const ProtectedIsland = () => {
  const [data, setData] = useState<ProtectedData | null>(null)
  const [error, setError] = useState<ErrorData | null>(null)
  const [loading, setLoading] = useState<boolean>(true)

  useEffect(() => {
    axiosBackendInstance
      .get('/protected')
      .then((response) => {
        setData(response.data)
        setError(null)
        setLoading(false)
      })
      .catch((err) => {
        if (err.response && err.response.data) {
          setError(err.response.data)
        } else {
          setError({
            error: 'Network Error',
            message: 'Failed to fetch protected data'
          })
        }
        setData(null)
        setLoading(false)
      })
  }, [])

  const borderColor = useMemo(() => {
    return data ? '#4CAF50' : '#f44336'
  }, [data])

  return (
    <div
      style={{
        padding: '20px',
        border: `2px solid ${borderColor}`,
        borderRadius: '8px',
        margin: '20px 0'
      }}
    >
      <h3>🔒 Protected React Island</h3>
      {loading && <p>Loading protected data...</p>}
      {data && (
        <div style={{ color: borderColor }}>
          <p>
            <strong>Status:</strong> {data.status}
          </p>
          <p>
            <strong>Message:</strong> {data.message}
          </p>
        </div>
      )}
      {error && (
        <div style={{ color: borderColor }}>
          <p>
            <strong>Error:</strong> {error.error}
          </p>
          <p>
            <strong>Message:</strong> {error.message}
          </p>
          <p>
            <em>Please login to access this resource.</em>
          </p>
        </div>
      )}
    </div>
  )
}
