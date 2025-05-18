/**
 * @typedef {Object} Node
 * @property {number|string} node_id - Unique identifier for the node
 * @property {number} x - X coordinate
 * @property {number} y - Y coordinate
 * @property {string} text - Text displayed on the node
 * @property {Array<number|string>} connected - IDs of connected nodes
 * @property {string} information - Additional information about the node
 * @property {number} category - Category ID
 * @property {Date} createdAt - Creation timestamp
 */

/**
 * @typedef {Object} NodeCategory
 * @property {number} id - Category identifier
 * @property {string} name - Category name
 * @property {string} color - Hex color code
 */

/**
 * @typedef {Object} User
 * @property {string} id - User ID
 * @property {string} name - User's full name
 * @property {string} email - User's email
 */

/**
 * @typedef {Object} AuthResponse
 * @property {boolean} success - Whether authentication was successful
 * @property {string} message - Response message
 * @property {string} [user_id] - User ID if successful
 */

// Export for documentation purposes - not required for Vue 3
export const Types = {
  Node: {},
  NodeCategory: {},
  User: {},
  AuthResponse: {}
};
