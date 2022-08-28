+++
title = "Layouts"
description = "Layouts"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 110
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

Layouts are pieces that fit together (for example header, footer, menus, etc) to make a complete view. An application may have as many layouts as needed. 

In Nails a layout is just a function that takes HTML content and returns more HTML content. Let's put together our cache busting strategy with our asset pipeline into a Layout we can use.

## Create a Layout

Create `app/templates/layout.rs.html` with the following

```rust
@(title: &str, content: Content, header: Content)

<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    </meta>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    </meta>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    </meta>
    <title>@title</title>
</head>

<body>
    <div>
        <header>@:header()</header>
        <main class="container">
            <section class="content">
                <div>
                @:content()
                </div>
            </section>
        </main>
    </div>
</body>
</html>
```

## Integrate assets from our asset pipeline

Add the following to our `app/templates/layout.rs.html` just below the `<title>` node.

```html
<link rel="stylesheet" href="/static/@index_css.name" type="text/css" />
<script src="/static/@index_js.name" type="text/javascript" async></script>
```

This will include our styles and any JavaScript.

## Call the Layout

To use the layout from your template simply call it from another template. e.g.

```html
@use crate::queries::Fortune;

@(title: &str, fortunes: Vec<Fortune>)

@:layout_html(title, {
    <table>
        <tr><th>id</th><th>message</th></tr>
        @for fortune in fortunes {
            <tr><td>@fortune.id</td><td>@fortune.message</td></tr>
        }
    </table>
},
{
    <h2>Header Content</h2>
})
```