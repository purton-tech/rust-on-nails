## Build locally

`cd rust-fullstack-devcontainer`

`docker build -t ianpurton/rust-fullstack-devcontainer .` 


## Start a new Project

`mkdir proj-name`

`cd proj-name`

`cargo init`

## Create .devcontainer

```
mkdir .devcontainer \
  && curl -L https://github.com/ianpurton/development_environments/archive/master.zip -O -J \
  && unzip development_environments-master.zip \
  && mv development_environments-master/rust-fullstack-devcontainer/Dockerfile.devcontainer .devcontainer/Dockerfile \
  && mv development_environments-master/rust-fullstack-devcontainer/docker-compose.yml .devcontainer \
  && mv development_environments-master/rust-fullstack-devcontainer/devcontainer.json .devcontainer \
  && mv development_environments-master/rust-fullstack-devcontainer/ps1.bash .devcontainer \
  && mv development_environments-master/rust-fullstack-devcontainer/aliases.bash .devcontainer \
  && mv development_environments-master/rust-fullstack-devcontainer/.githooks .devcontainer \
  && rm -rf development_environments-master*
```

Windows

```
mkdir .devcontainer 
curl -L https://github.com/ianpurton/development_environments/archive/master.zip -O -J
tar -xf  development_environments-master.zip 
move development_environments-master\rust-fullstack-devcontainer\Dockerfile.devcontainer .devcontainer\Dockerfile
move development_environments-master\rust-fullstack-devcontainer\docker-compose.yml .devcontainer 
move development_environments-master\rust-fullstack-devcontainer\devcontainer.json .devcontainer 
move development_environments-master\rust-fullstack-devcontainer\ps1.bash .devcontainer 
move development_environments-master\rust-fullstack-devcontainer\aliases.bash .devcontainer 
move development_environments-master\rust-fullstack-devcontainer\.githooks .devcontainer 
del /S development_environments-master.zip
rmdir /S /Q development_environments-master
```
