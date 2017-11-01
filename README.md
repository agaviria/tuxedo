TUXEDO
======
Tuxedo is Rust dressed up in Elm with Sass and bootstrap.  Rocket, is the backend framework, Elm for purely functional frontend development, and handlebars as our HTML templating engine.  The beauty of tuxedo is that you can mix and match your stack to your hearts content.

## Initial build and run:

* Initialize node project<br/>
```
$> cd app
$> npm init
 ```

 - The next (2) bullets are optional. Do this if you prefer to have latest build dependencies.

* (Optional) Open package.json delete `devDependencies`

* (Optional) Load development dependencies.  Do this to get latest npm dependencies versions.

```
npm install --save-dev auto-reload-brunch babel-brunch brunch clean-css-brunch css-brunch elm elm-brunch uglify-js-brunch
```

* Install elm project dependencies (Assumes you have Elm installed in system) <br/>
 ```
$> cd elm
$> elm-package install elm-lang/html
 ```

* Run brunch script <br/>
 ```
$> cd app
$> brunch build
 ```

* Cargo build && Run <br/>
 ```
$> cargo build
$> cargo run
 ```

Pointing your browser to [http://localhost:8000](http://localhost:8000) should render text/html.<br/>
*Caveat: Must have Rust nightly as default.*

## Script run
Once you have built the code for the first time successfully, you can run `cargo script setup` a script to do some grunt work.  Make sure it builds with status code 0.  Consequentially, run `cargo build.`
