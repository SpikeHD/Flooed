<h1 align="center">
 <img height="100px" src="https://raw.githubusercontent.com/SpikeHD/Flooed/main/assets/icon.png" />
 <br />
 Flooed
</h1>
<div align="center">
 <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/Flooed/build.yml" />
 <img src="https://img.shields.io/github/repo-size/SpikeHD/Flooed" />
</div>
<div align="center">
 <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/Flooed" />
 <img src="https://img.shields.io/github/release-date/SpikeHD/Flooed" />
 <img src="https://img.shields.io/github/stars/SpikeHD/Flooed" />
 <img src="https://img.shields.io/github/downloads/SpikeHD/Flooed/total" />
</div>

<div align="center">
  Flooed is a lightweight, themeable, and plugin-able Discord client that uses whatever browser you already use!
  <br />
  https://discord.gg/agQ9mRdHMZ

  <br />
  <br />
  <sup>Not into Flooed? Try <a href="https://github.com/SpikeHD/Dorion">Dorion</a> instead!</sup>
</div>

# Table of Contents

* [Installation](#installation)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [Features](#features)

# Installation

Download a release for your platform from the [releases page](https://github.com/SpikeHD/Flooed/releases)!


# Features

* Stupidly small (< 5mb) binary size
* Uses whatever browser you already have installed
  * While it should cover 90% of cases, I apologize in advance if your fork-of-a-fork of Firefox isn't detected
* Theme support
* Plugin support
  * Comes with [Shelter](https://github.com/uwu/shelter) by default
  * Also supports [Vencord](https://github.com/vendicated/vencord)
* RPC support through [rsRPC](https://github.com/SpikeHD/rsRPC) and the [shelteRPC plugin](https://github.com/SpikeHD/shelter-plugins?tab=readme-ov-file#shelterpc)

# Building

## Prerequisites

1. [Rust and Cargo](https://www.rust-lang.org/tools/install)
2. [Static WebUI binary](https://github.com/SpikeHD/webui/actions?query=branch%3Aext) (if on Windows, use the MSVC build)

## Steps

1. Clone the repository
2. Place `webui-2-static` in the root of the repository
3. Run `cargo build --release`

# Contributing

PRs, issues, etc. are all welcome!