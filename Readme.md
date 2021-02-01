# Vim markdownizer

Goal: replicate [Roam](https://roamresearch.com/) features on top of markdown
files in neovim and add some project management tools

Status: early stages. Nothing working at the moment !

## Installation

Use your favorite plugin manager.

With Plug :

```vim
Plug 'mmai/vim-markdownizer', { 'do': 'sh install.sh' }
```

With dein :

```vim
- repo: mmai/vim-markdownizer
  build: 'sh install.sh'
```

The installation script will try to download the prebuilt binaries if they exists for your
platform. If not, it will try to compile the rust sources, in this case you will
need to have [Cargo](https://www.rust-lang.org/tools/install) available on your system.

## Usage

* Open dashboard `:MarkdownizerDashboard`
* Close dashboard `q`

In a markdown file
* init a new project : `:MarkdownizerInitProject`
* insert list of projects : `:MarkdownizerProjectList`
