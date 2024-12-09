+++
title = "Adding Some Style"
description = "Adding Some Style"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 90
sort_by = "weight"


[extra]
toc = true
top = false
+++

You may have guessed by now if you looked at the layout we created that we're going to use [Tailwind](https://tailwindcss.com/) for styling.

We'll add this to our `web-assets` folder and then to our layout. We have pre-installed [tailwind-extra](https://github.com/dobicinaitis/tailwind-cli-extra) so we can use tailwind without creating an npm pipeline.

## Adding Tailwind

```sh
cd crates/web-assets
tailwind-extra init
```
This will create a `tailwind.config.js` file.

We also need to create a `input.css` file.


```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

## Watching Tailwind

Add the following to your ´Justfile´

```justfile
tailwind:
    cd /workspace/crates/web-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
```

Now we can run 

```sh
just tailwind
```

The stylesheet will be compiled.