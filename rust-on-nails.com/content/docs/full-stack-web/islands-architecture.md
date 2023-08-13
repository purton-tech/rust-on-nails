+++
title = "Islands Architecture"
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

The [islands architecture](https://www.patterns.dev/posts/islands-architecture) encourages small, focused chunks of interactivity within server-rendered web pages. The output of islands is progressively enhanced HTML, with more specificity around how the enhancement occurs. Rather than a single application being in control of full-page rendering, there are multiple entry points. The script for these "islands" of interactivity can be delivered with web components, allowing the rest of the page to be just static HTML.

## Implementing islands architecture

There are several ways to support the islands architecture for example [Stimulus](https://stimulus.hotwired.dev/), which I've used on multiple projects.

However all modern browser come with web components built in and as they are pretty simple to use it makes sense to implement client side enhancement using this technology.

## Example WebComponent

An example of a very simple component create the following in `crates/asset-pipelines/components/hello_world.ts`.

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
