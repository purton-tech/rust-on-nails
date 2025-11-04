if (document.readyState !== 'loading') {
    initCopyPaste();
} else {
    document.addEventListener("DOMContentLoaded", function () {
        initCopyPaste();
    });
}

function initCopyPaste() {
    const copyButtonLabel = "Copy Code";

    // Copy (clipboard) icon
    const copyIcon = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="white" viewBox="0 0 16 16">
            <path d="M10 1.5A1.5 1.5 0 0 1 11.5 3v1h-1V3a.5.5 0 0 0-.5-.5H4A1.5 1.5 0 0 0 2.5 4v8A1.5 1.5 0 0 0 4 13.5h6a.5.5 0 0 0 .5-.5v-1h1v1a1.5 1.5 0 0 1-1.5 1.5H4A2.5 2.5 0 0 1 1.5 12V4A2.5 2.5 0 0 1 4 1.5h6z"/>
            <path d="M13.5 4a.5.5 0 0 1 .5.5v8a.5.5 0 0 1-.5.5h-6a.5.5 0 0 1-.5-.5v-8a.5.5 0 0 1 .5-.5h6zm-.5 1h-5v7h5V5z"/>
        </svg>
    `;

    // Tick/check icon
    const tickIcon = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="white" viewBox="0 0 16 16">
            <path d="M13.485 1.037a.75.75 0 0 1 .078 1.06l-7.5 8a.75.75 0 0 1-1.08.02l-3.5-3.5a.75.75 0 1 1 1.06-1.06l2.927 2.927 6.94-7.39a.75.75 0 0 1 1.075-.057z"/>
        </svg>
    `;

    let blocks = document.querySelectorAll("pre");

    blocks.forEach((block) => {
        if (navigator.clipboard) {
            let button = document.createElement("button");
            button.innerHTML = copyIcon;
            button.setAttribute("aria-label", copyButtonLabel);

            button.style.position = "absolute";
            button.style.top = "5px";
            button.style.right = "5px";
            button.style.background = "black";
            button.style.border = "none";
            button.style.cursor = "pointer";
            button.style.padding = "4px";

            let wrapper = document.createElement("div");
            wrapper.style.position = "relative";

            block.parentNode.insertBefore(wrapper, block);
            wrapper.appendChild(block);
            wrapper.appendChild(button);

            button.addEventListener("click", async () => {
                await copyCode(block, button);
            });
        }
    });

    async function copyCode(block, button) {
        let text = block.innerText;

        await navigator.clipboard.writeText(text);

        // Replace icon with tick
        button.innerHTML = tickIcon;

        // Revert to original icon after 700ms
        setTimeout(() => {
            button.innerHTML = copyIcon;
        }, 700);
    }
}