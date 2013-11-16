% Rust 빌린 포인터 튜토리얼

# 소개

빌린 포인터는 Rust에서 사용할 수 있는 더 유연하고 강력한 도구 중에 하나이다.
빌린 포인터는 어느 곳이든 가리킬 수 있다. (관리되거나 exchange 힙으로, 스택으로, 그리고 그 외의 자료구조의 내부 조차도).
빌린 포인터는 C 포인터나 C++ 레퍼런스만큼 유연하다.
그러나, C 와 C++ 컴파일러와는 다르게, Rust 컴파일러는 프로그램이 빌린 포인터를 안전하게 사용할 수 있도록 보장하는 특별한 정적 검사를 가지고 있다.
빌린 포인터의 또다른 장점은 가비지 컬렉팅이 보이지 않는다는 것이다.
그래서 빌린 포인터로 작업하는 것은 자동 메모리 관리의 오버헤드를 감소시키는데 도움을 준다.

빌린 포인터가 완전히 안전함에도 불구하고, 런타임 중에 빌린 포인터는 C 프로그램에서의 평범한 포인터와 같다. 이것은 전혀 오버헤드가 없다. 컴파일러는 모든 안전성 검사를 컴파일 시간에 한다.

비록 빌린 포인터가 정교한 이론상의 기반(지역 포인터)은 없지만, 핵심 컨셉은 C나 C++로 일하는 누구나에게 친숙할 것이다. 
빌린 포인터가 어떻게 사용되고 제한은 어디까지 인지를 설명하기위한 최고의 방법은 여러 예제를 통해 작업해보는 것 이다.

# 예제를 통해 알아보기

빌린 포인터는 *borrowed*라고 불린다. 왜냐하면 제한된 지속기간에만 유효하기 때문이다.
빌린 포인터는 가리키는 데이터로 어떤 종류의 소유권도 절대 요구하지 않는다.
대신, 짧은 시간동안 데이터를 사용하는 상황에 사용될 것이다.

예를 들면, 단순한 구조체 타입 `Point`를 살펴보자.

~~~
struct Point {x: float, y: float}
~~~

우리는 다양한 방식으로 점을 할당하기 위해 이 단순한 정의를 사용할 수 있다.
예를 들어, 이 코드에서, 각각의 세개의 지역 변수는 점을 포함하지만, 서로 다른 장소에 할당되어 있다.

~~~
# struct Point {x: float, y: float}
let on_the_stack :  Point =  Point {x: 3.0, y: 4.0};
let managed_box  : @Point = @Point {x: 5.0, y: 1.0};
let owned_box    : ~Point = ~Point {x: 7.0, y: 9.0};
~~~

두 점 사이의 거리를 계산하는 절차를 기술하는 것을 원했다고 가정하면, 그것들이 저장되는 장소는 중요하지 않다.
예를 들어, `on_the_stack`과 `managed_box` 사이나 `managed_box`과 `owned_box` 사이의 거리를 계산해보자.
한가지 방법은 `Point` 타입의 두개의 인자를 취하는 함수를 정의하는 것이다. 즉, 값으로 점들을 취한다.
그러나 이 방법으로 정의한다면, 함수를 호출하는 것은 점들이 복사되는 것을 야기할 것이다.
점들의 입장에서는, 이것은 아마도 그닥 나쁘지는 않지만, 잦은 복사는 비용이 많이 든다.
더 안좋은건, 데이터 타입이 변할 수 있는 필드를 가지고 있으면, 복사하는 것은 당신의 프로그램의 의미를 의도하지 않는 방식으로 바꿀 수도 있다.
그래서 포인터로 점들을 취하는 함수를 정의하려 한다.
빌린 포인터를 사용해 이렇게 할 수 있다.

~~~
# struct Point {x: float, y: float}
# fn sqrt(f: float) -> float { 0f }
fn compute_distance(p1: &Point, p2: &Point) -> float {
    let x_d = p1.x - p2.x;
    let y_d = p1.y - p2.y;
    sqrt(x_d * x_d + y_d * y_d)
}
~~~

이제 우리는 `compute_distance()`를 다양한 방식으로 호출할 수 있다.

