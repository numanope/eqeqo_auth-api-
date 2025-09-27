/**
 * A very simple and safe Markdown to HTML converter.
 * - Supports h2 (##), bold (**text**), and links ([text](url)).
 * - Sanitizes the output by removing any tags other than the allowed ones.
 * - Specifically blocks <script> and <iframe>.
 *
 * This is NOT a full-featured Markdown parser. It's a minimal implementation
 * for this specific project to avoid external dependencies.
 *
 * @param {string} md The Markdown string to convert.
 * @returns {string} A sanitized HTML string.
 */
export function renderMarkdown(md) {
  let html = md
    // Headlines (##)
    .replace(/^## (.*$)/gim, '<h2>$1</h2>')
    // Bold (**text**)
    .replace(/\*\*(.*?)\*\*/gim, '<strong>$1</strong>')
    // Links ([text](url))
    .replace(/\[(.*?)\]\((.*?)\)/gim, '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>')
    // Paragraphs (wrap lines that are not headlines)
    .split('\n')
    .map(line => {
      if (line.trim() === '') return '';
      if (line.startsWith('<h')) return line;
      return `<p>${line}</p>`;
    })
    .join('');

  return sanitize(html);
}

/**
 * Sanitizes an HTML string by removing potentially dangerous tags and attributes.
 * @param {string} dirtyHtml The HTML string to sanitize.
 * @returns {string} A cleaner, safer HTML string.
 */
function sanitize(dirtyHtml) {
  // Step 1: Use the browser's built-in parser to create a DOM tree.
  // This is safer than using regex to parse HTML.
  const tempDiv = document.createElement('div');
  tempDiv.innerHTML = dirtyHtml;

  // Step 2: Define allowed tags and attributes.
  const allowedTags = ['H2', 'P', 'STRONG', 'A', 'BR'];
  const allowedAttributes = ['href', 'target', 'rel'];

  // Step 3: Recursively clean the tree.
  const cleanNode = (node) => {
    // If it's not an element node (e.g., text node), keep it.
    if (node.nodeType !== Node.ELEMENT_NODE) {
      return;
    }

    // Remove the node if its tag is not allowed.
    if (!allowedTags.includes(node.tagName)) {
      node.parentNode?.removeChild(node);
      return;
    }

    // Remove any disallowed attributes.
    const attributes = Array.from(node.attributes);
    for (const { name } of attributes) {
      if (!allowedAttributes.includes(name)) {
        node.removeAttribute(name);
      }
    }

    // Recursively clean child nodes.
    const children = Array.from(node.children);
    for (const child of children) {
      cleanNode(child);
    }
  };

  // Start cleaning from the children of the temporary div.
  Array.from(tempDiv.children).forEach(child => cleanNode(child));

  return tempDiv.innerHTML;
}