// Initialize Telegram WebApp
const tg = window.Telegram.WebApp;

// Expand app to full height
tg.expand();

// Apply Telegram theme colors
document.documentElement.style.setProperty('--tg-theme-bg-color', tg.themeParams.bg_color || '#ffffff');
document.documentElement.style.setProperty('--tg-theme-text-color', tg.themeParams.text_color || '#000000');
document.documentElement.style.setProperty('--tg-theme-hint-color', tg.themeParams.hint_color || '#999999');
document.documentElement.style.setProperty('--tg-theme-link-color', tg.themeParams.link_color || '#2481cc');
document.documentElement.style.setProperty('--tg-theme-button-color', tg.themeParams.button_color || '#2481cc');
document.documentElement.style.setProperty('--tg-theme-button-text-color', tg.themeParams.button_text_color || '#ffffff');

// Show app is ready
tg.ready();

// Simple content for now
document.getElementById('content').innerHTML = `
    <div class="status">
        <p>✓ Telegram WebApp initialized</p>
        <p>User ID: ${tg.initDataUnsafe.user?.id || 'Unknown'}</p>
        <p>First Name: ${tg.initDataUnsafe.user?.first_name || 'Unknown'}</p>
    </div>
`;

console.log('Telegram WebApp initialized:', tg);
console.log('Init data:', tg.initData);
