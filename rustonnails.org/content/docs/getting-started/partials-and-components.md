+++
title = "Partials and Components"
description = "AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme Doks for Zola."
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 130
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme <a href="https://github.com/h-enk/doks">Doks</a> for Zola.'
toc = true
top = false
+++

## Partials and Components

Partials in Rails are a way to split your templating into more manageable chunks. We can get the same thing in Rust by using functions. So if you have a piece of template you repeat often, this can be refactored into a function.

It's a similar process for components. With [markup.rs](https://github.com/utkarshkukreti/markup.rs) we can create use our existing templating to make libraries of reusable components.

Example 'Footer' component.

```rust
Footer<'a>(name: &'a str, year: u32) {
    "(c) " @year " " @name
}
```