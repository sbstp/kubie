#Kubie completion script

_kubie_complete()
{
    { \unalias command; \unset -f command; } >/dev/null 2>&1 || true
    local IFS=$'\n'
    COMPREPLY=($(command kubie get-completions --comp-cword="$COMP_CWORD" --comp-line="$COMP_LINE"))
}

export PATH="$PWD/target/debug:$PATH"
complete -F _kubie_complete kubie
