#Kubie completion script

_kubiecomplete()
{
    local cur prev

    cur=${COMP_WORDS[COMP_CWORD]}
    prev=${COMP_WORDS[COMP_CWORD-1]}
    prevprev=${COMP_WORDS[COMP_CWORD-2]}

    case ${COMP_CWORD} in
        1)
            cmds="ctx edit edit-config exec help info lint ns"
            COMPREPLY=($(printf "%s\n" $cmds | grep -e "^$cur" | xargs))
            ;;
        2)
            case ${prev} in
                ctx)
                    COMPREPLY=($(kubie ctx | grep -e "^$cur" | xargs))
                    ;;
                edit)
                    COMPREPLY=($(kubie ctx | grep -e "^$cur" | xargs))
                    ;;
                exec)
                    COMPREPLY=($(kubie ctx | grep -e "^$cur" | xargs))
                    ;;
                ns)
                    COMPREPLY=($(kubie ns | grep -e "^$cur" | xargs))
                    ;;
            esac
            ;;
        3)
            case ${prevprev} in
                exec)
                    COMPREPLY=($(kubie exec ${prev} default kubectl get namespaces|tail -n+2|awk '{print $1}'| grep -e "^$cur" |xargs))
                    ;;
            esac
            ;;
        *)
            COMPREPLY=()
            ;;
    esac
}

complete -F _kubiecomplete kubie
