+++
title = "Security"
description = "Securing Your Web Application"
date = 2023-03-10T08:00:00+00:00
updated = 2023-03-10T08:00:00+00:00
draft = false
weight = 120
sort_by = "weight"


[extra]
toc = true
top = false
+++

Security is extremely important in any application. Here are some things that you should consider.

## CSRF

Cross-Site Request Forgery (CSRF) is an attack that forces an end user to execute unwanted actions on a web application in which they’re currently authenticated. With a little help of social engineering (such as sending a link via email or chat), an attacker may trick the users of a web application into executing actions of the attacker’s choosing. If the victim is a normal user, a successful CSRF attack can force the user to perform state changing requests like transferring funds, changing their email address, and so forth. If the victim is an administrative account, CSRF can compromise the entire web application.

Source: [OWASP](https://owasp.org/www-community/attacks/csrf)

By default, when you login/register with barricade [https://rust-on-nails.com/docs/auxiliary-services/authentication/](https://rust-on-nails.com/docs/auxiliary-services/authentication/) a cookie is set which has SameSite=Strict

This is enough to prevent CSRF in most modern browsers (95%+ of browsers support this attribute, see [https://caniuse.com/same-site-cookie-attribute](https://caniuse.com/same-site-cookie-attribute)), however if you want additional protection for older browsers and want to be extra cautious you can practice practice defense in depth by integrating with the axum_csrf crate which provides traditional CSRF tokens.

[https://crates.io/crates/axum_csrf](https://crates.io/crates/axum_csrf)

## Pen Tester Checklist

For more information on important web security practices, check out this checklist: [https://pentestbook.six2dez.com/others/web-checklist](https://pentestbook.six2dez.com/others/web-checklist)
