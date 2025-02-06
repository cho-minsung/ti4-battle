# ti4-battle
Twilight Imperium 4 battle simulator

# build and run locally
```bash
cargo run --release
```

# build and run in docker
```bash
docker build -t battle-simulator . && docker run battle-simulator
```

explanation:
-t battle-simulator: This option tags the image with the name battle-simulator. Tags are like labels that make it easier to refer to the image later. You can use this tag instead of the image ID when running the container.
- .: This argument specifies the build context. The . refers to the current directory where the command is executed. Docker looks for a file named Dockerfile in this directory, which contains the instructions for building the image. All files and directories in the current directory are also sent to the Docker daemon as the build context, even if they are not explicitly referenced in the Dockerfile. This is so the COPY command can access them. It's good practice to have a .dockerignore file to exclude unnecessary files from being sent to the Docker daemon to improve build performance.
