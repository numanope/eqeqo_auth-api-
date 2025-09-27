const template = document.createElement('template');
template.innerHTML = `
  <style>
    :host {
      display: block;
    }
    .filter-group {
      margin-bottom: var(--spacing-lg);
    }
    .filter-label {
      font-weight: 600;
      margin-bottom: var(--spacing-sm);
      display: block;
    }
    .filter-options label {
      display: flex;
      align-items: center;
      gap: var(--spacing-sm);
      margin-bottom: var(--spacing-xs);
      cursor: pointer;
    }
  </style>
  <form id="filter-form"></form>
`;

class FilterBar extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
    this._filters = [];
  }

  set filters(filterData) {
    if (!filterData) return;
    this._filters = filterData;
    this.render();
  }

  get filters() {
    return this._filters;
  }

  render() {
    const form = this.shadowRoot.getElementById('filter-form');
    form.innerHTML = ''; // Clear previous filters

    this._filters.forEach(group => {
      const groupEl = document.createElement('div');
      groupEl.className = 'filter-group';

      const labelEl = document.createElement('span');
      labelEl.className = 'filter-label';
      labelEl.textContent = group.label;
      groupEl.appendChild(labelEl);

      const optionsEl = document.createElement('div');
      optionsEl.className = 'filter-options';

      group.options.forEach(option => {
        const optionLabel = document.createElement('label');
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.name = group.id;
        checkbox.value = option.tag;

        optionLabel.appendChild(checkbox);
        optionLabel.append(option.label);
        optionsEl.appendChild(optionLabel);
      });

      groupEl.appendChild(optionsEl);
      form.appendChild(groupEl);
    });

    form.addEventListener('change', this.onFilterChange.bind(this));
  }

  onFilterChange() {
    const formData = new FormData(this.shadowRoot.getElementById('filter-form'));
    const selectedTags = [];
    // In this simple case, we just collect all checked tags.
    for (const tag of formData.values()) {
        selectedTags.push(tag);
    }

    this.dispatchEvent(new CustomEvent('filter-changed', {
      detail: {
        selectedTags: selectedTags
      }
    }));
  }
}

customElements.define('filter-bar', FilterBar);