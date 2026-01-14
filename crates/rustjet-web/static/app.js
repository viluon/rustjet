"use strict";(()=>{var n=window.Telegram.WebApp;n.expand();document.documentElement.style.setProperty("--tg-theme-bg-color",n.themeParams.bg_color||"#ffffff");document.documentElement.style.setProperty("--tg-theme-text-color",n.themeParams.text_color||"#000000");document.documentElement.style.setProperty("--tg-theme-hint-color",n.themeParams.hint_color||"#999999");document.documentElement.style.setProperty("--tg-theme-link-color",n.themeParams.link_color||"#2481cc");document.documentElement.style.setProperty("--tg-theme-button-color",n.themeParams.button_color||"#2481cc");document.documentElement.style.setProperty("--tg-theme-button-text-color",n.themeParams.button_text_color||"#ffffff");var d=class{constructor(){this.baseUrl=""}async fetch(e,s={}){let o={"X-Telegram-Init-Data":n.initData,"Content-Type":"application/json",...s.headers},a=await fetch(`${this.baseUrl}${e}`,{...s,headers:o});if(!a.ok)throw new Error(`API error: ${a.status}`);return a}async getUser(){return(await this.fetch("/api/user")).json()}async getTickets(){return(await this.fetch("/api/tickets")).json()}async saveCredentials(e,s){await this.fetch("/api/credentials",{method:"POST",body:JSON.stringify({account_code:e,password:s})})}async deleteCredentials(){await this.fetch("/api/credentials",{method:"DELETE"})}async saveNotificationSettings(e){await this.fetch("/api/settings/notifications",{method:"POST",body:JSON.stringify({enabled:e})})}},r=new d,i=null,c=[];function p(){let t=document.getElementById("content");if(t){if(!i){t.innerHTML='<p class="loading">Loading...</p>';return}t.innerHTML=`
        <div class="section">
            <h2>Settings</h2>
            <div class="setting-item">
                <label>
                    <input type="checkbox" id="notifications-toggle" ${i.notifications_enabled?"checked":""}>
                    <span>Enable notifications</span>
                </label>
            </div>
        </div>

        ${i.has_credentials?`
            <div class="section">
                <h2>Your Tickets</h2>
                <div id="tickets-list"></div>
            </div>
            <div class="section">
                <button id="remove-credentials" class="button-danger">Remove Account</button>
            </div>
        `:`
            <div class="section">
                <h2>Connect RegioJet Account</h2>
                <form id="credentials-form">
                    <input type="text" id="account-code" placeholder="Account code" required>
                    <input type="password" id="password" placeholder="Password" required>
                    <button type="submit">Save Credentials</button>
                </form>
            </div>
        `}
    `,i.has_credentials&&g(),h()}}function g(){let t=document.getElementById("tickets-list");if(t){if(c.length===0){t.innerHTML='<p class="hint">No upcoming tickets found.</p>';return}t.innerHTML=c.map(e=>{let s=new Date(e.route.departure_time),o=new Date(e.route.arrival_time);return`
            <div class="ticket ${typeof e.state=="string"?e.state.toLowerCase():"unknown"}">
                <div class="ticket-header">
                    <span class="ticket-code">#${e.ticket_code}</span>
                    <span class="ticket-state">${typeof e.state=="string"?e.state:"Unknown"}</span>
                </div>
                <div class="ticket-route">
                    <div class="route-station">${e.route.from}</div>
                    <div class="route-arrow">\u2192</div>
                    <div class="route-station">${e.route.to}</div>
                </div>
                <div class="ticket-time">
                    ${s.toLocaleString()} - ${o.toLocaleTimeString()}
                </div>
                <div class="ticket-price">
                    ${e.price.amount.toFixed(2)} ${e.price.currency}
                </div>
            </div>
        `}).join("")}}function h(){let t=document.getElementById("notifications-toggle");t&&t.addEventListener("change",async()=>{try{await r.saveNotificationSettings(t.checked),n.showPopup({message:"Settings saved"})}catch(o){console.error("Failed to save settings:",o),n.showPopup({message:"Failed to save settings"}),t.checked=!t.checked}});let e=document.getElementById("credentials-form");e&&e.addEventListener("submit",async o=>{o.preventDefault();let a=document.getElementById("account-code").value,m=document.getElementById("password").value;try{await r.saveCredentials(a,m),await l(),n.showPopup({message:"Credentials saved"})}catch(u){console.error("Failed to save credentials:",u),n.showPopup({message:"Failed to save credentials"})}});let s=document.getElementById("remove-credentials");s&&s.addEventListener("click",async()=>{try{await r.deleteCredentials(),await l(),n.showPopup({message:"Account removed"})}catch(o){console.error("Failed to remove credentials:",o),n.showPopup({message:"Failed to remove account"})}})}async function l(){try{i=await r.getUser(),i.has_credentials?c=(await r.getTickets()).tickets||[]:c=[],p()}catch(t){console.error("Failed to load data:",t);let e=document.getElementById("content");e&&(e.innerHTML='<p class="error">Failed to load data. Please try again.</p>')}}n.ready();l();console.log("Telegram WebApp initialized:",n);})();
//# sourceMappingURL=app.js.map
