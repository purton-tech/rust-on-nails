const template = document.createElement('template');

template.innerHTML = `
<div class="drawer" part="base">
    <div part="overlay" class="drawer__overlay" tabindex="-1">
    </div>
    <div part="panel" class="drawer__panel" role="dialog" aria-modal="true"  tabindex="0">

        <header part="header" class="drawer__header">
            <span part="title" class="drawer__title" id="title">Title</span>
            <a hre="#" class="drawer__close" name="x" library="system">X</a>
        </header>
        <div class="drawer__body">
        </div>
        <footer part="footer" class="drawer__footer">
            <slot name="footer"></slot>
        </footer>
    </div>
</div>
`

export class SideDrawer extends HTMLElement {

    constructor() {
        super()
        const body = this.querySelector("template[slot='body']").cloneNode(true)
        const footer = this.querySelector("template[slot='footer']").cloneNode(true)
        const title = this.attributes.getNamedItem('label').value
        const templateNode = template.cloneNode(true)

        if(templateNode instanceof HTMLTemplateElement && body instanceof HTMLTemplateElement
            && footer instanceof HTMLTemplateElement) {
            const templateDocument = templateNode.content
            const drawerBody = templateDocument.querySelector(".drawer__body")
            drawerBody.appendChild(body.content)
            const drawerFooter = templateDocument.querySelector(".drawer__footer")
            drawerFooter.appendChild(footer.content)

            const templateTitle = templateDocument.querySelector(".drawer__title")
            templateTitle.innerHTML = title

            const thiz = this

            const closeButton = templateDocument.querySelector(".drawer__close")
            closeButton.addEventListener("click", function(e) {
                e.stopPropagation()
                thiz.open = false
            });

            const overlay = templateDocument.querySelector(".drawer__overlay")
            overlay.addEventListener("click", function(e) {
                e.stopPropagation()
                thiz.open = false
            });

            overlay.addEventListener('keydown', (event : Event) => {
                console.log(event)
                if(event instanceof KeyboardEvent) {
                    if (event.key === 'Escape') {
                        this.open = false
                    }
                }
              }, false);

            // Catch all clicks in the panel so they don't propogate up to the document
            const panel = templateDocument.querySelector(".drawer__panel")
            panel.addEventListener("click", function(e) {
                e.stopPropagation()
            });
    
            this.appendChild(templateDocument)
        }

    }

    static get observedAttributes() {
        return ['open'];
    }

    get open(): Boolean {
        return Boolean(this.getAttribute('open'))
    }

    set open(value: Boolean) {
        this.setAttribute('open', value.toString())
    }

    attributeChangedCallback(name: string, oldVal: string, newVal: string) {
        if (oldVal !== newVal) {
            switch (name) {
                case 'open':
                    var val = false
                    if(newVal == 'true') {
                        val = true
                    }
                    if(val == true) {
                        this.querySelector('.drawer').classList.remove('drawer--open')
                        this.querySelector('.drawer').classList.add('drawer--open')
                    } else {
                        this.querySelector('.drawer').classList.remove('drawer--open')
                    }
                    break;
            }
        }
    }
}

customElements.define('side-drawer', SideDrawer);