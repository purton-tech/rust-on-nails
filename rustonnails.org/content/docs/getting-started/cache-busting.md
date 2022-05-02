+++
title = "Cache Busting and Images"
description = "Cache Busting and Images"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 100
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

Cache busting is where we invalidate a cached file and force the browser to retrieve the file from the server. We can instruct the browser to bypass the cache by simply changing the filename. To the browser, this is a completely new resource so it will fetch the resource from the server. The most common way to do this is to add the hash of the file to the URL.

What we can do with Rust is take advantage of the `build.rs` mechanism which runs some code before each compile. We can generate a set of function that let us retrieve our assets and generate the necessary hashes at the same time. So for example to use the `index.css` in our code it would be nice to be able to call something like.

```rust
get_index_css() // Returns the URL with the hash.
```

The code for this is quite large so I won't publish it here. Please check out [https://github.com/purton-tech/cloak/blob/main/app/build.rs](https://github.com/purton-tech/cloak/blob/main/app/build.rs) for a full implementation.

You'll also need to add the following to your `app/Cargo.toml`

```
tower-http = { version = "0", default-features = false, features = ["fs", "trace"] }

[build-dependencies]
sha1 = "0"  # Use by build.rs for cache busting.
```

Now when your build your project a helper class will be created which we will use in the next section.
