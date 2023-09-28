+++
title = "Introduction"
description = "Introduction"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"

[extra]
toc = true
top = false
+++


![Yay! You're on Nails! screenshot](/yay-your-on-nails.png)

When creating software it's often a good practice to document the architecture using  a technique called [Architecture Decision Records](https://adr.github.io/).

An ADR is nothing more than a markdown document that records the title, status, context, decision, and consequences of a particular design choice.

When a decision is made it's often helpful to create a small Proof of Concept that illustrates how the decision will play out in the real world. 

## This Guide

After running through a few projects creating ADR's I realised a lot of them are re-usable. With the Proofs of Concept which are required to prove an architecture decision you end up with something almost like a tutorial.

This guide shows how to get Rust web applications into production.

The following applications were built using decisions that are documented here.

## Showcase

The following projects were built using these guidelines

- [Bionic GPT](https://github.com/purton-tech/bionicgpt?campaign=rustonnails)
- [SkyTrace](https://github.com/purton-tech/skytrace?campaign=rustonnails)

## Architecture


![Creating a vault](/architecture-diagram.svg)