~~~
# struct Point {x: float, y: float}
# let on_the_stack :  Point =  Point{x: 3.0, y: 4.0};
# let managed_box  : @Point = @Point{x: 5.0, y: 1.0};
# let owned_box    : ~Point = ~Point{x: 7.0, y: 9.0};
# fn compute_distance(p1: &Point, p2: &Point) -> float { 0f }
compute_distance(&on_the_stack, managed_box);
compute_distance(managed_box, owned_box);
~~~

여기, `&` 연산자는 `on_the_stack` 변수의 주소를 취한다.
이는 `on_the_stack`는 `Point` 타입(즉, 구조체 값)을 가지고 값을 얻기 위해 그것의 주소를 취해야하기 때문이다.
또한 우리는 이 _borrowing_ 지역변수를 'on_the_stack' 으로 콜 할수 있다 , 그이유는 우리가 : 라는 alias 로 지정을 해놓았기 때문이다.
즉, 동일한 데이터에 대한 서로 다른 이름이다.

반대로, `managed_box`와 `owned_box`를 `compute_distance`에 직접 넘길 수 있다.
컴파일러는 자동적으로 `@Point` 또는 `~Point`와 같은 박스를 `&Point`와 같은 빌린 포인터로 변환한다.
이것은 빌림의 또 다른 형식이다.
이 경우, 호출자는 관리되거나 소유되는 박스의 내용을 피호출자에게 빌려준다.

