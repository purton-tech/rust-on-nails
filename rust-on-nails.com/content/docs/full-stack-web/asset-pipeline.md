+++
title = "Asset Pipeline"
description = "Asset Pipeline"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 90
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

The asset pipeline provides a framework to concatenate and minify or compress JavaScript and CSS assets. It also adds the ability to write these assets in other languages and pre-processors such as [Typescript](https://www.typescriptlang.org/) and [Sass](https://sass-lang.com/).

I've used [Parcel](https://parceljs.org/) on several projects and before that [Webpack](https://webpack.js.org/). I've found Parcel to be easier to use and so that is the recommendation for Nails.

## Setting up a Volume

If you look at your `.devcontainer/docker-compose.yml` you'll see a line that is commented out.

```yml
#- node_modules:/vscode/node_modules # Set target as a volume for performance. 
```

Comment that back in and rebuild your devcontainer. This will setup the node_modules folder as a volume and you will get way better performance during builds. This is due to the fact the node_modules folder has many files and docker tries to sync them with your main file system.

## Installing Parcel

To install parcel

```sh
$ cd app
$ npm install --save-dev parcel
```

Now create an `app/asset-pipeline/index.ts`

```typescript
import './scss/index.scss'
```

And also `app/asset-pipeline/scss/index.scss`

```typescript
h1 {
    color: red;
}
```

Add a scripts section to your package.json

```json
  "scripts": {
    "start": "parcel watch ./asset-pipeline/index.ts",
    "release": "parcel build ./asset-pipeline/index.ts"
  },
```

## npm run start

And now when you run `npm run start` parcel will generate your assets into the dist folder. We should also update our `./.gitignore` to exclude the generated files.

```
/target
.parcel-cache
/app/dist
node_modules
```

## Adding Images

Create an empty images folder in `app/asset-pipeline/images` then your project should now look something like this.

```sh
.
├── .devcontainer/
│   └── ...
├── app
│   ├── .parcel-cache/  <-- Created by parcel for caching
│   │   └── ...
│   ├── asset-pipeline/
│   │   ├── images/
│   │   │   └── ...
│   │   ├── scss/
│   │   │   └── index.scss
│   │   └── index.ts
│   ├── dist/           <-- Where parcel builds your assets
│   │   └── ...
│   ├── node_modules/
│   │   └── ...
│   ├── src/
│   │   └── ...
│   ├── Cargo.toml
│   ├── package-lock.json
│   ├── package.json
├── db/
│   └── ...
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## What we have

We now have a pipeline to compile Typescript and SCSS assets and a place to store images.