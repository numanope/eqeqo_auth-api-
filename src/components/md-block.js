import { renderMarkdown } from '../lib/md.js';

const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
      padding: var(--spacing-md) 0;
    }
    /* Styles for rendered markdown */
    .content ::slotted(h2) {
      font-size: 1.5rem;
      margin-bottom: var(--spacing-sm);
      border-bottom: 1px solid var(--color-border);
      padding-bottom: var(--spacing-sm);
    }
    .content ::slotted(p) {
      max-width: 65ch;
      line-height: 1.7;
    }
    .content ::slotted(a) {
      font-weight: 500;
    }
  </style>
  <div class="content">
    <slot></slot>
  </div>
`;

class MdBlock extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  static get observedAttributes() {
    return ['content'];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'content' && oldValue !== newValue) {
      this.render(newValue);
    }
  }

  set content(value) {
    this.setAttribute('content', value);
  }

  get content() {
    return this.getAttribute('content');
  }

  render(markdown) {
    if (!markdown) {
      this.innerHTML = '';
      return;
    }
    const html = renderMarkdown(markdown);
    // We render the sanitized HTML into the light DOM to allow for easier global styling
    // and to let the content be accessible to screen readers without shadow DOM piercing.
    this.innerHTML = html;
  }
}

customElements.define('md-block', MdBlock);