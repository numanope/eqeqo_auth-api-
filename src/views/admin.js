import { initializeTheme } from '../lib/theme.js';
import '../components/app-shell.js';
import '../components/admin-editor.js';

// Initialize the theme
initializeTheme();

// The admin-editor component handles all its own logic,
// so this file just needs to import the necessary components.
document.addEventListener('DOMContentLoaded', () => {
    const appShell = document.querySelector('app-shell');
    if (appShell) {
        // The admin page doesn't have unique footer data, so we can leave it blank
        // or fetch the default home data. For simplicity, we'll leave it.
        // A more robust app might have a global data fetcher.
    }
});