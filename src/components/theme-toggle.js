import { toggleTheme } from '../lib/theme.js';

const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: inline-block;
    }
    button {
      background: none;
      border: none;
      cursor: pointer;
      padding: var(--spacing-sm);
      border-radius: var(--radius-full, 50%);
      color: var(--color-foreground);
      transition: background-color 0.2s;
    }
    button:hover {
      background-color: var(--color-muted);
    }
    svg {
      width: 1.25rem;
      height: 1.25rem;
      vertical-align: middle;
    }
    .icon-sun, .icon-moon {
      display: none;
    }
    html[data-theme='light'] .icon-sun {
      display: block;
    }
    html[data-theme='dark'] .icon-moon {
      display: block;
    }
  </style>
  <button title="Toggle theme">
    <span class="icon-sun">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-6.364-.386 1.591-1.591M3 12h2.25m.386-6.364 1.591 1.591" />
      </svg>
    </span>
    <span class="icon-moon">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
      </svg>
    </span>
  </button>
`;

class ThemeToggle extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
  }

  connectedCallback() {
    this.shadowRoot.querySelector('button').addEventListener('click', () => {
      toggleTheme();
      // We need to dispatch an event so the component can update its own icon state,
      // as the attribute is on the root html element, not the component itself.
      // A more robust solution might use a global state manager.
      // For this project, we'll just re-render the icons based on the new theme.
      // A simple way to do this is to just let the CSS handle it based on the html[data-theme] attribute.
      // The provided CSS in the template already does this.
    });
  }
}

customElements.define('theme-toggle', ThemeToggle);