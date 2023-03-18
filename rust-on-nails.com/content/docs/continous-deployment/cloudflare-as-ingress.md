+++
title = "Cloudflare as Ingress"
description = "Cloudflare as Ingress"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"


[extra]
toc = true
top = false
+++

[cloudfared](https://github.com/cloudflare/cloudflared) is a tool we can use to connect our cluster securely to the outside world.

Cloudflare also gives us the option to use [Quick Tunnels](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/do-more-with-tunnels/trycloudflare/) so we don't even have to setup a cloudflare account at this stage.

Cloudflare will handle TLS termination for us, saving us some setup on our cluster.

## Setting up the tunnel

Create a `infra-as-code/cloudflare.ts` and add the following

```typescript
import * as kx from "@pulumi/kubernetesx"
import * as pulumi from "@pulumi/pulumi"

export function cloudflareTunnel(
    namespace: pulumi.Output<string>,
    url: string) {
    const cloudflaredPod = new kx.PodBuilder({
        containers: [{
            name: "cloudflare-tunnel",
            image: "cloudflare/cloudflared:latest",
            command: ["cloudflared", "tunnel", "--url", url],
        }]
    })

    let deployName = pulumi.interpolate `${namespace}-cloudflare-tunnel`
    
    new kx.Deployment("cloudflare-tunnel", {
        metadata: {
            name: deployName,
            namespace: namespace
        },
        spec: cloudflaredPod.asDeploymentSpec({ replicas: 1 })
    })
}
```
