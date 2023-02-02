# Deployment of this action

## Updating Binaryen

Check https://github.com/WebAssembly/binaryen/tags for the latest version and update all occurrences of the version number in the [Dockerfile](Dockerfile).

## Updating the action

1. Build a new docker image and push it to dockerhub
   * `docker build -t niklasei/wasm-opt-action:v<new version> . --no-cache`
       * tag it with the new action version
   * `docker push niklasei/wasm-opt-action:v<new version>`
   * update the docker image version in [action.yml](action.yml)
2. Push all changes to GitHub
3. Tag the latest commit with the new action version
    * remember to move all other sem-version compatible tags to the new commit
