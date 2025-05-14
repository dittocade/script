# Dittoscript

Dittoscript is a text-based abstraction for node-based programs. It achieves
simplicity by replacing node-based programming paradigms with familiar
text-based ones. Its syntax has similarities with Python and Go.

## Conventions

### Nodes

Nodes are similar to expressions or statements in text-based programming.

### I/O

Nodes can optionally be invoked with _inputs_ and provide _outputs_. Builtin
nodes can also be passed _options__, custom data which must be known at transpile time and cannot be changed by any
inputs. Inputs and options not passed remain set to a default value, while
unused outputs are discarded.

### Execution flow

Nodes can provide multiple _entries_ that can be called by other nodes and
_callbacks_ that will call further nodes.

## Syntax

### Invoking Nodes

Nodes can be directly invoked by their name. Inputs and options can be passed
in parentheses seperated by commata. They can be passed positionally, by name,
or left out using `_`. Positional options are passed after positional inputs.

```py
set_score(5) # Input by position
set_score(coins 3) # Input by name
set_score(_, 3) # Skip input
set_score(5, _, most_points) # Mixed inputs and options
win(delay: 20) # Option by name
```

### Assignments

Assignments replace node-based wire splitting and spaghetti code with clear
reusable labels. They work by assigning a value to a named literal to be used as label. Labels can be used as inputs in place of nodes.

```py
obj = grass()
obj = clone(obj)
_, y = touch_sensor()
y: y = touch_sensor()
```

### Callbacks

Callbacks are defined by position or name after the inputs.

```py
if(greater_than(random(), 0.5)) true: {
    win()
} false: {
    lose()
}

i = loop(_, 5) do: {
    inspect(i)
}
```

### Literal transformations

Literals might behave differently when used as inputs.

#### Numeric

Numeric literals will be transformed into number nodes.

```py
-1.5 # num(-1.5)
3.14159 # num(3.14159)
```

#### Named

Named literals will be transformed to labels for assigned values. See [assignments](#assignments).

### Defining nodes

Nodes can be defined after the `def` keyword with their outputs, name, inputs
and entries in that order.

```py
def result = lerp(from, to, amount): {
    result = add(from, multiply(subtract(to, from), amount))
}
```

## ToDo

- implement the transpiler
  - lexer & parser
- implement all node-based paradigms into text-based ones
  - recursion
  - declaring and calling callbacks
- macros
  - batch list assignments

## Disclaimer

We are not affiliated with Fancade. See
[our disclaimer](https://cade.party/disclaimer). For inquiries please
[contact us](https://cade.party/contact).
