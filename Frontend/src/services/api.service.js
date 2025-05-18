// API service for centralized backend communication

const API_BASE_URL = 'http://localhost:8080';

/**
 * Generic API request handler with error handling
 */
async function apiRequest(endpoint, options = {}) {
  try {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers
      },
      ...options
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.message || `API error: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    console.error(`API request failed: ${error.message}`);
    throw error;
  }
}

/**
 * Generate nodes from prompt
 */
export async function generateNodesFromPrompt(prompt) {
  return apiRequest('/generate', {
    method: 'POST',
    body: JSON.stringify({ prompt })
  });
}

/**
 * User authentication
 */
export async function login(email, password) {
  return apiRequest('/login', {
    method: 'POST',
    body: JSON.stringify({ email, password })
  });
}

/**
 * User registration
 */
export async function signup(name, email, password) {
  return apiRequest('/signup', {
    method: 'POST',
    body: JSON.stringify({ name, email, password })
  });
}

export default {
  generateNodesFromPrompt,
  login,
  signup
};
