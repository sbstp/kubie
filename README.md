# Kubie
<img src="./assets/logo.svg" align="right"/>

`kubie` is an alternative to `kubectx`, `kubens` and the `k on` prompt modification script. It offers context switching,
namespace switching and prompt modification in a way that makes each shell independent from others. It also has
support for split configuration files, meaning it can load Kubernetes contexts from multiple files. You can configure
the paths where kubie will look for contexts, see the [settings](#settings) section.

Kubie also has other nice features such as `kubie exec` which allows you to execute commands in a context and a
namespace without having to spawn a shell and `kubie lint` which scans your k8s config files for issues and informs
you of what they are.

* [Installation](#installation)
* [Usage](#usage)
* [Settings](#settings)
* [Future plans](#future-plans)

Thanks to [@ahermant](https://github.com/ahermant) for the lovely logo!

## Installation
You can download a binary for Linux or OS X on the [GitHub releases page](https://github.com/sbstp/kubie/releases). You
can use `curl` or `wget` to download it. Don't forget to `chmod +x` the file!

You can build `kubie` from source using `cargo` and crates.io. If you do not have a Rust compiler installed, go to
[rustup.rs](https://rustup.rs) to get one. Then you can run `cargo install kubie` and kubie will be downloaded from
crates.io and then built.

There is a `kubie` Nix package maintained by @illiusdope that you can install.

Homebrew support [is pending](https://github.com/Homebrew/homebrew-core/pull/52502).

### Bash autocomplete
If you want autocompletion for `kubie ctx`, `kubie ns` and `kubie exec`, please install this script:
```
sudo cp ./completion/kubie.bash $(pkg-config --variable=completionsdir bash-completion)/kubie.bash
```

## Usage
Note that if you have [`fzf`](https://github.com/junegunn/fzf) installed, the experience will be greatly improved.
Selectable menus will be available when using `kubie ctx` and `kubie ns`.

---

* `kubie ctx` show the list of available contexts (if fzf is installed, display a selectable menu of contexts)
* `kubie ctx <context>` switch the current shell to the given context (spawns a shell if not a kubie shell)
* `kubie ctx -` switch back to the previous context
* `kubie ctx <context> -r` spawn a recursive shell in the given context
* `kubie ctx <context> -n <namespace>` spawn a shell in the given context and namespace
* `kubie ns` show the list of available namespaces (if fzf is installed, display a selectable menu of namespaces)
* `kubie ns <namespace>` switch the current shell to the given namespace
* `kubie ns -` switch back to the previous namespace
* `kubie ns <namespace> -r` spawn a recursive shell in the given namespace
* `kubie exec <context> <namespace> <cmd> <args>...` execute a command in the given context and namespace
* `kubie exec <wildcard> <namespace> <cmd> <args>...` execute a command in all the contexts matched by the wildcard and
  in the given namespace
* `kubie exec <wildcard> <namespace> -e <cmd> <args>...` execute a command in all the contexts matched by the wildcard and
  in the given namespace but fail early if any of the commands executed return a non-zero exit code
* `kubie edit` if fzf is installed, display a selectable menu of contexts to edit
* `kubie edit <context>` edit the file that contains this context
* `kubie edit-config` edit kubie's own config file
* `kubie lint` lint k8s config files for issues
* `kubie info ctx` print name of current context
* `kubie info ns` print name of current namespace
* `kubie info depth` print depth of recursive contexts
* `kubie update` will check the latest kubie version and update your local installation if needed

## Settings
You can customize kubie's behavior with the `~/.kube/kubie.yaml` file. The settings available and their defaults are
available below.

```yaml
# Force kubie to use a particular shell, if unset detect shell currently in use.
# Possible values: bash, dash, fish, zsh
# Default: unset
shell: bash

# Configure where to look for kubernetes config files.
configs:

    # Include these globs.
    # Default: values listed below.
    include:
        - ~/.kube/config
        - ~/.kube/*.yml
        - ~/.kube/*.yaml
        - ~/.kube/configs/*.yml
        - ~/.kube/configs/*.yaml
        - ~/.kube/kubie/*.yml
        - ~/.kube/kubie/*.yaml

    # Exclude these globs.
    # Default: values listed below.
    # Note: kubie's own config file is always excluded.
    exclude:
        - ~/.kube/kubie.yaml

# Prompt settings.
prompt:
    # Disable kubie's custom prompt inside of a kubie shell. This is useful
    # when you already have a prompt displaying kubernetes information.
    # Default: false
    disable: true

    # When using recursive contexts, show depth when larger than 1.
    # Default: true
    show_depth: true

    # When using zsh, show context and namespace on the right side using RPS1.
    # Default: false
    zsh_use_rps1: false
```

## Future plans
* Integration with vault to automatically download k8s configs from a vault server
* Import/edit configs
