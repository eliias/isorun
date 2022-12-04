# Types

There are a couple of types that are interoperable between JavaScript and Ruby.
This list might be extended in the future. Types that nest other types (Array,
Hash, …) are also supported. There is no built-in limit, but keep in mind that
conversion of types is not for free and that you might pollute the stack when
having heavy recursion calls. 

| Ruby    | JavaScript      |
|---------|-----------------|
| nil     | null, undefined |
| bool    | Boolean         |
| String  | String          |
| Float   | Number          |
| Integer | Number          |
| FixNum  | Number          |
| Struct  | Object          |
| Array   | Array           |
| Hash    | Object          |
| Symbol  | Symbol          |
