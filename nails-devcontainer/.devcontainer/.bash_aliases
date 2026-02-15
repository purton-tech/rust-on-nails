# Auto-load /workspace/.env for interactive shells.
# This avoids needing a devcontainer reload when env values change.
# The file is re-sourced when its mtime changes, checked before each prompt.
__workspace_env_file="/workspace/.env"
__workspace_env_mtime=""

load_workspace_env() {
    if [ ! -f "$__workspace_env_file" ]; then
        return
    fi

    local mtime
    mtime="$(stat -c %Y "$__workspace_env_file" 2>/dev/null || stat -f %m "$__workspace_env_file" 2>/dev/null)"
    if [ "$mtime" = "$__workspace_env_mtime" ]; then
        return
    fi

    set -a
    . "$__workspace_env_file"
    set +a

    # Backward-compatible names used by existing aliases/scripts.
    export DATABASE_URL="${DATABASE_URL:-$APPLICATION_URL}"
    export SUPERUSER_DATABASE_URL="${SUPERUSER_DATABASE_URL:-$MIGRATIONS_URL}"

    __workspace_env_mtime="$mtime"
}

__workspace_env_prompt_hook() {
    load_workspace_env
}

# Run the env refresh hook as part of PROMPT_COMMAND (before each prompt).
# PROMPT_COMMAND is a Bash variable containing command(s) executed
# right before Bash prints the next interactive prompt.
# Preserve any existing PROMPT_COMMAND and avoid duplicate hook entries.
case ";$PROMPT_COMMAND;" in
    *";__workspace_env_prompt_hook;"*) ;;
    *) PROMPT_COMMAND="__workspace_env_prompt_hook${PROMPT_COMMAND:+; $PROMPT_COMMAND}" ;;
esac

# Initial load so aliases/commands in this shell have vars immediately.
load_workspace_env

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

# Cargo watch
alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'

# npm
alias nrs='npm run start'

# Database
alias dbmate='dbmate --no-dump-schema -e "SUPERUSER_DATABASE_URL" --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'
