# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gdb='git branch | grep -v "main" | xargs git branch -D'

gitsetup() {
  git config --global user.name "$NAME"
  git config --global user.email "$EMAIL"

  echo "Git user.name  = $(git config --global --get user.name)"
  echo "Git user.email = $(git config --global --get user.email)"
}

# Database (you'll need to run just dev-secrets before these will work)
db() {
  /workspace/scripts/psql "$@"
}

dbmate() {
  /workspace/scripts/dbmate "$@"
}

alias dbdown='while dbmate down; do :; done'

alias j='just'
alias c=clear
