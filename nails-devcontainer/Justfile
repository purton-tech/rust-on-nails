dev-init:
    k3d cluster delete k3d-nails
    k3d cluster create k3d-nails --agents 1 -p "30000-30001:30000-30001@agent:0"
    just get-config

dev-setup:
    stack init
    stack install --manifest stack.dev.yaml

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write k3d-nails --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"
