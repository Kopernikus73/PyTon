# PyTon
---------------
PyTon is a free python learning software that is still in development

## Installation

### Using the realease tab

If you want to test it out, download the newest version of PyTon (depending on your OS) and the files/ folder (both can be found in the Release tab)

Then put both, the folder and the Executable file in the same directory somewhere on your computer

If you are on Linux,
run this in your terminal to get acces to it

(replace the x.x.x with the version of PyTon)

### Manual installation and compiling using git (Linux only)
### 1. Download git
You need to have git installed
If you don't hvae it installed, you can simply install by using your package manager
If you don't know your package manager, simply look it up on the internet

#### 2. Install rustc
You also need to install the rust compiler "rustc" over your package manager, just as you did with git

#### 3. Clone the repository
```
git clone https://github.com/Kopernikus73/PyTon
```
#### 4. Move to the /src/ directory and change permissions
```
cd PyTon/src
```
```
chmod +x main.rs
```
#### 5. Compile the program and move it outside of the /src/ directory
```
rustc main.rs
```
```
mv main ../
```

 ## Starting the Program
 ### Windows
 If you are on windows, you can simply run the file over the Terminal
 ```
--/PyTon-Windows-vx.x.x
```
Replace the 2 dashes with the name of the directory the file is in. And replace the x.x.x with the version of the downloaded file

### Linux
If you are on a GNU/Linux based operating system, you can also simply run the file in the Terminal

--------------
If you have installed PyTon over the releases tab on GitGub take the following steps

#### Change working directory to the directory you have installed the PyTon-Linux-vx.x.x file in
```
cd --
```
Replace the 2 dashes with the directory

#### Gain acces to the file
```
chmod +x PyTon-Linux-vx.x.x
```
#### Running the file
```
./PyTon-Linux-vx.x.x
```
---------------
If you have installed and compiled PyTon manually take the following steps

#### Change working directory to the directory you have installed the PyTon-Linux-vx.x.x file in
```
cd --
```
Replace the 2 dashes with the directory

#### Gain acces to the file
```
chmod +x main
```

#### Running the file
```
./main
```

