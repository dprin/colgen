# Intro

This project was made because I wanted to use the same or similar theme for
a lot of configs that I have. Previously I was using pywal, but I didn't like
that everything was stored in `.cache`, and that I couldn't configure where
each file went.

As such, the goal of this application is to allow users to create templates,
use colorschemes on them and output them wherever they want, in a way so that
updating colorschemes is as simple as changing the colorscheme in config and
writing a simple command.

# How to install

(TODO: check if `cargo install` works after a later, more stable version)

# How to use

Create a template folder in `(TODO)`, along with a `config.toml` file in `(TODO)`.

Create at least a default colorscheme inside of the config file:
```toml
[colorschemes.default]
color = "value"
```

And create a file in template folder that will be transformed, for example:

```toml
bg = {color}
```

When running the command, you should see a file in the default 
output folder (`TODO`):

```toml
bg = value
```
