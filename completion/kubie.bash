#Kubie completion script

_kubie_complete()
{
    local IFS=$'\n'
    COMPREPLY=($(kubie get-completions --comp-cword="$COMP_CWORD" --comp-line="$COMP_LINE"))
}

export PATH="$PWD/target/debug:$PATH"
complete -F _kubie_complete kubie
