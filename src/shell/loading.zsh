# Reference for loading behavior
# https://shreevatsa.wordpress.com/2008/03/30/zshbash-startup-files-loading-order-bashrc-zshrc-etc/

if [[ -f "/etc/zshenv" ]] ; then
    source "/etc/zshenv"
elif [[ -f "/etc/zsh/zshenv" ]] ; then
    source "/etc/zsh/zshenv"
fi

if [[ -f "$HOME/.zshenv" ]] ; then
    tmp_ZDOTDIR=$ZDOTDIR
    source "$HOME/.zshenv"
    # If the user has overridden $ZDOTDIR, we save that in $_KUBIE_USER_ZDOTDIR for later reference
    # and reset $ZDOTDIR
    if [[ "$tmp_ZDOTDIR" != "$ZDOTDIR" ]]; then
        _KUBIE_USER_ZDOTDIR=$ZDOTDIR
        ZDOTDIR=$tmp_ZDOTDIR
        unset tmp_ZDOTDIR
    fi
fi

# If a zsh_history file exists, copy it over before zsh initialization so history is maintained
if [[ -f "$HISTFILE" ]] ; then
    cp $HISTFILE $ZDOTDIR
fi

KUBIE_LOGIN_SHELL=0
if [[ "$OSTYPE" == "darwin"* ]] ; then
    KUBIE_LOGIN_SHELL=1
fi

if [[ -f "/etc/zprofile" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zprofile"
elif [[ -f "/etc/zsh/zprofile" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zsh/zprofile"
fi

if [[ -f "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zprofile" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zprofile"
fi

if [[ -f "/etc/zshrc" ]] ; then
    source "/etc/zshrc"
elif [[ -f "/etc/zsh/zshrc" ]] ; then
    source "/etc/zsh/zshrc"
fi

if [[ -f "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zshrc" ]] ; then
    source "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zshrc"
fi

if [[ -f "/etc/zlogin" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zlogin"
elif [[ -f "/etc/zsh/zlogin" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zsh/zlogin"
fi

if [[ -f "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zlogin" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "${{_KUBIE_USER_ZDOTDIR:-$HOME}}/.zlogin"
fi

unset _KUBIE_USER_ZDOTDIR

autoload -Uz add-zsh-hook

# This function sets the proper KUBECONFIG variable before a command runs,
# in case something overwrote it.
function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

add-zsh-hook preexec __kubie_cmd_pre_exec__