Whenever a caller lends data to a callee, there are some limitations on what
the caller can do with the original. For example, if the contents of a
variable have been lent out, you cannot send that variable to another task. In
addition, the compiler will reject any code that might cause the borrowed
value to be freed or overwrite its component fields with values of different
types (I'll get into what kinds of actions those are shortly). This rule
should make intuitive sense: you must wait for a borrower to return the value
that you lent it (that is, wait for the borrowed pointer to go out of scope)
before you can make full use of it again.

# Other uses for the & operator

In the previous example, the value `on_the_stack` was defined like so:

~~~
# struct Point {x: float, y: float}
let on_the_stack: Point = Point {x: 3.0, y: 4.0};
~~~

This declaration means that code can only pass `Point` by value to other
functions. As a consequence, we had to explicitly take the address of
`on_the_stack` to get a borrowed pointer. Sometimes however it is more
convenient to move the & operator into the definition of `on_the_stack`:

~~~
# struct Point {x: float, y: float}
let on_the_stack2: &Point = &Point {x: 3.0, y: 4.0};
~~~

Applying `&` to an rvalue (non-assignable location) is just a convenient
shorthand for creating a temporary and taking its address. A more verbose
way to write the same code is:

~~~
# struct Point {x: float, y: float}
let tmp = Point {x: 3.0, y: 4.0};
let on_the_stack2 : &Point = &tmp;
~~~

# Taking the address of fields

As in C, the `&` operator is not limited to taking the address of
local variables. It can also take the address of fields or
individual array elements. For example, consider this type definition
for `rectangle`:

~~~
struct Point {x: float, y: float} // as before
struct Size {w: float, h: float} // as before
struct Rectangle {origin: Point, size: Size}
~~~

Now, as before, we can define rectangles in a few different ways:

~~~
# struct Point {x: float, y: float}
# struct Size {w: float, h: float} // as before
# struct Rectangle {origin: Point, size: Size}
let rect_stack   = &Rectangle {origin: Point {x: 1f, y: 2f},
                               size: Size {w: 3f, h: 4f}};
let rect_managed = @Rectangle {origin: Point {x: 3f, y: 4f},
                               size: Size {w: 3f, h: 4f}};
let rect_owned   = ~Rectangle {origin: Point {x: 5f, y: 6f},
                               size: Size {w: 3f, h: 4f}};
~~~

In each case, we can extract out individual subcomponents with the `&`
operator. For example, I could write:

~~~
# struct Point {x: float, y: float} // as before
# struct Size {w: float, h: float} // as before
# struct Rectangle {origin: Point, size: Size}
# let rect_stack  = &Rectangle {origin: Point {x: 1f, y: 2f}, size: Size {w: 3f, h: 4f}};
# let rect_managed = @Rectangle {origin: Point {x: 3f, y: 4f}, size: Size {w: 3f, h: 4f}};
# let rect_owned = ~Rectangle {origin: Point {x: 5f, y: 6f}, size: Size {w: 3f, h: 4f}};
# fn compute_distance(p1: &Point, p2: &Point) -> float { 0f }
compute_distance(&rect_stack.origin, &rect_managed.origin);
~~~

which would borrow the field `origin` from the rectangle on the stack
as well as from the managed box, and then compute the distance between them.

# Borrowing managed boxes and rooting

We’ve seen a few examples so far of borrowing heap boxes, both managed
and owned. Up till this point, we’ve glossed over issues of
safety. As stated in the introduction, at runtime a borrowed pointer
is simply a pointer, nothing more. Therefore, avoiding C's problems
with dangling pointers requires a compile-time safety check.

The basis for the check is the notion of _lifetimes_. A lifetime is a
static approximation of the span of execution during which the pointer
is valid: it always corresponds to some expression or block within the
program. Code inside that expression can use the pointer without
restrictions. But if the pointer escapes from that expression (for
example, if the expression contains an assignment expression that
assigns the pointer to a mutable field of a data structure with a
broader scope than the pointer itself), the compiler reports an
error. We'll be discussing lifetimes more in the examples to come, and
a more thorough introduction is also available.

When the `&` operator creates a borrowed pointer, the compiler must
ensure that the pointer remains valid for its entire
lifetime. Sometimes this is relatively easy, such as when taking the
address of a local variable or a field that is stored on the stack:

~~~
struct X { f: int }
fn example1() {
    let mut x = X { f: 3 };
    let y = &mut x.f;  // -+ L
    ...                //  |
}                      // -+
~~~

Here, the lifetime of the borrowed pointer `y` is simply L, the
remainder of the function body. The compiler need not do any other
work to prove that code will not free `x.f`. This is true even if the
code mutates `x`.

The situation gets more complex when borrowing data inside heap boxes:

~~~
# struct X { f: int }
fn example2() {
    let mut x = @X { f: 3 };
    let y = &x.f;      // -+ L
    ...                //  |
}                      // -+
~~~

In this example, the value `x` is a heap box, and `y` is therefore a
pointer into that heap box. Again the lifetime of `y` is L, the
remainder of the function body. But there is a crucial difference:
suppose `x` were to be reassigned during the lifetime L? If the
compiler isn't careful, the managed box could become *unrooted*, and
would therefore be subject to garbage collection. A heap box that is
unrooted is one such that no pointer values in the heap point to
it. It would violate memory safety for the box that was originally
assigned to `x` to be garbage-collected, since a non-heap
pointer *`y`* still points into it.

> ***Note:*** Our current implementation implements the garbage collector
> using reference counting and cycle detection.

For this reason, whenever an `&` expression borrows the interior of a
managed box stored in a mutable location, the compiler inserts a
temporary that ensures that the managed box remains live for the
entire lifetime. So, the above example would be compiled as if it were
written

~~~
# struct X { f: int }
fn example2() {
    let mut x = @X {f: 3};
    let x1 = x;
    let y = &x1.f;     // -+ L
    ...                //  |
}                      // -+
~~~

Now if `x` is reassigned, the pointer `y` will still remain valid. This
process is called *rooting*.

# Borrowing owned boxes

The previous example demonstrated *rooting*, the process by which the
compiler ensures that managed boxes remain live for the duration of a
borrow. Unfortunately, rooting does not work for borrows of owned
boxes, because it is not possible to have two references to a owned
box.

For owned boxes, therefore, the compiler will only allow a borrow *if
the compiler can guarantee that the owned box will not be reassigned
or moved for the lifetime of the pointer*. This does not necessarily
mean that the owned box is stored in immutable memory. For example,
the following function is legal:

~~~
# fn some_condition() -> bool { true }
# struct Foo { f: int }
fn example3() -> int {
    let mut x = ~Foo {f: 3};
    if some_condition() {
        let y = &x.f;      // -+ L
        return *y;         //  |
    }                      // -+
    x = ~Foo {f: 4};
    ...
# return 0;
}
~~~

Here, as before, the interior of the variable `x` is being borrowed
and `x` is declared as mutable. However, the compiler can prove that
`x` is not assigned anywhere in the lifetime L of the variable
`y`. Therefore, it accepts the function, even though `x` is mutable
and in fact is mutated later in the function.

It may not be clear why we are so concerned about mutating a borrowed
variable. The reason is that the runtime system frees any owned box
_as soon as its owning reference changes or goes out of
scope_. Therefore, a program like this is illegal (and would be
rejected by the compiler):

~~~ {.xfail-test}
fn example3() -> int {
    let mut x = ~X {f: 3};
    let y = &x.f;
    x = ~X {f: 4};  // Error reported here.
    *y
}
~~~

To make this clearer, consider this diagram showing the state of
memory immediately before the re-assignment of `x`:

~~~ {.notrust}
    Stack               Exchange Heap

  x +----------+
    | ~{f:int} | ----+
  y +----------+     |
    | &int     | ----+
    +----------+     |    +---------+
                     +--> |  f: 3   |
                          +---------+
~~~

Once the reassignment occurs, the memory will look like this:

~~~ {.notrust}
    Stack               Exchange Heap

  x +----------+          +---------+
    | ~{f:int} | -------> |  f: 4   |
  y +----------+          +---------+
    | &int     | ----+
    +----------+     |    +---------+
                     +--> | (freed) |
                          +---------+
~~~

Here you can see that the variable `y` still points at the old box,
which has been freed.

In fact, the compiler can apply the same kind of reasoning to any
memory that is _(uniquely) owned by the stack frame_. So we could
modify the previous example to introduce additional owned pointers
and structs, and the compiler will still be able to detect possible
mutations:

~~~ {.xfail-test}
fn example3() -> int {
    struct R { g: int }
    struct S { f: ~R }

    let mut x = ~S {f: ~R {g: 3}};
    let y = &x.f.g;
    x = ~S {f: ~R {g: 4}};  // Error reported here.
    x.f = ~R {g: 5};        // Error reported here.
    *y
}
~~~

In this case, two errors are reported, one when the variable `x` is
modified and another when `x.f` is modified. Either modification would
invalidate the pointer `y`.

# Borrowing and enums

The previous example showed that the type system forbids any borrowing
of owned boxes found in aliasable, mutable memory. This restriction
prevents pointers from pointing into freed memory. There is one other
case where the compiler must be very careful to ensure that pointers
remain valid: pointers into the interior of an `enum`.

As an example, let’s look at the following `shape` type that can
represent both rectangles and circles:

~~~
struct Point {x: float, y: float}; // as before
struct Size {w: float, h: float}; // as before
enum Shape {
    Circle(Point, float),   // origin, radius
    Rectangle(Point, Size)  // upper-left, dimensions
}
~~~

Now we might write a function to compute the area of a shape. This
function takes a borrowed pointer to a shape, to avoid the need for
copying.

~~~
# struct Point {x: float, y: float}; // as before
# struct Size {w: float, h: float}; // as before
# enum Shape {
#     Circle(Point, float),   // origin, radius
#     Rectangle(Point, Size)  // upper-left, dimensions
# }
# static tau: float = 6.28f;
fn compute_area(shape: &Shape) -> float {
    match *shape {
        Circle(_, radius) => 0.5 * tau * radius * radius,
        Rectangle(_, ref size) => size.w * size.h
    }
}
~~~

The first case matches against circles. Here, the pattern extracts the
radius from the shape variant and the action uses it to compute the
area of the circle. (Like any up-to-date engineer, we use the [tau
circle constant][tau] and not that dreadfully outdated notion of pi).

[tau]: http://www.math.utah.edu/~palais/pi.html

The second match is more interesting. Here we match against a
rectangle and extract its size: but rather than copy the `size`
struct, we use a by-reference binding to create a pointer to it. In
other words, a pattern binding like `ref size` binds the name `size`
to a pointer of type `&size` into the _interior of the enum_.

To make this more clear, let's look at a diagram of memory layout in
the case where `shape` points at a rectangle:

~~~ {.notrust}
Stack             Memory

+-------+         +---------------+
| shape | ------> | rectangle(    |
+-------+         |   {x: float,  |
| size  | -+      |    y: float}, |
+-------+  +----> |   {w: float,  |
                  |    h: float}) |
                  +---------------+
~~~

Here you can see that rectangular shapes are composed of five words of
memory. The first is a tag indicating which variant this enum is
(`rectangle`, in this case). The next two words are the `x` and `y`
fields for the point and the remaining two are the `w` and `h` fields
for the size. The binding `size` is then a pointer into the inside of
the shape.

Perhaps you can see where the danger lies: if the shape were somehow
to be reassigned, perhaps to a circle, then although the memory used
to store that shape value would still be valid, _it would have a
different type_! The following diagram shows what memory would look
like if code overwrote `shape` with a circle:

~~~ {.notrust}
Stack             Memory

+-------+         +---------------+
| shape | ------> | circle(       |
+-------+         |   {x: float,  |
| size  | -+      |    y: float}, |
+-------+  +----> |   float)      |
                  |               |
                  +---------------+
~~~

As you can see, the `size` pointer would be pointing at a `float`
instead of a struct. This is not good: dereferencing the second field
of a `float` as if it were a struct with two fields would be a memory
safety violation.

So, in fact, for every `ref` binding, the compiler will impose the
same rules as the ones we saw for borrowing the interior of a owned
box: it must be able to guarantee that the `enum` will not be
overwritten for the duration of the borrow.  In fact, the compiler
would accept the example we gave earlier. The example is safe because
the shape pointer has type `&Shape`, which means "borrowed pointer to
immutable memory containing a `shape`". If, however, the type of that
pointer were `&mut Shape`, then the ref binding would be ill-typed.
Just as with owned boxes, the compiler will permit `ref` bindings
into data owned by the stack frame even if the data are mutable,
but otherwise it requires that the data reside in immutable memory.

# Returning borrowed pointers

So far, all of the examples we have looked at, use borrowed pointers in a
“downward” direction. That is, a method or code block creates a
borrowed pointer, then uses it within the same scope. It is also
possible to return borrowed pointers as the result of a function, but
as we'll see, doing so requires some explicit annotation.

For example, we could write a subroutine like this:

~~~
struct Point {x: float, y: float}
fn get_x<'r>(p: &'r Point) -> &'r float { &p.x }
~~~

