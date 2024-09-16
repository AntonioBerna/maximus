# maximus

```
 __  __               _              __  __           ___     __  __           _   _           ____       
|  \/  |             / \             \ \/ /          |_ _|   |  \/  |         | | | |         / ___|      
| |\/| |            / _ \             \  /            | |    | |\/| |         | | | |         \___ \      
| |  | |           / ___ \            /  \            | |    | |  | |         | |_| |          ___) |     
|_|  |_|asterful  /_/   \_\lphabet  e/_/\_\pression  |___|n  |_|  |_|ajestic   \___/ppercase  |____/tyles
```

## Why maximus?

I decided to create this tool to use the very famous `figlet` (for more informations click [here](https://github.com/cmatsuoka/figlet)) tool in a customized way.

## mini docs

First of all, open the terminal and write:

```
git clone https://github.com/AntonioBerna/maximus.git
```

now use the following command to access the project folder:

```
cd maximus
```

then just use the following command to build the project in your operating system in order to generate the executable file:

```
cargo build
```

Once this procedure is finished you can use the program using the following command:

```
cargo run
```

In this way you will receive the following message which will help you understand how you should use the program, in particular:

```
Usage: target/debug/maximus <text>
```

For example, the command `cargo run "AntonioBerna"` produce the following result:

```
    _            ____      
   / \          | __ )     
  / _ \         |  _ \     
 / ___ \        | |_) |    
/_/   \_\ntonio |____/ erna
```

finally if you want to delete the executable in a simple way you can use the following command:

```
cargo clean
```
