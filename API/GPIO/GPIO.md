# Implementation variants
Note: maybe too much overhead

By means of a capability, e.g.:
```shell
runtime --ins 2,3,4 --outs 5,6,7 --inouts 8,9,10 cmd
```
## Global pin access
The runtime environment contains resources which hold references to pin objects which implement their own specific interfaces (for ins, outs and inouts)

In the code the programmer needs to ask the resources for specific pin numbers to get a reference to the object so that they can manipulate only those pins

```Rust
let in_resource = construct_ins_resource();

let mut pin2_input : mut & InPin = in_resource.get(2).unwrap();
```

## Functions and errors
There are no resources but only functions that accept a pin number and return an error which specifies of the function call passed through or if the pin is not allowed to be used

```Rust
match set_pin_value(2) {
	PinAllowed => { /* Pin is allowed to be used as output pin */ },
	PinNotAllowed => { /* Pin is not allowed to be used as output pin and function call changed nothing */ },
	_ => { /* Other error, perhaps lower level error */ }
}
```