Here, the function `get_x()` returns a pointer into the structure it
was given. The type of the parameter (`&'r Point`) and return type
(`&'r float`) both use a new syntactic form that we have not seen so
far.  Here the identifier `r` names the lifetime of the pointer
explicitly. So in effect, this function declares that it takes a
pointer with lifetime `r` and returns a pointer with that same
lifetime.

In general, it is only possible to return borrowed pointers if they
are derived from a parameter to the procedure. In that case, the
pointer result will always have the same lifetime as one of the
parameters; named lifetimes indicate which parameter that
is.

In the previous examples, function parameter types did not include a
lifetime name. In those examples, the compiler simply creates a fresh
name for the lifetime automatically: that is, the lifetime name is
guaranteed to refer to a distinct lifetime from the lifetimes of all
other parameters.

Named lifetimes that appear in function signatures are conceptually
the same as the other lifetimes we have seen before, but they are a bit
abstract: they don’t refer to a specific expression within `get_x()`,
but rather to some expression within the *caller of `get_x()`*.  The
lifetime `r` is actually a kind of *lifetime parameter*: it is defined
by the caller to `get_x()`, just as the value for the parameter `p` is
defined by that caller.

In any case, whatever the lifetime of `r` is, the pointer produced by
`&p.x` always has the same lifetime as `p` itself: a pointer to a
field of a struct is valid as long as the struct is valid. Therefore,
the compiler accepts the function `get_x()`.

