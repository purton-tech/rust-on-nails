parse_git_branch() {
     branch=$(git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ (\1)/')
     if [ ! -z "$branch" ]; then
          echo "$branch"
     fi
}
export PS1="\[\033[32m\]\w\[\033[33m\]\$(parse_git_branch)\[\033[00m\] \n$ "
