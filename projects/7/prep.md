# Hand-coding individual VM commands

## Pseudocode

### `push constant i`
```
RAM[SP] = i
SP++
```

### `push local i` - local, argument, this, that
```
addr = LCL + i
RAM[SP] = RAM[addr]
SP++
```

### `pop local i` - local, argument, this, that
```
SP--
val = RAM[SP]
addr = LCL + i
RAM[addr] = val;
```

### `push static i`
```
val = @Foo.i
RAM[SP] = val
SP++
```
### `pop static i`
```
val = @Foo.i
SP--
RAM[SP] = val
```
### `push temp i`
```
// assert(i < 8)
addr = RAM[5+i]
val = RAM[addr]
RAM[SP] = val
SP++
```
### `pop temp i`
```
// assert(i < 8)
addr = RAM[5+i]
SP--
val = RAM[SP]
RAM[addr] = val
```
### `push pointer i`
```
// assert(i < 2)
// reg = i ? THAT: THIS
RAM[SP] = RAM[reg]
SP++
```
### `pop pointer i`
```
// assert(i < 2)
// reg = i ? THAT: THIS
SP--
RAM[reg] = RAM[SP]
```
### Implementing arithmetic two-arg op `add`
```
SP--
// Load RAM[SP] into D
D=RAM[SP]
SP--
// Now M contains the second argument for the arithmetic op
@SP
M=D+M
// Don't forget to increment the SP!
SP++
```
### Implementing arithmetic single-arg op `not`
```
SP--
@SP
M=!M
// Don't forget to increment the SP!
SP++
```