To emphasize this point, let’s look at a variation on the example, this
time one that does not compile:

~~~ {.xfail-test}
struct Point {x: float, y: float}
fn get_x_sh(p: @Point) -> &float {
    &p.x // Error reported here
}
~~~

Here, the function `get_x_sh()` takes a managed box as input and
returns a borrowed pointer. As before, the lifetime of the borrowed
pointer that will be returned is a parameter (specified by the
caller). That means that `get_x_sh()` promises to return a borrowed
pointer that is valid for as long as the caller would like: this is
subtly different from the first example, which promised to return a
pointer that was valid for as long as its pointer argument was valid.

Within `get_x_sh()`, we see the expression `&p.x` which takes the
address of a field of a managed box. The presence of this expression
implies that the compiler must guarantee that, so long as the
resulting pointer is valid, the managed box will not be reclaimed by
the garbage collector. But recall that `get_x_sh()` also promised to
return a pointer that was valid for as long as the caller wanted it to
be. Clearly, `get_x_sh()` is not in a position to make both of these
guarantees; in fact, it cannot guarantee that the pointer will remain
valid at all once it returns, as the parameter `p` may or may not be
live in the caller. Therefore, the compiler will report an error here.

In general, if you borrow a managed (or owned) box to create a
borrowed pointer, the pointer will only be valid within the function
and cannot be returned. This is why the typical way to return borrowed
pointers is to take borrowed pointers as input (the only other case in
which it can be legal to return a borrowed pointer is if the pointer
points at a static constant).

