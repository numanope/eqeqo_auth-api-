import './product-card.js';

const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
    }
    .grid-container {
      /* Using global grid utility classes from main.css */
    }
    .no-results {
      padding: var(--spacing-xl);
      text-align: center;
      background-color: var(--color-muted);
      border-radius: var(--radius-lg);
    }
  </style>
  <div class="grid grid-cols grid-cols-md-2 grid-cols-lg-3 gap-lg" id="grid-container">
    <!-- Product cards will be rendered here -->
  </div>
  <div id="no-results" class="no-results" style="display: none;">
    <h3>No se encontraron productos</h3>
    <p>Intenta ajustar tus filtros de búsqueda.</p>
  </div>
`;

class ProductGrid extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  set products(productsData) {
    const gridContainer = this.shadowRoot.getElementById('grid-container');
    const noResultsEl = this.shadowRoot.getElementById('no-results');

    gridContainer.innerHTML = ''; // Clear previous products

    if (!productsData || productsData.length === 0) {
      noResultsEl.style.display = 'block';
      gridContainer.style.display = 'none';
      return;
    }

    noResultsEl.style.display = 'none';
    gridContainer.style.display = 'grid';

    productsData.forEach(product => {
      const productCard = document.createElement('product-card');
      productCard.product = product;
      gridContainer.appendChild(productCard);
    });
  }
}

customElements.define('product-grid', ProductGrid);