import { initializeTheme } from '../lib/theme.js';
import { getHomeData } from '../lib/api.js';
import '../components/app-shell.js';
import '../components/hero-section.js';
import '../components/md-block.js';

// Initialize the theme as the first action
initializeTheme();

// Main function to render the home page content
async function renderHomePage() {
  const contentContainer = document.getElementById('home-content');
  const appShell = document.querySelector('app-shell');

  if (!contentContainer || !appShell) {
    console.error('Required elements #home-content or app-shell not found.');
    return;
  }

  try {
    const homeData = await getHomeData();

    // 1. Create and populate the Hero Section
    const heroSection = document.createElement('hero-section');
    heroSection.data = {
      title: homeData.title,
      subtitle: homeData.subtitle,
      hero_image_url: homeData.hero_image_url,
    };
    contentContainer.appendChild(heroSection);

    // 2. Create and populate the flexible Markdown blocks
    const blocksWrapper = document.createElement('div');
    blocksWrapper.className = 'container'; // Center the blocks
    homeData.blocks_md.forEach(markdownContent => {
      const mdBlock = document.createElement('md-block');
      mdBlock.content = markdownContent;
      blocksWrapper.appendChild(mdBlock);
    });
    contentContainer.appendChild(blocksWrapper);

    // 3. Populate the footer in the app-shell
    appShell.footerData = {
      text: homeData.footer_text_md,
      links: homeData.social_links,
    };

  } catch (error) {
    contentContainer.innerHTML = `
      <div class="container">
        <h2>Error al cargar el contenido</h2>
        <p>No se pudo obtener la información de la página. Por favor, inténtalo de nuevo más tarde.</p>
      </div>
    `;
    console.error('Failed to render home page:', error);
  }
}

// Execute the render function once the DOM is fully loaded
document.addEventListener('DOMContentLoaded', renderHomePage);