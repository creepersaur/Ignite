<EstimatedTime>
	<span>READING</span>
	<SmallBullet>•</SmallBullet>
	<span>4 MIN</span>
</EstimatedTime>

# Using

---

The `using` keyword brings children (properties, methods, etc.) of an object into scope by their name, or an optional alias. This is usually done to simplify the usage of namespace functions or to properties of something to the local scope.

<br>

**Note:** The `using` keyword is local scope, which means it cannot be accessed globally.

<br>

Here's an example of exposing the `abs` function from the `Math` standard library:

```ignite
using Std::Math::abs
print(abs(-50)) // prints 50
```

We didn't have to manually write `Std::Math::abs(-50)` since it brought the abs function into scope. This is the same as

```ignite
print(Std::Math::abs(-50))
```
