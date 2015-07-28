Aims to be a simple interpreter for logical statements such as

```
    (and 1 0)
```

or

```
   (or (not (and 1 0 ) ) 1)
```

You get the idea

Currently implemented functions:

1) def takes a placeholder and any LVal and inserts a variable with the plaeholder as it's name in the stack frame above the one where def is called. Eg:
````
>>> (def (p y) 1)
<<< 1 
>>> y
<<< 1 
>>> (def (p def_x) (def (p x)))
<<< func(unknown)->unknown 
>>> (def_x 0)
<<< 0 
>>> x
<<< 0
````
