import axios from 'axios'

export const axiosBackendInstance = axios.create({
  baseURL: import.meta.env.PUBLIC_API_URL
})
