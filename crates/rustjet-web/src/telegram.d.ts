/// <reference types="@twa-dev/types" />

declare global {
  interface Window {
    Telegram: {
      WebApp: import('@twa-dev/types').WebApp;
    };
  }
}

export {};
