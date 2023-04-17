(()=>{const t="Copy";document.addEventListener("DOMContentLoaded",(function(e){async function n(e){const n=e.srcElement;n.innerText="Copied",setTimeout((()=>{n.innerText=t}),5e3);let o=n.parentElement.querySelector("code").innerText;await navigator.clipboard.writeText(o)}document.querySelectorAll("pre").forEach((e=>{if(navigator.clipboard){let o=document.createElement("button");o.innerText=t,o.classList.add("clipboard"),o.addEventListener("click",n),e.appendChild(o)}}))})),function(){"use strict";window.goatcounter&&window.goatcounter.vars?window.goatcounter=window.goatcounter.vars:window.goatcounter=window.goatcounter||{};var t=document.querySelector("body[data-goatcounter]");if(t&&t.dataset.goatcounterSettings){try{var e=JSON.parse(t.dataset.goatcounterSettings)}catch(t){console.error("invalid JSON in data-goatcounter-settings: "+t)}for(var n in e)["no_onload","no_events","allow_local","allow_frame","path","title","referrer","event"].indexOf(n)>-1&&(window.goatcounter[n]=e[n])}var o=encodeURIComponent,r=function(t){return null==t||"function"==typeof t},a=function(){var t=window,e=document;return t.callPhantom||t._phantom||t.phantom?150:t.__nightmare?151:e.__selenium_unwrapped||e.__webdriver_evaluate||e.__driver_evaluate?152:navigator.webdriver?153:0},i=function(t){console&&"warn"in console&&console.warn("goatcounter: "+t)},c=function(){var t=document.querySelector("body[data-goatcounter]");return t&&t.dataset.goatcounter?t.dataset.goatcounter:goatcounter.endpoint||window.counter},u=function(){var t=location,e=document.querySelector('link[rel="canonical"][href]');if(e){var n=document.createElement("a");n.href=e.href,n.hostname.replace(/^www\./,"")===location.hostname.replace(/^www\./,"")&&(t=n)}return t.pathname+t.search||"/"},l=function(t){null===document.body?document.addEventListener("DOMContentLoaded",(function(){t()}),!1):t()};goatcounter.filter=function(){return"visibilityState"in document&&"prerender"===document.visibilityState?"visibilityState":goatcounter.allow_frame||location===parent.location?!goatcounter.allow_local&&location.hostname.match(/(localhost$|^127\.|^10\.|^172\.(1[6-9]|2[0-9]|3[0-1])\.|^192\.168\.|^0\.0\.0\.0$)/)?"localhost":goatcounter.allow_local||"file:"!==location.protocol?!(!localStorage||"t"!==localStorage.getItem("skipgc"))&&"disabled with #toggle-goatcounter":"localfile":"frame"},window.goatcounter.url=function(t){var e=function(t){var e,n,o,i={p:void 0===t.path?goatcounter.path:t.path,r:void 0===t.referrer?goatcounter.referrer:t.referrer,t:void 0===t.title?goatcounter.title:t.title,e:!(!t.event&&!goatcounter.event),s:[window.screen.width,window.screen.height,window.devicePixelRatio||1],b:a(),q:location.search};return"function"==typeof i.r&&(e=i.r),"function"==typeof i.t&&(o=i.t),"function"==typeof i.p&&(n=i.p),r(i.r)&&(i.r=document.referrer),r(i.t)&&(i.t=document.title),r(i.p)&&(i.p=u()),e&&(i.r=e(i.r)),o&&(i.t=o(i.t)),n&&(i.p=n(i.p)),i}(t||{});if(null!==e.p){e.rnd=Math.random().toString(36).substr(2,5);var n=c();return n?n+function(t){var e=[];for(var n in t)""!==t[n]&&null!==t[n]&&void 0!==t[n]&&!1!==t[n]&&e.push(o(n)+"="+o(t[n]));return"?"+e.join("&")}(e):i("no endpoint found")}},window.goatcounter.count=function(t){var e=goatcounter.filter();if(e)return i("not counting because of: "+e);var n=goatcounter.url(t);if(!n)return i("not counting because path callback returned null");var o=document.createElement("img");o.src=n,o.style.position="absolute",o.style.bottom="0px",o.style.width="1px",o.style.height="1px",o.loading="eager",o.setAttribute("alt",""),o.setAttribute("aria-hidden","true");o.addEventListener("load",(function(){o&&o.parentNode&&o.parentNode.removeChild(o)}),!1),document.body.appendChild(o)},window.goatcounter.get_query=function(t){for(var e=location.search.substr(1).split("&"),n=0;n<e.length;n++)if(0===e[n].toLowerCase().indexOf(t.toLowerCase()+"="))return e[n].substr(t.length+1)},window.goatcounter.bind_events=function(){if(document.querySelectorAll){Array.prototype.slice.call(document.querySelectorAll("*[data-goatcounter-click]")).forEach((function(t){if(!t.dataset.goatcounterBound){var e=function(t){return function(){goatcounter.count({event:!0,path:t.dataset.goatcounterClick||t.name||t.id||"",title:t.dataset.goatcounterTitle||t.title||(t.innerHTML||"").substr(0,200)||"",referrer:t.dataset.goatcounterReferrer||t.dataset.goatcounterReferral||""})}}(t);t.addEventListener("click",e,!1),t.addEventListener("auxclick",e,!1),t.dataset.goatcounterBound="true"}}))}},window.goatcounter.visit_count=function(t){l((function(){(t=t||{}).type=t.type||"html",t.append=t.append||"body",t.path=t.path||u(),t.attr=t.attr||{width:"200",height:t.no_branding?"60":"80"},t.attr.src=c()+"er/"+o(t.path)+"."+o(t.type)+"?",t.no_branding&&(t.attr.src+="&no_branding=1"),t.style&&(t.attr.src+="&style="+o(t.style)),t.start&&(t.attr.src+="&start="+o(t.start)),t.end&&(t.attr.src+="&end="+o(t.end));var e={png:"img",svg:"img",html:"iframe"}[t.type];if(!e)return i("visit_count: unknown type: "+t.type);"html"===t.type&&(t.attr.frameborder="0",t.attr.scrolling="no");var n=document.createElement(e);for(var r in t.attr)n.setAttribute(r,t.attr[r]);var a=document.querySelector(t.append);if(!a)return i("visit_count: append not found: "+t.append);a.appendChild(n)}))},"#toggle-goatcounter"===location.hash&&("t"===localStorage.getItem("skipgc")?(localStorage.removeItem("skipgc","t"),alert("GoatCounter tracking is now ENABLED in this browser.")):(localStorage.setItem("skipgc","t"),alert("GoatCounter tracking is now DISABLED in this browser until "+location+" is loaded again."))),goatcounter.no_onload||l((function(){if("visibilityState"in document&&"visible"!==document.visibilityState){var t=function(e){"visible"===document.visibilityState&&(document.removeEventListener("visibilitychange",t),goatcounter.count())};document.addEventListener("visibilitychange",t)}else goatcounter.count();goatcounter.no_events||goatcounter.bind_events()}))}()})();
//# sourceMappingURL=primer-index.js.map