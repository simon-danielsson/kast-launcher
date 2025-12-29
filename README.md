<!-- <p align="center"> -->
<!--     <img src="media/logo.png" alt="kast" width="200"/> -->
<!-- </p> -->
  
<h1 align="center">
  <em>kast</em>
</h1>

<p align="center">
  <em>A GUI launcher for launching applications, scripts, <br>
        and .AppImage files all in the same place.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/github/v/release/simon-danielsson/kast-launcher?color=blueviolet&style=flat-square" alt="Latest release" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/kast/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#controls">Controls</a> •
  <a href="#built-with">Built With</a> •
  <a href="#license">License</a>
</p>  
  
<!-- <p align="center"> -->
<!--   <img src="media/screenshots/3.png" alt="screenshot"> -->
<!-- </p> -->

---
<div id="features"></div>

## ✨ Features
+ ⌨️ the features have not been written down yet :(

> [!IMPORTANT]  
> 'kast' was created exclusively for my own use on a computer running Void Linux with i3. I can't guarantee that this program will run on your computer - for the best results, clone this repo and adjust the source code to you liking!
  
---
<div id="installation"></div>

## 💻 Installation
  
**0. (Optional) Install a nerdfont**  
This program relies on the 0xProto Nerd Font for its icons (although the program works just fine without the font of course).  
[Install this font and set it as your terminal font](https://www.nerdfonts.com/font-downloads)  
  
**1. Ensure `~/.local/bin/` exists and is in your shell path**
``` bash
mkdir -p ~/.local/bin
export PATH="$HOME/.local/bin:$PATH" # add this line to your shell config
source ~/.bashrc   # source ~/.zshrc
```
  
**2. Download the latest release of kast**  
``` bash
curl -L https://github.com/simon-danielsson/kast/releases/latest/download/kast-linux-x86_64 -o ~/.local/bin/kast
```
  
**3. Make it executable**  
``` bash
chmod +x ~/.local/bin/kast
```
  
**3b. If using the i3 wm, add this line to your config**
``` bash
for_window [title="kast"] floating enable
```
  
**4. Launch kast for the first time to create a ".kast" config file home directory**  
``` bash
kast
```
  
**5. Bind kast to a key in your linux config!**
  
**6. Done!**
  

---
<div id="controls"></div>

## 🚀 Controls
  
```
[Esc]:
Quit the program

[Space/Enter]:
Launch application

[Up/Down]:
Navigate results
```

---
<div id="built-with"></div>

## 🛠️ Built With
+ [egui](https://github.com/emilk/egui)  
+ [eframe](https://github.com/emilk/egui)  
+ [winit](https://github.com/rust-windowing/winit)  
+ [serde](https://github.com/serde-rs/serde)  
+ [toml](https://github.com/toml-rs/toml)  
+ [home](https://docs.rs/home/latest/home/)  

---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/kast/blob/main/LICENSE).  
