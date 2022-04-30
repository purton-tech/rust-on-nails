# Git aliases.
alias gst='git status'
alias gcm='git checkout master'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
#alias gpsup='git push --set-upstream origin $(git_current_branch)'
alias gcb='git checkout -b'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL && mkdir -p ~/.ssh && cp -u /home/host-ssh/id_rsa ~/.ssh && chmod 600 ~/.ssh/id_rsa && ssh-keygen -y -f ~/.ssh/id_rsa > ~/.ssh/id_rsa.pub'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'

# Cargo watch
alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'


# npm
alias nrs='npm run start'

# Database migrations
alias mr='diesel migration run'
alias mre='diesel migration redo'
alias ml='diesel migration list'
alias db='psql $DATABASE_URL'

alias p='sudo chmod 777 /var/run/docker.sock'
# Leave a line below or the files will cat together

