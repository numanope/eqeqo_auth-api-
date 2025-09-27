import {
  getHomeData,
  getProductsData,
  getContactData,
  updateHomeData,
  updateProductData,
  updateContactData
} from '../lib/api.js';

const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
      margin-top: var(--spacing-lg);
    }
    .editor-section {
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      margin-bottom: var(--spacing-lg);
      background-color: var(--color-background);
    }
    .editor-section summary {
      font-size: 1.25rem;
      font-weight: 600;
      padding: var(--spacing-md) var(--spacing-lg);
      cursor: pointer;
      list-style: none; /* Remove default marker */
    }
    .editor-section summary::-webkit-details-marker {
      display: none; /* Hide for Safari */
    }
    .editor-section[open] summary {
      border-bottom: 1px solid var(--color-border);
    }
    .form-container {
      padding: var(--spacing-lg);
    }
    .form-group {
      margin-bottom: var(--spacing-lg);
    }
    label {
      display: block;
      font-weight: 500;
      margin-bottom: var(--spacing-sm);
    }
    textarea {
      min-height: 120px;
      resize: vertical;
    }
    .product-selector {
        margin-bottom: var(--spacing-lg);
    }
  </style>

  <form id="home-form">
    <details class="editor-section" open>
      <summary>Página de Inicio</summary>
      <div class="form-container">
        <div class="form-group">
          <label for="home-title">Título del Hero</label>
          <input type="text" id="home-title" name="title" required maxlength="80">
        </div>
        <div class="form-group">
          <label for="home-subtitle">Subtítulo del Hero</label>
          <input type="text" id="home-subtitle" name="subtitle" required maxlength="160">
        </div>
        <div id="home-blocks"></div>
        <div class="form-group">
            <label for="home-footer">Texto del Footer (Markdown)</label>
            <textarea id="home-footer" name="footer_text_md"></textarea>
        </div>
        <button type="submit" class="btn btn-primary">Guardar Cambios de Inicio</button>
      </div>
    </details>
  </form>

  <form id="products-form">
    <details class="editor-section">
      <summary>Productos</summary>
      <div class="form-container">
        <div class="form-group product-selector">
          <label for="product-select">Seleccionar Producto a Editar</label>
          <select id="product-select" class="input"></select>
        </div>
        <div class="form-group">
          <label for="product-name">Nombre del Producto</label>
          <input type="text" id="product-name" name="name" required>
        </div>
        <div class="form-group">
          <label for="product-price">Precio</label>
          <input type="number" id="product-price" name="price" required min="0" step="0.01">
        </div>
        <div class="form-group">
          <label for="product-stock">Stock</label>
          <input type="number" id="product-stock" name="stock" required min="0">
        </div>
        <button type="submit" class="btn btn-primary">Guardar Cambios de Producto</button>
      </div>
    </details>
  </form>

  <form id="contact-form">
    <details class="editor-section">
        <summary>Página de Contacto</summary>
        <div class="form-container">
            <div class="form-group">
                <label for="contact-md">Contenido (Markdown)</label>
                <textarea id="contact-md" name="content_md" required></textarea>
            </div>
            <button type="submit" class="btn btn-primary">Guardar Cambios de Contacto</button>
        </div>
    </details>
  </form>
`;

class AdminEditor extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
    this.products = [];
  }

  async connectedCallback() {
    this.loadInitialData();
    this.addFormListeners();
  }

  async loadInitialData() {
    // Load Home Data
    const homeData = await getHomeData();
    this.shadowRoot.querySelector('#home-title').value = homeData.title;
    this.shadowRoot.querySelector('#home-subtitle').value = homeData.subtitle;
    this.shadowRoot.querySelector('#home-footer').value = homeData.footer_text_md;
    const blocksContainer = this.shadowRoot.querySelector('#home-blocks');
    homeData.blocks_md.forEach((content, index) => {
        const group = document.createElement('div');
        group.className = 'form-group';
        group.innerHTML = `
            <label for="home-block-${index}">Bloque de Contenido ${index + 1} (Markdown)</label>
            <textarea id="home-block-${index}" name="block_md_${index}">${content}</textarea>
        `;
        blocksContainer.appendChild(group);
    });

    // Load Products Data
    const productsData = await getProductsData();
    this.products = productsData.items;
    const productSelect = this.shadowRoot.querySelector('#product-select');
    this.products.forEach(p => {
        const option = document.createElement('option');
        option.value = p.id;
        option.textContent = p.name;
        productSelect.appendChild(option);
    });
    productSelect.addEventListener('change', () => this.populateProductForm());
    this.populateProductForm();

    // Load Contact Data
    const contactData = await getContactData();
    this.shadowRoot.querySelector('#contact-md').value = contactData.content_md;
  }

  populateProductForm() {
    const selectedId = this.shadowRoot.querySelector('#product-select').value;
    const product = this.products.find(p => p.id === selectedId);
    if (product) {
      this.shadowRoot.querySelector('#product-name').value = product.name;
      this.shadowRoot.querySelector('#product-price').value = product.price;
      this.shadowRoot.querySelector('#product-stock').value = product.stock;
    }
  }

  addFormListeners() {
    this.shadowRoot.querySelector('#home-form').addEventListener('submit', async (e) => {
      e.preventDefault();
      const formData = new FormData(e.target);
      const payload = {
        title: formData.get('title'),
        subtitle: formData.get('subtitle'),
        blocks_md: [
            formData.get('block_md_0'),
            formData.get('block_md_1'),
            formData.get('block_md_2'),
        ].filter(Boolean), // Filter out empty blocks
        footer_text_md: formData.get('footer_text_md'),
      };
      await updateHomeData(payload);
      alert('Contenido de Inicio guardado (ver consola).');
    });

    this.shadowRoot.querySelector('#products-form').addEventListener('submit', async (e) => {
      e.preventDefault();
      const selectedId = this.shadowRoot.querySelector('#product-select').value;
      const formData = new FormData(e.target);
      const payload = {
        name: formData.get('name'),
        price: parseFloat(formData.get('price')),
        stock: parseInt(formData.get('stock'), 10),
      };
      await updateProductData(selectedId, payload);
      alert(`Producto ${selectedId} guardado (ver consola).`);
    });

    this.shadowRoot.querySelector('#contact-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const payload = {
            content_md: formData.get('content_md'),
        };
        await updateContactData(payload);
        alert('Contenido de Contacto guardado (ver consola).');
    });
  }
}

customElements.define('admin-editor', AdminEditor);