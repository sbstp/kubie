# Kubie completion script

_kubie_complete()
{
    local cmd
    local prev
    local cur

    function set_result() {
        local line
        COMPREPLY=()
        while IFS='' read -r line; do COMPREPLY+=("$line"); done <<< "$1"
    }

    { \unalias command; \unset -f command; } >/dev/null 2>&1 || true

    case ${COMP_CWORD} in
        1)
            cur="${COMP_WORDS[COMP_CWORD]}"
            set_result "$(command compgen -W "ctx edit edit-config help info lint ns update" -- "$cur")"
            ;;
        2)
            cmd="${COMP_WORDS[COMP_CWORD-1]}"
            cur="${COMP_WORDS[COMP_CWORD]}"
            case $cmd in
                ctx)
                    set_result "$(command compgen -W "$(kubie ctx) -f --kubeconfig" -- "$cur")"
                    ;;
                edit|exec)
                    set_result "$(command compgen -W "$(kubie ctx)" -- "$cur")"
                    ;;
                ns)
                    set_result "$(command compgen -W "$(kubie ns 2> /dev/null || true)" -- "$cur")"
                    ;;
            esac
            ;;
        *)
            cmd="${COMP_WORDS[1]}"
            prev="${COMP_WORDS[COMP_CWORD-1]}"
            cur="${COMP_WORDS[COMP_CWORD]}"
            case $cmd in
                ctx)
                    case $prev in
                        -f|--kubeconfig)
                            set_result "$(command compgen -f -- "$cur")"
                        ;;
                    esac
                    ;;
            esac
            ;;
    esac
}

complete -F _kubie_complete kubie
