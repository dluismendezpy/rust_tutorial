# Variables and Mutability

This Rust code showcases examples of variables and mutability in Rust, including shadowing and mutability.

## Shadowing

The code includes an example of shadowing, where the same variable name can be reused and changed while keeping the
variable immutable.

The example first declares a variable x and sets its initial value to 5. It then shadows the previous value of x by
declaring a new variable x and setting its value to the previous value of x plus 1.

## Mutability

The code includes an example of mutability, where the value of a variable can be changed.

The example declares a mutable variable y and sets its initial value to 5. It then changes the value of y by adding 1 to
its current value.

## Scopes

The code includes an example of variable scopes, where variables can have different values in different scopes.

The example starts a new scope and declares a new variable x within that scope. It then sets the value of x to the
previous value of x times 2. The code then prints the value of x within the inner scope and outside of the inner scope
to demonstrate how the value changes within the scope.
