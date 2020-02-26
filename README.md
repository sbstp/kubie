# Kubie
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

## Installation
First, clone the repo.

If you have `rust` and `cargo` on your machine, you can use `cargo install --path .` to build the project and install
it in `~/.cargo/bin`. Make sure `~/.cargo/bin` is in your PATH variable.

If you don't have Rust installed you can simply copy one of the pre-built binaries available in the releases folder to
one of the directories in your PATH variable. For instance you can do `sudo cp releases/linux/amd64/v0.3.0/kubie /usr/local/bin`
from the git repo.

You may also create a bash alias such as `alias kubectx='kubie ctx'` and `alias kubens='kubie ns'` since old habits die
hard.

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

## Settings
You can customize kubie's behavior with the `~/.kube/kubie.yaml` file. The settings available and their defaults are
available below.

```yaml
configs: # configure where to lookup the kubernetes configs
    include: # include these globs
        - ~/.kube/config
        - ~/.kube/*.yml
        - ~/.kube/*.yaml
        - ~/.kube/configs/*.yml
        - ~/.kube/configs/*.yaml
        - ~/.kube/kubie/*.yml
        - ~/.kube/kubie/*.yaml
    exclude: # exclude these globs
        - ~/.kube/kubie.yaml
prompt: # prompt settings
    show_depth: true # show depth
```

## Future plans
* Integration with vault to automatically download k8s configs from a vault server
* Import/edit configs
