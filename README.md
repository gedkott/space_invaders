# space_invaders

## Debugging

Using VSCode, I found that when using the bundled feature for the rust bindings for SDL2. you have to tell lldb to load the shared library from ./target/debug. 
There is where the bundled feature places the compiled .so files for SDL2.

So, if you are using VSCode, add this line to the top level of the object for lldb configurations in launch.json:
```
            "preRunCommands": [
                "env LD_LIBRARY_PATH=./target/debug"
            ]
```
