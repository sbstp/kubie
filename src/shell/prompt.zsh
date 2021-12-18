# Activate prompt substitution.
setopt PROMPT_SUBST

# This function fixes the prompt via a precmd hook.
function __kubie_cmd_pre_cmd__() {{
    local KUBIE_PROMPT=$'{}'

    # If KUBIE_ZSH_USE_RPS1 is set, we use RPS1 instead of PS1.
    if [[ "$KUBIE_ZSH_USE_RPS1" == "1" ]] ; then

        # Avoid modifying RPS1 again if the RPS1 has not been reset.
        if [[ "$RPS1" != *"$KUBIE_PROMPT"* ]] ; then

            # If RPS1 is empty, we do not seperate with a space.
            if [[ -z "$RPS1" ]] ; then
                RPS1="$KUBIE_PROMPT"
            else
                RPS1="$KUBIE_PROMPT $RPS1"
            fi
        fi
    else
        # Avoid modifying PS1 again if the PS1 has not been reset.
        if [[ "$PS1" != *"$KUBIE_PROMPT"* ]] ; then
            PS1="$KUBIE_PROMPT $PS1"
        fi
    fi
}}

# When promptinit is activated, a precmd hook which updates PS1 is installed.
# In order to inject the kubie PS1 when promptinit is activated, we must
# also add our own precmd hook which modifies PS1 after promptinit themes.
add-zsh-hook precmd __kubie_cmd_pre_cmd__