# Named lifetimes

Let's look at named lifetimes in more detail. Named lifetimes allow
for grouping of parameters by lifetime. For example, consider this
function:

~~~
# struct Point {x: float, y: float}; // as before
# struct Size {w: float, h: float}; // as before
# enum Shape {
#     Circle(Point, float),   // origin, radius
#     Rectangle(Point, Size)  // upper-left, dimensions
# }
# fn compute_area(shape: &Shape) -> float { 0f }
fn select<'r, T>(shape: &'r Shape, threshold: float,
                 a: &'r T, b: &'r T) -> &'r T {
    if compute_area(shape) > threshold {a} else {b}
}
~~~

This function takes three borrowed pointers and assigns each the same
lifetime `r`.  In practice, this means that, in the caller, the
lifetime `r` will be the *intersection of the lifetime of the three
region parameters*. This may be overly conservative, as in this
example:

~~~
# struct Point {x: float, y: float}; // as before
# struct Size {w: float, h: float}; // as before
# enum Shape {
#     Circle(Point, float),   // origin, radius
#     Rectangle(Point, Size)  // upper-left, dimensions
# }
# fn compute_area(shape: &Shape) -> float { 0f }
# fn select<'r, T>(shape: &Shape, threshold: float,
#                  a: &'r T, b: &'r T) -> &'r T {
#     if compute_area(shape) > threshold {a} else {b}
# }
                                                     // -+ r
fn select_based_on_unit_circle<'r, T>(               //  |-+ B
    threshold: float, a: &'r T, b: &'r T) -> &'r T { //  | |
                                                     //  | |
    let shape = Circle(Point {x: 0., y: 0.}, 1.);    //  | |
    select(&shape, threshold, a, b)                  //  | |
}                                                    //  |-+
                                                     // -+
~~~

In this call to `select()`, the lifetime of the first parameter shape
is B, the function body. Both of the second two parameters `a` and `b`
share the same lifetime, `r`, which is a lifetime parameter of
`select_based_on_unit_circle()`. The caller will infer the
intersection of these two lifetimes as the lifetime of the returned
value, and hence the return value of `select()` will be assigned a
lifetime of B. This will in turn lead to a compilation error, because
`select_based_on_unit_circle()` is supposed to return a value with the
lifetime `r`.

To address this, we can modify the definition of `select()` to
distinguish the lifetime of the first parameter from the lifetime of
the latter two. After all, the first parameter is not being
returned. Here is how the new `select()` might look:

~~~
# struct Point {x: float, y: float}; // as before
# struct Size {w: float, h: float}; // as before
# enum Shape {
#     Circle(Point, float),   // origin, radius
#     Rectangle(Point, Size)  // upper-left, dimensions
# }
# fn compute_area(shape: &Shape) -> float { 0f }
fn select<'r, 'tmp, T>(shape: &'tmp Shape, threshold: float,
                       a: &'r T, b: &'r T) -> &'r T {
    if compute_area(shape) > threshold {a} else {b}
}
~~~

Here you can see that `shape`'s lifetime is now named `tmp`. The
parameters `a`, `b`, and the return value all have the lifetime `r`.
However, since the lifetime `tmp` is not returned, it would be more
concise to just omit the named lifetime for `shape` altogether:

~~~
# struct Point {x: float, y: float}; // as before
# struct Size {w: float, h: float}; // as before
# enum Shape {
#     Circle(Point, float),   // origin, radius
#     Rectangle(Point, Size)  // upper-left, dimensions
# }
# fn compute_area(shape: &Shape) -> float { 0f }
fn select<'r, T>(shape: &Shape, threshold: float,
                 a: &'r T, b: &'r T) -> &'r T {
    if compute_area(shape) > threshold {a} else {b}
}
~~~

This is equivalent to the previous definition.

# Conclusion

So there you have it: a (relatively) brief tour of the borrowed pointer
system. For more details, we refer to the (yet to be written) reference
document on borrowed pointers, which will explain the full notation
and give more examples.
