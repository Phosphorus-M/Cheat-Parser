# Cheat-Parser

## What is it?

It's a very quickly parser of [NDS Cheat Databases](https://bitbucket.org/DeadSkullzJr/nds-cheat-databases/src/master/) for [melonDS](https://github.com/Arisotura/melonDS).

It was done without time, please understand. 

## How work? 

You must pass the ROM ID to the parser and this program will search the database for cheats and return the cheats code to an output file.

## How use it?

```bash
$ cargo run -- the_id_of_the_ROM the_path_of_the_cheats.xml the_path_of_the_output.mch
```

![](https://raw.githubusercontent.com/Phosphorus-M/Cheat-Parser/master/docs/imgs/how-use-it.png)

It's important that the output file has the same name as the ROM, but with the extension `.mch`.
The input is the usrcheat.dat but in format XML, you can download it [here](https://bitbucket.org/DeadSkullzJr/nds-cheat-databases/src/master/Cheats/cheats.xml).

