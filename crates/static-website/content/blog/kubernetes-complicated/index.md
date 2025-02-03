## Introduction

What we'll show here is not that Kubernetes is not as complicated as people often think, but also that it can actually reduce the complexity on certain projects.

## Easy Install

![K3s Website](./k3s-screenshot.png)

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --disable=traefik --write-kubeconfig-mode="644"' sh -
mkdir -p ~/.kube
cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sed -i "s,127.0.0.1,$(hostname -I | awk '{print $1}'),g" ~/.kube/config
```

### Now the VM is ready for

- Deployments
- Jobs
- etvc etc.

## Infrastructure as Code

## Unified Development

## One skill many uses

- Clouds
- On prem

## But the learning curve

## Conclusion