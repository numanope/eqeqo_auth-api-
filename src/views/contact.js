import { initializeTheme } from '../lib/theme.js';
import { getContactData, getHomeData } from '../lib/api.js';
import '../components/app-shell.js';
import '../components/md-block.js';

// Initialize the theme first
initializeTheme();

// Main function to render the contact page
async function renderContactPage() {
  const contentContainer = document.getElementById('contact-content');
  const appShell = document.querySelector('app-shell');

  if (!contentContainer || !appShell) {
    console.error('Required elements #contact-content or app-shell not found.');
    return;
  }

  try {
    // Fetch contact data and footer data
    const [contactData, homeData] = await Promise.all([
        getContactData(),
        getHomeData() // For footer
    ]);

    // Create an md-block to render the contact info
    const mdBlock = document.createElement('md-block');
    mdBlock.content = contactData.content_md;
    contentContainer.appendChild(mdBlock);

    // Populate the footer
    appShell.footerData = {
      text: homeData.footer_text_md,
      links: homeData.social_links,
    };

  } catch (error) {
    contentContainer.innerHTML = `
      <h2>Error al cargar la página</h2>
      <p>No se pudo obtener la información de contacto. Por favor, inténtalo de nuevo más tarde.</p>
    `;
    console.error('Failed to render contact page:', error);
  }
}

// Execute the render function once the DOM is fully loaded
document.addEventListener('DOMContentLoaded', renderContactPage);