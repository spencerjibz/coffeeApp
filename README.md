## Simple Coffee App 
 This commandLine app was built as a simple illustration of using rust to build command line applications
 
### Requirements for usage
 -Rust and Cargo 
 

### Installation
 Before you install, ensure that you have a stable version of rust,typically 1.3.5 and above,then use the command above in your chosen terminal .<br>
 
 ``` cargo install --path . ``` OR 
 ``` cargo build ```
 - First command installs the app globally so you can access it from 
### Features
 - its cross-platform (can be used on windows, linux and MacOS)
 -  order coffee with various packaging, different sugar levels and different coffee types
 - Colorful interface with prompts and other components
### Usage
```
    USAGE:
        Coffee App [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help     Prints this message or the help of the given subcommand(s)
        list     List the coffee menu
        order    Order a coffee

  ```
  

###  illustration

1.**list command**: <br> 
  
 ```
                COFFEE MENU
                ----------------
                Expresso   /£3.99
                Cappuchino /£3.10
                Latte      /£2.75
                Americano  /£2.00
                Mocha      /£2.50


 ```
2.**order command**: <br>
 Order coffee  with this command. 
 .<br>
```
            C:\Users\*****> coffeeapp order
            ✔ choose your coffee type · Mocha (£2.50)
            ✔ choose your sugar level · High (3)
            ✔ Do you prefer your coffee to be decaf? · no
            ✔ Do you prefer your coffee to be cold? · yes
            ✔ What  would you like your coffee to be served in  · Takeway package
            ✔ Do you prefer your coffee with a stirrer? · no
            YOUR ORDER
            -----------------------------------------------------
            CoffeeType:                   Mocha (£2.50)
            Sugar Level:                  High (3)
            Decaf:                        false
            Cold:                         true
            Served In:                    Takeway package
            with stirrer                  false
            -----------------------------------------------------
            Enjoy your order!
```

Thanks!

### License
  [MIT](https://github.com/spencerjibz/coffeeApp/blob/main/LICENSE)
