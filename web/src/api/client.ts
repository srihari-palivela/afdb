import axios from 'axios';
import { useSession } from '../store/session';

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
});

api.interceptors.request.use((config) => {
  try {
    const { sessionId } = useSession.getState();
    if (sessionId) {
      config.headers = config.headers ?? {};
      (config.headers as any)['X-Session-Id'] = sessionId;
    }
  } catch {}
  return config;
});

api.interceptors.response.use(
  (r) => r,
  (err) => {
    // Basic normalization of errors
    const message = err?.response?.data?.message || err?.message || 'Request failed';
    return Promise.reject(new Error(message));
  }
);
