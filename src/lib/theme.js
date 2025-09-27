const THEME_STORAGE_KEY = 'theme-preference';
const THEME_ATTRIBUTE = 'data-theme';

/**
 * Gets the preferred theme, checking localStorage first, then system settings.
 * @returns {'dark' | 'light'}
 */
const getPreferredTheme = () => {
  const storedTheme = localStorage.getItem(THEME_STORAGE_KEY);
  if (storedTheme) {
    return storedTheme;
  }
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
};

/**
 * Sets the theme by applying the 'data-theme' attribute to the document element.
 * @param {'dark' | 'light'} theme The theme to apply.
 */
const applyTheme = (theme) => {
  document.documentElement.setAttribute(THEME_ATTRIBUTE, theme);
};

/**
 * Toggles the current theme between light and dark and saves the preference.
 */
export const toggleTheme = () => {
  const currentTheme = document.documentElement.getAttribute(THEME_ATTRIBUTE) || getPreferredTheme();
  const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
  localStorage.setItem(THEME_STORAGE_KEY, newTheme);
  applyTheme(newTheme);
};

/**
 * Initializes the theme on page load. Should be called once in the main script.
 */
export const initializeTheme = () => {
  const initialTheme = getPreferredTheme();
  applyTheme(initialTheme);
};