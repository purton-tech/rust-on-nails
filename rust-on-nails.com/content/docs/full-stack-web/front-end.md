+++
title = "Front end enhancement"
description = "Front End"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 120
sort_by = "weight"


[extra]
toc = true
top = false
+++

Sometimes you need to enhance server side generated HTML. There are many ways to do this for example [Stimulus](https://stimulus.hotwired.dev/), which I've used on multiple projects.

However all modern browser come with web components built in and as they are pretty simple to use it makes sense to implement client side enhancement using this technology.

An example of a very simple component create the following in `app/src/asset-pipelibne/components/hello_world.ts`.

```typescript
//define a class extending HTMLElement
class HelloWorld extends HTMLElement {
    connectedCallback () {
      this.innerHTML = 'Hello, World!'
    }
}

//register the new custom element
customElements.define( 'hello-world', HelloWorld )
```

Include the element into your `app/src/asset-pipeline/index.ts` i.e.

```typescript
import './scss/index.scss'
import './components/hello_world.ts'
```

To use the element

```html
<hello-world></hello-world>
```