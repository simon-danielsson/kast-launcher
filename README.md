<p align="center">
    <img src="media/logo/logo.png" alt="kast" width="200"/>
</p>
  
<p align="center">
  <em>A launcher for applications, scripts and .AppImage files. <br>
        Built with rust for the i3 window manager.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/github/v/release/simon-danielsson/kast-launcher?color=blueviolet&style=flat-square" alt="Latest release" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/kast-launcher/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#controls">Controls</a> •
  <a href="#built-with">Built With</a> •
  <a href="#license">License</a>
</p>  
  
<p align="center">
  <img src="media/screenshots/2.png" alt="screenshot">
</p>

---
<div id="features"></div>

## ✨ Features
+ ⚡ Launch .desktop, .sh, .AppImage and binary files in a singular interface.
+ 📦 Organize apps into configurable "groups" for dumber and faster searching across hundreds of items.
+ 🎨 Simple (but effective) options for customization such as the font, colors, icons, proportions and more.
+ ⌨️ Intuitive controls and a minimal interface - zero visual distractions.
+ 🧠 Configurable with a toml file, ensuring your settings are easily portable across different machines. 
+ 🖊 A default configuration file is created automatically if none is present at launch.

> [!IMPORTANT]  
> Kast was created exclusively for my own use, on a computer running Void Linux with the i3 window manager. I can't guarantee that this program will run on your computer - for the best results, clone this repo and compile your own binary. If you're feeling lucky, follow the installation instructions below!
  
---
<div id="installation"></div>

## 💻 Installation
  
**0. (Optional) Install a nerdfont**  
The icon you set for each program is gonna be a devicon, and so you should download a nerdfont with devicon support. A fallback nerdfont is bundled in the binary though, and so you don't have to supply your own if you don't want to. (Kast also supports emojis)
[Install this font if you like](https://www.nerdfonts.com/font-downloads)  
  
**1a. Ensure `~/.local/bin/` exists and is in your shell path**
``` bash
mkdir -p ~/.local/bin
export PATH="$HOME/.local/bin:$PATH" # add this line to your shell config
```
  
**1b. Source your shell after the previous step**
``` bash
source ~/.bashrc   # or: source ~/.zshrc
```
  
**2. Download and install the latest release of Kast, and give it permissions**  
``` bash
curl -L -o ~/.local/bin/kast $(curl -s https://api.github.com/repos/simon-danielsson/kast-launcher/releases/latest \
| grep "browser_download_url.*kast\"" \
| cut -d '"' -f 4)
chmod +x ~/.local/bin/kast

```
  
**3. If using i3, add this line to your i3 config to make sure the launcher won't be launched as a tiled window**  
``` bash
for_window [title="kast"] floating enable
```
  
**4. Launch Kast for the first time to create a config file: `~/.config/kast/kast.toml`**  
``` bash
kast
```
  
**5. Bind Kast to a key in your linux config**
``` bash
bindsym $mod+space exec kast # i3 config
```

  
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
