# M-Script Compiler

M-Script is a useless toy-language designed around the goal of human-readable
code at the expense of conciseness.

## Examples
```
from "io.mlib" import
start
    func print_line,
end

func main() returns None
start
    io.print_line("Hello, World");
end
```

```
func main() returns None
start
    set Integer x to 2;
    set y to 4; # Data types can be omitted. #

    set sum to x plus y;
    set difference to x minus y;
    set product to x times y;
    set quotient to x div y;
    set remainder to x mod y;
    set power to x exp y;
end
```

**THIS PROJECT IS A WIP**
