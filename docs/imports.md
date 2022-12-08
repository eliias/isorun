# Import ECMAScript modules and members in Ruby

## Import default export

```js
export default function(name) {
    return name
}
```

### Option 1

```ruby
Isorun::Context.create do |ctx|
  default = ctx.import(:default).from(path)
  default.("Hannes") # "Hannes"
end
```

### Option 2

If no argument is provided to "import" the default is selected automatically.
As imports for the cause of side effects do not make too much sense, loading a
module without accessing any members is not supported.

```ruby
Isorun::Context.create do |ctx|
  default = ctx.import.from(path)
  default.("Hannes") # "Hannes"
end
```

## Import named exports

```js
const hello = "world";
export default hello;
```

```ruby
Isorun::Context.create do |ctx|
  hello = ctx.import(:hello).from(path)
  hello # "world"
end
```

## Import multiple exports

```js
const field1 = "field1";
const field2 = "field1";
export default field1;
```

```ruby
Isorun::Context.create do |ctx|
  default, field1, field2 = ctx.import(:default, :field1, :field2).from(path)
  default # "field1"
  field1  # "field1"
  field2  # "field2"
end
```
