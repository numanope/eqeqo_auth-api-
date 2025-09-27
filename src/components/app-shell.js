import './theme-toggle.js';

const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }
    .header {
      background-color: var(--color-background);
      border-bottom: 1px solid var(--color-border);
      padding: var(--spacing-md) 0;
      position: sticky;
      top: 0;
      z-index: 10;
      box-shadow: var(--shadow-sm);
    }
    .header-nav {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .logo {
      font-family: var(--font-serif);
      font-size: 1.5rem;
      font-weight: 700;
      color: var(--color-foreground);
      text-decoration: none;
    }
    .nav-links {
      display: flex;
      align-items: center;
      gap: var(--spacing-lg);
    }
    .nav-links a {
      font-weight: 500;
      color: var(--color-muted-foreground);
      text-decoration: none;
      transition: color 0.2s;
    }
    .nav-links a:hover,
    .nav-links a.active {
      color: var(--color-primary);
    }
    main {
      flex-grow: 1;
      padding: var(--spacing-xl) 0;
    }
    .footer {
      background-color: var(--color-muted);
      border-top: 1px solid var(--color-border);
      padding: var(--spacing-xl) 0;
      color: var(--color-muted-foreground);
    }
    .footer-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: var(--spacing-lg);
    }
    .footer-socials {
        display: flex;
        gap: var(--spacing-lg);
    }
    .footer-socials a {
        color: var(--color-muted-foreground);
    }
    .footer-socials svg {
        width: 1.5rem;
        height: 1.5rem;
    }
    .footer-text {
        text-align: center;
    }
  </style>
  <header class="header">
    <div class="container header-nav">
      <a href="/" class="logo">CMS Simple</a>
      <nav class="nav-links">
        <a href="/">Inicio</a>
        <a href="/tienda.html">Tienda</a>
        <a href="/contact.html">Contacto</a>
        <theme-toggle></theme-toggle>
      </nav>
    </div>
  </header>
  <main>
    <slot></slot>
  </main>
  <footer class="footer">
      <div class="container footer-content">
        <div id="footer-socials" class="footer-socials"></div>
        <div id="footer-text" class="footer-text"></div>
      </div>
  </footer>
`;

// Clean SVG icons for social media
const socialIcons = {
    instagram: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line></svg>`,
    twitter: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"></path></svg>`,
    facebook: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path></svg>`,
};


class AppShell extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  connectedCallback() {
    this.updateActiveLink();
  }

  updateActiveLink() {
    const currentPath = window.location.pathname;
    this.shadowRoot.querySelectorAll('.nav-links a').forEach(link => {
      if (link.getAttribute('href') === currentPath || (currentPath.startsWith('/tienda') && link.getAttribute('href').includes('tienda'))) {
        link.classList.add('active');
      }
    });
  }

  set footerData({ text, links }) {
    const socialContainer = this.shadowRoot.getElementById('footer-socials');
    const textContainer = this.shadowRoot.getElementById('footer-text');

    socialContainer.innerHTML = links.map(link => `
        <a href="${link.url}" target="_blank" rel="noopener noreferrer" title="${link.name}">
            ${socialIcons[link.name] || ''}
        </a>
    `).join('');

    // A simple markdown-like renderer for the footer text
    textContainer.innerHTML = text.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
  }
}

customElements.define('app-shell', AppShell);