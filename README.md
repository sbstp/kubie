# Kubie
`kubie` is an alternative to `kubectx`, `kubens` and `k on`. The main design goal of kubie is to isolalate kubernetes
environments from each other. Everytime you enter a context with kubie, it spawns a new shell which is isolated from
other shells. If you enter a context in another shell it won't affect the other shells in any way.

Kubie also has other nice features such as `kubie exec` which allows you to execute commands in a context without
having to spawn a shell.

Other features are also planned. One of them is a config manager which helps you keep clean config files by detecting
orphanned clusters and users. The command will probably be something like `kubie vet`.

# Commands

###  List available contexts
`kubie ctx`

### Enter a context
`kubie ctx <context>`

### Enter a context while also specifying the namespace
`kubie ctx <context> -n <namespace>`

### List available namespaces
`kubie ns`

### Switch namespace
`kubie ns <namespace>`

### Get current context
`kubie info ctx`

### Get current namespace
`kubie info ns`

### Execute command in context & namespace
`kubie exec <context> <namespace> <command> <args...>`
