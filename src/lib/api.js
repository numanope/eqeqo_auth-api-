// DTO types are removed as they are for development guidance and not used at runtime.

const API_BASE_URL = 'src/db.json'; // In a real app, this would be an absolute URL to the backend.

/**
 * Fetches data from the local JSON file and selects a specific key.
 * @param {string} key The key to select from the root of the JSON file.
 * @returns The data corresponding to the key.
 */
async function fetchFromJson(key) {
  try {
    const response = await fetch(API_BASE_URL);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    if (!data[key]) {
      throw new Error(`Key "${key}" not found in db.json`);
    }
    return data[key];
  } catch (error) {
    console.error('Failed to fetch API data:', error);
    throw error;
  }
}

// --- GET Endpoints ---

/**
 * GET /api/home
 * Fetches the content for the home page.
 */
export const getHomeData = () => fetchFromJson('home');

/**
 * GET /api/products
 * Fetches the list of all products and available tags.
 */
export const getProductsData = () => fetchFromJson('products');

/**
 * GET /api/filters
 * Fetches the available filters for the store page.
 */
export const getFiltersData = () => fetchFromJson('filters');

/**
 * GET /api/contact
 * Fetches the content for the contact page.
 */
export const getContactData = () => fetchFromJson('contact');


// --- PUT Endpoints (Simulated) ---

/**
 * PUT /api/home
 * Updates the home page content.
 * (In this simulation, it just logs the payload to the console).
 * @param {object} payload The data to update.
 */
export const updateHomeData = async (payload) => {
  console.log('--- SIMULATED API CALL ---');
  console.log('Endpoint: PUT /api/home');
  console.log('Payload:', payload);
  console.log('--------------------------');
  // In a real implementation, this would be:
  // await fetch('/api/home', { method: 'PUT', body: JSON.stringify(payload), headers: {'Content-Type': 'application/json'} });
};

/**
 * PUT /api/products/:id
 * Updates a specific product.
 * (In this simulation, it just logs the payload to the console).
 * @param {string} id The ID of the product to update.
 * @param {object} payload The data to update.
 */
export const updateProductData = async (id, payload) => {
  console.log('--- SIMULATED API CALL ---');
  console.log(`Endpoint: PUT /api/products/${id}`);
  console.log('Payload:', payload);
  console.log('--------------------------');
};

/**
 * PUT /api/contact
 * Updates the contact page content.
 * (In this simulation, it just logs the payload to the console).
 * @param {object} payload The data to update.
 */
export const updateContactData = async (payload) => {
    console.log('--- SIMULATED API CALL ---');
    console.log('Endpoint: PUT /api/contact');
    console.log('Payload:', payload);
    console.log('--------------------------');
};