# Rust-Bind-Shell
A bind shell built in rust; compatible with both Windows and Linux. 
![](https://github.com/Msfv3n0m/Rust-Bind-Shell/blob/main/Rust-Bind-Shell.PNG)

## Usage
1. Open a codespace with this project and launch a tmux session <br>
2. `tmux` <br>
3. Hit `Ctrl + b` and then `"` to split the tmux session into two horizantal panes <br>
4. Run the project in one pane <br>
5. `cargo run ./src/` <br>
6. Hit `Ctrl + b` and then the up arrow to change the active tmux session to the top <br>
7. Enter a netcat connection on localhost port 4444 <br>
8. `nc 127.0.0.1 4444` <br>
