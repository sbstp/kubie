# Kubie
`kubie` is an alternative to `kubectx`, `kubens` and `k on`. It spawns shells in isolated kubernetes environment.
Each shell gets a prompt

###  List available contexts
`kubie ctx`

### Enter a context
`kubie ctx <context>`

### Enter a context while also specifying the namespace
`kubie ctx <context> -n <namespace>

### List available namespaces
`kubie ns`

### Switch namespace
`kubie ns <namespace>`

### Get current context
`kubie info ctx`

### Get current namespace
`kubie info ns`

## Plan
* Each cluster has its own config file. The `current-context` in that file is always the same.
* Kubie spawns a shell with the correct KUBECONFIG variable and a modified PS1 to display the environment.
* `kubie ls` list contexts
* `kubie ctx <context>` spawn a shell with the given context
* `kubie ns <namespace>` switch namespace in current context
* `kubie edit <context>` edit the config file for the given context
* `kubie import -n <context> /path/to/config.yaml` import a cluster in kubie with the given name
* `kubie get-ctx` get current context
* `kubie get-ns` get current namespace
