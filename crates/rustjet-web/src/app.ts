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

// API client
class ApiClient {
    private baseUrl = '';

    private async fetch(path: string, options: RequestInit = {}): Promise<Response> {
        const headers = {
            'X-Telegram-Init-Data': tg.initData,
            'Content-Type': 'application/json',
            ...options.headers,
        };

        const response = await fetch(`${this.baseUrl}${path}`, {
            ...options,
            headers,
        });

        if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
        }

        return response;
    }

    async getUser() {
        const response = await this.fetch('/api/user');
        return response.json();
    }

    async getTickets() {
        const response = await this.fetch('/api/tickets');
        return response.json();
    }

    async saveCredentials(accountCode: string, password: string) {
        await this.fetch('/api/credentials', {
            method: 'POST',
            body: JSON.stringify({ account_code: accountCode, password }),
        });
    }

    async deleteCredentials() {
        await this.fetch('/api/credentials', {
            method: 'DELETE',
        });
    }

    async saveNotificationSettings(enabled: boolean) {
        await this.fetch('/api/settings/notifications', {
            method: 'POST',
            body: JSON.stringify({ enabled }),
        });
    }
}

const api = new ApiClient();

// Show app is ready
tg.ready();

// Simple content for now
const contentEl = document.getElementById('content');
if (contentEl) {
    contentEl.innerHTML = `
    <div class="status">
        <p>✓ Telegram WebApp initialized</p>
        <p>User ID: ${tg.initDataUnsafe.user?.id || 'Unknown'}</p>
        <p>First Name: ${tg.initDataUnsafe.user?.first_name || 'Unknown'}</p>
    </div>
`;
}

console.log('Telegram WebApp initialized:', tg);
console.log('Init data:', tg.initData);
