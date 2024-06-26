# PyTon
---------------
PyTon is a free python learning software that is still in development

## Installation

### Using the release tab

If you want to test it out, download the newest version of PyTon (depending on your OS) and the files/ folder

Then put both, the folder and the Executable file, in the same directory somewhere on your computer

The tree should look like this (only change "head_directory" to your actual head directory)

```bash
head_directory/
├── files/
└── PyTon-Linux-x.x.x / PyTon-Windows-x.x.x
```

### Manual installation and compiling using git (Linux only)
#### 1. Download git
You need to have git installed
If you don't have it installed, you can simply install by using your package manager

If you don't know your package manager, simply look it up on the internet

##### Using the apt package manager
```
sudo apt-get install git
```

#### 2. Install rustc
You also need to install the rust compiler "rustc" using your package manager, just as you did with git

##### Using the apt package manager
```
sudo apt-get install rustc
```

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
--/PyTon-Windows-x.x.x
```
Replace the 2 dashes with the name of the directory the file is in. And replace the x.x.x with the version of the downloaded file

### Linux
If you are on a GNU/Linux based operating system, you can also simply run the file in the Terminal

--------------
If you have installed PyTon over the releases tab on GitGub take the following steps

#### 1. Change working directory to the directory you have installed the PyTon-Linux-x.x.x file in
```
cd --
```
Replace the 2 dashes with the directory

#### 2. Gain access to the file
```
chmod +x PyTon-Linux-x.x.x
```
#### 3.Running the file
```
./PyTon-Linux-x.x.x
```
---------------
If you have installed and compiled PyTon manually take the following steps

#### 1. Change working directory to the directory you have installed the PyTon-Linux-x.x.x file in
```
cd --
```
Replace the 2 dashes with the directory

#### 2. Running the file
```
./main
```

