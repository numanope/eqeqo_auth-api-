const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
      color: white;
    }
    .hero {
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      text-align: center;
      padding: var(--spacing-xl) var(--spacing-md);
      min-height: 60vh;
      background-size: cover;
      background-position: center;
      border-radius: var(--radius-lg);
      overflow: hidden;
      margin: 0 auto;
    }
    .hero::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: rgba(0, 0, 0, 0.5); /* Overlay for readability */
      z-index: 1;
    }
    .hero-content {
      position: relative;
      z-index: 2;
      max-width: 700px;
    }
    h1 {
      font-size: 3rem;
      margin-bottom: var(--spacing-sm);
      color: white;
    }
    p {
      font-size: 1.25rem;
      margin-bottom: var(--spacing-lg);
      color: rgba(255, 255, 255, 0.9);
    }
  </style>
  <section class="hero">
    <div class="hero-content">
      <h1 id="title"></h1>
      <p id="subtitle"></p>
      <a href="/tienda.html" class="btn btn-primary">Ver tienda</a>
    </div>
  </section>
`;

class HeroSection extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  set data(heroData) {
    if (!heroData) return;
    const { title, subtitle, hero_image_url } = heroData;
    const heroEl = this.shadowRoot.querySelector('.hero');
    heroEl.style.backgroundImage = `url('${hero_image_url}')`;
    this.shadowRoot.querySelector('#title').textContent = title;
    this.shadowRoot.querySelector('#subtitle').textContent = subtitle;
  }
}

customElements.define('hero-section', HeroSection);