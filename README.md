# Kubie
`kubie` is an alternative to `kubectx`, `kubens` and `k on`. It offers context switching, namespace switching and
prompt modification in an isolated fashion.

* [Primer](#primer)
* [Usage](#usage)
* [Future plans](#future-plans)

## Primer
The main design goal of kubie is to isolate kubernetes environments from each other. Kubie will never modify your k8s
config files. Before spawning a shell in a new context, it will create a temporary config file which contains the
context you wish to use. The namespace changes are made in that temporary config file, leaving your original config
files untouched. This is how Kubie achieves isolation.

Kubie also supports recursive contexts. The depth of the context recursion if displayed in the shell prompt, the third
component of the prompt: `[context|namespace|depth]` for instance `[dev|services|3]`.

Kubie also has other nice features such as `kubie exec` which allows you to execute commands in a context and a
namespace without having to spawn a shell. There's also `kubie lint` which scans your k8s config files for issues
and informs you of what they are.

## Usage
Note that if you have [`fzf`](https://github.com/junegunn/fzf) installed, the experience will be greatly improved.
Selectable menus will be available when using `kubie ctx` and `kubie ns`.

---

* `kubie ctx` show the list of available contexts (if fzf is installed, display a selectable menu of contexts)
* `kubie ctx <context>` spawn a shell in the given context
* `kubie ctx <context> -n <namespace>` spawn a shell in the given context and namespace
* `kubie ns` show the list of available namespaces (if fzf is installed, display a selectable menu of namespaces)
* `kubie ns <namespace>` switch the current shell to the given namespace
* `kubie exec <context> <namespace> <cmd> <args>...` execute a command in the given context and namespace
* `kubie lint` lint k8s config files for issues
* `kubie info ctx` print name of current context
* `kubie info ns` print name of current namespace
* `kubie info depth` print depth of recursive contexts

## Future plans
* Integration with vault to automatically download k8s configs from a vault server
* Import/edit configs
