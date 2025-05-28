<div align="center">
<img src="media/fib.jpeg" height="100">

# _☰fib_ ⚡️

## A Fibonacci calculator written in multiple languages to see what out-preforms what.
<img src="http://badgen.net/github/commits/rhhen122/fib/">
<a href="http://vimp.rhhen.xyz/Licenses/lookinggood/non/UNLICENSE.html">
<img src="http://badgen.net/static/license/VIMPNL/black"></a>
<img src="http://badgen.net/static/Very/Code Rich/green?icon=https://www.svgrepo.com/show/535314/code.svg">

[![forthebadge](https://forthebadge.com/images/featured/featured-built-with-love.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/featured/featured-gluten-free.svg)](https://forthebadge.com)

<a href="/CONTRIBUTING.md">Contributing</a>
|
<a href="/basics/1.md">Basic Docs</a>
|
<a href="/extra/COPYRIGHT">Copyright</a>
|
<a href="http://www.apache.org/licenses/LICENSE-2.0">License</a>
|
<a href="http://codeberg.org/rhhen122/fib">Codeberg</a>
|
<a href="http://github.com/rhhen122/fib">Github</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci_sequence">Fibonacci Sequence Wikipedia</a>
|
<a href="http://en.wikipedia.org/wiki/Fibonacci">Leonardo Fibonacci Wikipedia</a>
|
<a href="http://roky.rhhen.xyz">Who is rhhen122?</a>
|
<a href="http://rhhen122.github.io/fib/">Web Docs</a>

<h4>Special thanks to:

<img src="media/docker.png" height="10"> <a href="http://www.docker.com">Docker</a>
|
<img src="media/vscode.png" height="10"> <a href="http://code.visualstudio.com">Visual Studio Code</a>
|
<img src="../git/media/github-white.png" height="10"> <a href="http://github.com">Github</a>
|
<img src="media/codeberg.png" height="10"> <a href="http://codeberg.org">Codeberg</a>
|
<img src="media/debian.png" height="10"> <a href="http://www.debian.org/">Debian</a>
|
<img src="../git/media/git.png" height="10"> <a href="http://git-scm.com">Git</a>
|
<img src="media/helix.svg" height="10"> <a href="http://helix-editor.com">Helix</a>
<br>
For making this project possible!
</h4>
</div>
<br>
<img align="left" src="media/image.png" height="300">

######

If you are not very technologically sassy then don't worry! We have done all the hard work for you using `Docker`

First install the codebase, then run one of the commands inside of the `fib` dir and boom!

Within seconds your runnning the code on your machine without ever installing any dependencies.

When you run the program you are starting up a tiny linux machine install the dependencies to that and then running it on the machine.

<br>

#

### Install
```
curl -s https://raw.githubusercontent.com/rhhen122/fib/refs/heads/master/extra/install.sh | bash
```

#

### Tutorial.rs
Go into the `Tutorial/tutorial.rs` file and change any of the functions to `0` to run them.
```
docker build -t fib-dock-tut ./Tutorial
docker run fib-dock-tut
```

### How to use

#### Ruby
```
docker pull rokyh/fib-dock
docker run rokyh/fib-dock
```

Or compile the image yourself. Run the following in your terminal:
```
docker build -t fib-dock-rb ./RUBY-Docker
docker run fib-dock-rb
```

This will make a `docker` container and run the script `fib-op.rb`

#### C
Compile from source:
```
docker build -t fib-dock-c ./C-Docker
docker run fib-dock-c
```

#### JS
```
docker build -t fib-dock-js ./JS-Docker
docker run fib-dock-js
```

#### PHP
```
docker build -t fib-dock-php ./PHP-Docker
docker run fib-dock-php
```

#### Rust
```
docker build -t fib-dock-rust ./RUST-Docker
docker run fib-dock-rust
```

#### Java
```
docker build -t fib-dock-java ./JAVA-Docker
docker run fib-dock-java
```

#### Go
```
docker build -t fib-dock-go ./GO-Docker
docker run fib-dock-go
```

#### C++
```
docker build -t fib-dock-cpp ./CPP-Docker
docker run fib-dock-cpp
```

#### Python
```
docker build -t fib-dock-py ./BF-Docker
docker run fib-dock-py
```

#### Shell
```
docker build -t fib-dock-sh ./SHELL-Docker
docker run fib-dock-sh
```

#### Typescript
```
docker build -t fib-dock-ts ./TS-Docker
docker run fib-dock-ts
```

### Credits
Max Hendra

##### <a href="https://war.ukraine.ua/support-ukraine/">Support Ukraine! 🇺🇦</a>
