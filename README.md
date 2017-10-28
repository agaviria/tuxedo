TUXEDO
======
Tuxedo is Rust dressed up in Elm.  Rocket as the backend framework with Elm, and HTML template frontend.  
The template engine is yet to be defined.  Contenders are Tera, Askama, Handlebars or Maud.

## Initial build and run:

**Step 1:** Initialize node project<br/>
```
$> cd app
$> npm init
 ```

**Step 2:** (Optional)Open package.json delete `devDependencies`

**Step 3:** (Optional) Load development dependencies.  Do this to get latest npm dependencies versions.

```
npm install --save-dev auto-reload-brunch babel-brunch brunch clean-css-brunch css-brunch elm elm-brunch uglify-js-brunch
```

**Step 4:** Install elm project dependencies (Assumes you have Elm installed in system) <br/>
 ```
$> cd elm
$> package install elm-lang/html
 ```

**Step 5:** Run brunch script <br/>
 ```    
$> cd app
$> brunch build
 ```

**Step 6:** Cargo build && Run <br/>
 ```
$> cargo build
$> cargo run
 ```

Pointing your browser to [http://localhost:8000](http://localhost:8000) should render text/html.<br/>
*Caveat: Must have Rust nightly as default.*
