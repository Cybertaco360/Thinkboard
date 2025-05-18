// Node management service to extract logic from App.vue

import { reactive } from 'vue';

// Shared node store
const nodes = reactive([]);
const nodeHistory = reactive({
  past: [],
  future: []
});

/**
 * Validate and normalize node data from API
 */
function validateNodeSchema(data) {
  // Check if data is already an array or if it needs conversion
  const nodeArray = Array.isArray(data) ? data : [data];
  
  // Handle edge case where we might have gotten a string instead of objects
  if (nodeArray.length > 0 && typeof nodeArray[0] === 'string') {
    console.error('Received string array instead of node objects:', nodeArray);
    return [{
      node_id: Date.now(),
      x: 100,
      y: 100,
      text: 'Error: Received text instead of nodes',
      connected: [],
      information: 'The API returned text instead of properly formatted nodes: ' + 
                  (nodeArray.length > 20 
                   ? nodeArray.slice(0, 20).join('') + '...' 
                   : nodeArray.join('')),
      category: 3,
      createdAt: new Date()
    }];
  }
  
  return nodeArray.map((node, index) => ({
    node_id: node.node_id || Date.now() + index,
    x: node.x || 100,
    y: node.y || (100 + index * 150),
    text: node.text || 'Unknown',
    connected: Array.isArray(node.connected) ? node.connected : [],
    information: node.information || '',
    category: node.category || 1,
    createdAt: node.createdAt || new Date()
  }));
}

/**
 * Save current state for undo/redo
 */
function saveState() {
  nodeHistory.past.push(JSON.stringify(nodes));
  nodeHistory.future = []; // Clear redo stack on new action
  
  // Limit history size
  if (nodeHistory.past.length > 50) {
    nodeHistory.past.shift();
  }
}

/**
 * Add new nodes from API to the node store
 */
function addNodes(newNodes) {
  saveState();
  const validatedNodes = validateNodeSchema(newNodes);
  nodes.push(...validatedNodes);
  return validatedNodes;
}

/**
 * Create a new node
 */
function createNode(x, y, zoomLevel, panOffset, categoryId = 1) {
  saveState();
  
  // Adjust coordinates for zoom and pan
  const adjustedX = (x - panOffset.x) / zoomLevel;
  const adjustedY = (y - panOffset.y) / zoomLevel;
  
  const newNode = {
    node_id: Date.now(),
    x: adjustedX,
    y: adjustedY,
    text: 'New Node',
    information: 'Click to edit this node',
    connected: [],
    category: categoryId,
    createdAt: new Date()
  };
  
  nodes.push(newNode);
  return newNode;
}

/**
 * Delete nodes by ID
 */
function deleteNodes(nodeIds) {
  if (nodeIds.length === 0) return;
  
  saveState();
  
  const remainingNodes = nodes.filter(node => !nodeIds.includes(node.node_id));
  nodes.splice(0, nodes.length, ...remainingNodes);
}

/**
 * Create connections between nodes
 */
function connectNodes(nodeIds) {
  if (nodeIds.length < 2) return;
  
  saveState();
  
  // Create connections between selected nodes in order
  for (let i = 0; i < nodeIds.length - 1; i++) {
    const currentNodeId = nodeIds[i];
    const nextNodeId = nodeIds[i + 1];
    
    const currentNode = nodes.find(node => node.node_id === currentNodeId);
    if (currentNode && !currentNode.connected.includes(nextNodeId)) {
      currentNode.connected.push(nextNodeId);
    }
  }
}

/**
 * Remove connections between nodes
 */
function disconnectNodes(nodeIds) {
  if (nodeIds.length < 2) return;
  
  saveState();
  
  // Remove connections between all selected nodes
  for (let i = 0; i < nodes.length; i++) {
    const node = nodes[i];
    if (nodeIds.includes(node.node_id)) {
      // Filter out connections to other selected nodes
      node.connected = node.connected.filter(id => !nodeIds.includes(id));
    } else {
      // Also remove connections from non-selected nodes to selected nodes
      node.connected = node.connected.filter(id => !nodeIds.includes(id));
    }
  }
}

/**
 * Undo last action
 */
function undo() {
  if (nodeHistory.past.length === 0) return;
  
  const current = JSON.stringify(nodes);
  nodeHistory.future.push(current);
  
  const previous = nodeHistory.past.pop();
  const previousState = JSON.parse(previous);
  
  nodes.splice(0, nodes.length, ...previousState);
}

/**
 * Redo last undone action
 */
function redo() {
  if (nodeHistory.future.length === 0) return;
  
  const current = JSON.stringify(nodes);
  nodeHistory.past.push(current);
  
  const next = nodeHistory.future.pop();
  const nextState = JSON.parse(next);
  
  nodes.splice(0, nodes.length, ...nextState);
}

/**
 * Add sample nodes for demonstration
 */
function addSampleNodes() {
  saveState();
  const sampleNodes = [
    {
      node_id: 1,
      x: 300,
      y: 100,
      text: 'Project Start',
      information: 'This is the beginning of our project journey',
      connected: [2],
      category: 2,
      createdAt: new Date()
    },
    {
      node_id: 2,
      x: 300,
      y: 300,
      text: 'Research Phase',
      information: 'Collect and analyze requirements',
      connected: [3],
      category: 1,
      createdAt: new Date()
    },
    {
      node_id: 3,
      x: 300,
      y: 500,
      text: 'Design Decision',
      information: 'Should we use Vue or React?',
      connected: [4],
      category: 3,
      createdAt: new Date()
    },
    {
      node_id: 4,
      x: 300,
      y: 700,
      text: 'Implementation',
      information: 'Coding begins now',
      connected: [],
      category: 1,
      createdAt: new Date()
    }
  ];
  
  nodes.splice(0, nodes.length, ...sampleNodes);
}

export default {
  nodes,
  nodeHistory,
  validateNodeSchema,
  saveState,
  addNodes,
  createNode,
  deleteNodes,
  connectNodes,
  disconnectNodes,  // Add the new method
  undo,
  redo,
  addSampleNodes
};
