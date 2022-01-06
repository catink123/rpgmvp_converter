# RPG Maker V Picture Converter
This simple program is made to convert an (almost) proprietary picture format from the RPG Maker V game engine.

## How to use it
There are two ways to use the program: through the terminal or using your OS' File Explorer. First, [compile the program from source](##how-to-compile-it) or download the compiled binaries from the [Releases](https://github.com/catink123/rpgmvp_converter/releases) section. Then, proceed with one of the ways to convert a file with it.

### The Terminal Way
Open a terminal window in the directory with the program executable. Then type out this,
```shell
$ rpgmvp_converter path_to_your_file
```
changing `path_to_your_file` with corresponding path to a file which you want to convert. If, for example, your file is called `picture.rpgmvp`, you would enter this in terminal:
```shell
$ rpgmvp_converter picture.rpgmvp
```
After executing the command, this will be the likely output:
```shell
Path to file: picture.rpgmvp
Exported to picture.rpgmvp.png
```
Congratulations, you now have a converted picture in the same directory as the input picture!
### The File Explorer Way
In your OS, open up a file explorer and navigate to the directory where the program executable is located. Next, drag your input picture on top of the program executable to open your picture with it. Alternatively, you can right-click your input picture and select "Open With...". There you find the program executable and select it. Congratulations, there should be a new PNG file next to your input file.

## How to compile it
Requirements:

* Rust 2021 edition (preferrably installed through [rustup](https://rustup.rs/))

Download this repository through the web interface (or using [this link](https://github.com/catink123/rpgmvp_converter/archive/refs/heads/main.zip)) or clone it using git:
```shell
$ git clone https://github.com/catink123/rpgmvp_converter.git
```
Then, navigate to the cloned repository and build it:
```shell
$ cd rpgmvp_converter
$ cargo build --release
```
If there were no errors during the build, you should have the program executable in the `target/release/` directory. Congratulations, you have built the program yourself!