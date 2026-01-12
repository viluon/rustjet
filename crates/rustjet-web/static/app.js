"use strict";(()=>{var e=window.Telegram.WebApp;e.expand();document.documentElement.style.setProperty("--tg-theme-bg-color",e.themeParams.bg_color||"#ffffff");document.documentElement.style.setProperty("--tg-theme-text-color",e.themeParams.text_color||"#000000");document.documentElement.style.setProperty("--tg-theme-hint-color",e.themeParams.hint_color||"#999999");document.documentElement.style.setProperty("--tg-theme-link-color",e.themeParams.link_color||"#2481cc");document.documentElement.style.setProperty("--tg-theme-button-color",e.themeParams.button_color||"#2481cc");document.documentElement.style.setProperty("--tg-theme-button-text-color",e.themeParams.button_text_color||"#ffffff");e.ready();var t=document.getElementById("content");t&&(t.innerHTML=`
    <div class="status">
        <p>\u2713 Telegram WebApp initialized</p>
        <p>User ID: ${e.initDataUnsafe.user?.id||"Unknown"}</p>
        <p>First Name: ${e.initDataUnsafe.user?.first_name||"Unknown"}</p>
    </div>
`);console.log("Telegram WebApp initialized:",e);console.log("Init data:",e.initData);})();
//# sourceMappingURL=app.js.map
