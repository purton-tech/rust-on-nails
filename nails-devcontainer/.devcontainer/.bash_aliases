# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL'

# Database (you'll need to run just dev-secrets before these will work)
db() {
  set -a
  source /workspace/.env
  set +a
  export DATABASE_URL="${DATABASE_URL:-$MIGRATIONS_URL}"
  psql "$DATABASE_URL" "$@"
}

dbmate() {
  set -a
  source /workspace/.env
  set +a
  export DATABASE_URL="${DATABASE_URL:-$MIGRATIONS_URL}"
  command dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations "$@"
}

alias dbdown='while dbmate down; do :; done'