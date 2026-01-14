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

// Types
interface User {
    id: number;
    first_name: string;
    username?: string;
    has_credentials: boolean;
    notifications_enabled: boolean;
}

interface Ticket {
    id: number;
    ticket_code: string;
    route: {
        from: string;
        to: string;
        departure_time: string;
        arrival_time: string;
    };
    price: {
        amount: number;
        currency: string;
    };
    state: 'Valid' | 'Cancelled' | 'Expired' | { Unknown: string };
}

// State
let user: User | null = null;
let tickets: Ticket[] = [];

// UI rendering
function render() {
    const contentEl = document.getElementById('content');
    if (!contentEl) return;

    if (!user) {
        contentEl.innerHTML = '<p class="loading">Loading...</p>';
        return;
    }

    contentEl.innerHTML = `
        <div class="section">
            <h2>Settings</h2>
            <div class="setting-item">
                <label>
                    <input type="checkbox" id="notifications-toggle" ${user.notifications_enabled ? 'checked' : ''}>
                    <span>Enable notifications</span>
                </label>
            </div>
        </div>

        ${user.has_credentials ? `
            <div class="section">
                <h2>Your Tickets</h2>
                <div id="tickets-list"></div>
            </div>
            <div class="section">
                <button id="remove-credentials" class="button-danger">Remove Account</button>
            </div>
        ` : `
            <div class="section">
                <h2>Connect RegioJet Account</h2>
                <form id="credentials-form">
                    <input type="text" id="account-code" placeholder="Account code" required>
                    <input type="password" id="password" placeholder="Password" required>
                    <button type="submit">Save Credentials</button>
                </form>
            </div>
        `}
    `;

    // Render tickets if user has credentials
    if (user.has_credentials) {
        renderTickets();
    }

    // Attach event listeners
    attachEventListeners();
}

function renderTickets() {
    const ticketsEl = document.getElementById('tickets-list');
    if (!ticketsEl) return;

    if (tickets.length === 0) {
        ticketsEl.innerHTML = '<p class="hint">No upcoming tickets found.</p>';
        return;
    }

    ticketsEl.innerHTML = tickets.map(ticket => {
        const departureDate = new Date(ticket.route.departure_time);
        const arrivalDate = new Date(ticket.route.arrival_time);
        const stateClass = typeof ticket.state === 'string' ? ticket.state.toLowerCase() : 'unknown';

        return `
            <div class="ticket ${stateClass}">
                <div class="ticket-header">
                    <span class="ticket-code">#${ticket.ticket_code}</span>
                    <span class="ticket-state">${typeof ticket.state === 'string' ? ticket.state : 'Unknown'}</span>
                </div>
                <div class="ticket-route">
                    <div class="route-station">${ticket.route.from}</div>
                    <div class="route-arrow">→</div>
                    <div class="route-station">${ticket.route.to}</div>
                </div>
                <div class="ticket-time">
                    ${departureDate.toLocaleString()} - ${arrivalDate.toLocaleTimeString()}
                </div>
                <div class="ticket-price">
                    ${ticket.price.amount.toFixed(2)} ${ticket.price.currency}
                </div>
            </div>
        `;
    }).join('');
}

function attachEventListeners() {
    // Notifications toggle
    const notificationsToggle = document.getElementById('notifications-toggle') as HTMLInputElement;
    if (notificationsToggle) {
        notificationsToggle.addEventListener('change', async () => {
            try {
                await api.saveNotificationSettings(notificationsToggle.checked);
                tg.showPopup({ message: 'Settings saved' });
            } catch (error) {
                console.error('Failed to save settings:', error);
                tg.showPopup({ message: 'Failed to save settings' });
                notificationsToggle.checked = !notificationsToggle.checked;
            }
        });
    }

    // Credentials form
    const credentialsForm = document.getElementById('credentials-form') as HTMLFormElement;
    if (credentialsForm) {
        credentialsForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            const accountCode = (document.getElementById('account-code') as HTMLInputElement).value;
            const password = (document.getElementById('password') as HTMLInputElement).value;

            try {
                await api.saveCredentials(accountCode, password);
                await loadData();
                tg.showPopup({ message: 'Credentials saved' });
            } catch (error) {
                console.error('Failed to save credentials:', error);
                tg.showPopup({ message: 'Failed to save credentials' });
            }
        });
    }

    // Remove credentials button
    const removeButton = document.getElementById('remove-credentials');
    if (removeButton) {
        removeButton.addEventListener('click', async () => {
            try {
                await api.deleteCredentials();
                await loadData();
                tg.showPopup({ message: 'Account removed' });
            } catch (error) {
                console.error('Failed to remove credentials:', error);
                tg.showPopup({ message: 'Failed to remove account' });
            }
        });
    }
}

// Load data from API
async function loadData() {
    try {
        user = await api.getUser();

        if (user.has_credentials) {
            const response = await api.getTickets();
            tickets = response.tickets || [];
        } else {
            tickets = [];
        }

        render();
    } catch (error) {
        console.error('Failed to load data:', error);
        const contentEl = document.getElementById('content');
        if (contentEl) {
            contentEl.innerHTML = '<p class="error">Failed to load data. Please try again.</p>';
        }
    }
}

// Initialize app
tg.ready();
loadData();

console.log('Telegram WebApp initialized:', tg);
