import { initializeTheme } from '../lib/theme.js';
import { getProductsData, getFiltersData, getHomeData } from '../lib/api.js';
import '../components/app-shell.js';
import '../components/filter-bar.js';
import '../components/product-grid.js';

// Global state for the store page
let allProducts = [];

// Initialize the theme first
initializeTheme();

// Main function to set up and render the store page
async function renderStorePage() {
  const layoutContainer = document.getElementById('store-layout');
  const appShell = document.querySelector('app-shell');

  if (!layoutContainer || !appShell) {
    console.error('Required elements #store-layout or app-shell not found.');
    return;
  }

  try {
    // Fetch all necessary data in parallel
    const [productsData, filtersData, homeData] = await Promise.all([
      getProductsData(),
      getFiltersData(),
      getHomeData() // For footer data
    ]);
    allProducts = productsData.items;

    // Create the store layout (sidebar for filters, main for grid)
    layoutContainer.innerHTML = `
      <style>
        .store-grid-layout {
          display: grid;
          grid-template-columns: 1fr;
          gap: var(--spacing-lg);
        }
        @media (min-width: 768px) {
          .store-grid-layout {
            grid-template-columns: 250px 1fr;
          }
        }
      </style>
      <div class="store-grid-layout">
        <aside id="filters-container"></aside>
        <main id="products-container"></main>
      </div>
    `;

    // 1. Setup Filter Bar
    const filterBar = document.createElement('filter-bar');
    filterBar.filters = filtersData.items;
    document.getElementById('filters-container').appendChild(filterBar);

    // 2. Setup Product Grid
    const productGrid = document.createElement('product-grid');
    document.getElementById('products-container').appendChild(productGrid);

    // Initial render of all products
    productGrid.products = allProducts;

    // 3. Add event listener for filtering
    filterBar.addEventListener('filter-changed', (event) => {
      const { selectedTags } = event.detail;
      const filteredProducts = filterProducts(selectedTags);
      productGrid.products = filteredProducts;
    });

    // 4. Populate footer
    appShell.footerData = {
        text: homeData.footer_text_md,
        links: homeData.social_links,
    };

  } catch (error) {
    layoutContainer.innerHTML = `<h2>Error al cargar la tienda</h2><p>No se pudieron obtener los productos. Inténtalo de nuevo más tarde.</p>`;
    console.error('Failed to render store page:', error);
  }
}

/**
 * Filters products based on an array of selected tags.
 * @param {string[]} selectedTags - The tags to filter by.
 * @returns {object[]} The filtered list of products.
 */
function filterProducts(selectedTags) {
  if (selectedTags.length === 0) {
    return allProducts;
  }
  return allProducts.filter(product =>
    selectedTags.every(tag => product.tags.includes(tag))
  );
}

// Execute the render function once the DOM is loaded
document.addEventListener('DOMContentLoaded', renderStorePage);