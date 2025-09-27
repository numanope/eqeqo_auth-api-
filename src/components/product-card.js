const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
    }
    .card {
      /* Uses global card styles from main.css */
      display: flex;
      flex-direction: column;
      height: 100%;
    }
    .card__image-container {
      aspect-ratio: 4 / 3;
      overflow: hidden;
      background-color: var(--color-muted);
    }
    .card__image {
      width: 100%;
      height: 100%;
      object-fit: cover;
      transition: transform 0.3s ease-out;
    }
    .card:hover .card__image {
      transform: scale(1.05);
    }
    .card__content {
      padding: var(--spacing-lg);
      display: flex;
      flex-direction: column;
      flex-grow: 1;
    }
    .card__title {
      font-family: var(--font-serif);
      font-size: 1.25rem;
      font-weight: 600;
      margin-bottom: var(--spacing-sm);
    }
    .card__price {
      font-size: 1.1rem;
      font-weight: 500;
      color: var(--color-primary);
      margin-bottom: var(--spacing-sm);
    }
    .card__stock {
      font-size: 0.9rem;
      color: var(--color-muted-foreground);
      margin-bottom: var(--spacing-lg);
      flex-grow: 1;
    }
    .card__stock.low {
      color: #F87171; /* A sensible default for low stock warning */
    }
    .card__cta {
      margin-top: auto;
    }
  </style>
  <article class="card">
    <div class="card__image-container">
      <img class="card__image" loading="lazy" alt="">
    </div>
    <div class="card__content">
      <h3 class="card__title"></h3>
      <p class="card__price"></p>
      <p class="card__stock"></p>
      <a href="#" class="btn btn-primary card__cta">Ver Producto</a>
    </div>
  </article>
`;

class ProductCard extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  set product(data) {
    if (!data) return;
    this._product = data;
    this.render();
  }

  get product() {
    return this._product;
  }

  render() {
    const { id, name, price, stock, image_url } = this._product;

    const shadow = this.shadowRoot;
    shadow.querySelector('.card__image').src = image_url;
    shadow.querySelector('.card__image').alt = name;
    shadow.querySelector('.card__title').textContent = name;
    shadow.querySelector('.card__price').textContent = `$${price.toFixed(2)}`;

    const stockEl = shadow.querySelector('.card__stock');
    stockEl.textContent = `${stock} en stock`;
    stockEl.classList.toggle('low', stock < 10);

    // In a real app, this link would go to a product details page.
    shadow.querySelector('.card__cta').href = `/tienda?producto=${id}`;
  }
}

customElements.define('product-card', ProductCard);