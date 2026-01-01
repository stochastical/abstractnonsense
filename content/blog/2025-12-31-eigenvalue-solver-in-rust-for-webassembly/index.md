---
title: 'Writing an eigenvalue solver in Rust for WebAssembly'
date: 2025-12-31
tags: ['mathematics', 'linear-algebra', 'rust', 'webassembly']
description: |
    We build an eigenvalue solver in Rust and compile it to WebAssembly, to build an interactive demo of the Gershgorin Circle Theorem that bounds the region of the complex plane matrix eigenvalues live in.
---

I recently stumbled across the [Gershgorin Circle Theorem](https://en.wikipedia.org/wiki/Gershgorin_circle_theorem) and thought it was perfectly suited to an interactive visualisation:

{{< publish-html-as-resource >}}
{{< iframe src="component/component.html" title="Gershogrin Circle Theorem" >}}

The problem is I'm no web developer, and I don't really like JavaScript. So, the eigenvalue computation in the interactive component above is written in Rust, compiled to WebAssembly, and wrapped up with some HTML and JavaScript. As the old mathematics adage goes: *if you can't solve the base problem, turn it into multiple problems*.

Since I've been looking for a project to start learning Rust with, I figured this'd make for a great learning opportunity. In the spirit of [learning in the open](https://simonwillison.net/2022/Nov/6/what-to-blog-about/) we're going to step through how to put this together, and what I learned along the way. There'll be lots of open questions on my end - I nearly didn't publish this post as I was distracted by chasing up the answers to all my questions. But I figured it's equally important to pause and compile everything together for posterity.

> **Disclaimer**: This post and the Rust code was written by myself, in consultation with lecture notes and reference implementations from other languages. References are linked inline. An LLM was used to produce the HTML, CSS, and JavaScript code for the component. All mistakes are, as always, my own.

First, let's set ourselves up with some maths.

## The Gershgorin Circle Theorem

The Gershgorin Circle Theorem is a neat little theorem that allows you to bound the region of the complex plane that matrix eigenvalues live in, essentially just by inspection.

Let $A$ be a square matrix with complex entries. We define the Gershgorin circles as follows:

For each row $i$ of the matrix, let the <u>centre</u> $c_i$ be the diagonal element $a_{ii}$.  The <u>radius</u> $r_i = \sum_{j \neq i} |a_{ij}|$ is the sum of the norms of the *off*-diagonal elements in that row.

The Gershgorin circle![aside](fn "In mathematics, a *circle* typically refers to the one-dimensional boundary curve, whereas an open (closed) *disc* is the two-dimensional region excluding (including) the boundary. This makes the Gershgorin *Circle* Theorem a slight misnomer.") $D(c_i, r_i)$ is the set of complex numbers $z$ such that $|z - c_i| \leq r_i$.

> **Gershgorin Circle Theorem**: every eigenvalue of $A$ lies within the union of the circles $\bigcup_{i=1}^{n} D(c_i, r_i)$.

To restate the above, note that the theorem does not imply that every Gershgorin circle contains an eigenvalue, only that every eigenvalue is contained within the union of these discs. Where the matrix is real and symmetric, the eigenvalues are guaranteed to be real, so the Gershgorin circles can be further bounded to (closed) intervals on the real line.

### Examples

The following matrix

$$
\begin{bmatrix}
2 & -1 & 0 \\
1 & 1 & 0 \\
0 & 0 & 5
\end{bmatrix}
$$

has eigenvalues $\{\frac{1}{2}(3 \pm \sqrt{3}i), 5\}$ ([WolframAlpha](https://www.wolframalpha.com/input?i=eigenvalues+%5B%5B2%2C+-1%2C+0%5D%2C+%5B1%2C++1%2C+0%5D%2C+%5B0%2C+0%2C+5%5D%5D)) and Gershgorin circles: $D(2, 1), D(1, 1), D(5, 0)$. So the pair of complex conjugate eigenvalues lie *exactly* on the boundary of the first two circles $((3/2-2)^2+(\sqrt{3}/2-0)^2\le 1^2)$ and the real eigenvalue coincides directly with the degenerate Gershgorin circle $D(5, 0)$. Try it above to see what it looks like!

Or, for my [favourite class of matrices](/blog/2025-08-09-hard-problems-the-hadamard-matrix-problem), if we use a 4th-order Hadamard matrix:

$$
H_{4} = \begin{bmatrix}
1 & -1  & 1  & -1 \\
1 & -1 & -1  & 1 \\
1 & 1  & -1 & -1 \\
1 & 1 & 1 & 1
\end{bmatrix}
$$

we obtain the set of eigenvalues $\{\pm \sqrt 2 \pm \sqrt 2 i\}$ ([WolframAlpha](https://www.wolframalpha.com/input?i=eigenvalues+%5B%5B1%2C-1%2C1%2C-1%5D%2C%5B1%2C-1%2C-1%2C1%5D%2C%5B1%2C1%2C-1%2C-1%5D%2C%5B1%2C1%2C1%2C1%5D%5D)) and a pleasingly symmetric diagram.

## The QR eigenvalue algorithm

The [QR algorithm](https://en.wikipedia.org/wiki/QR_algorithm) hinges on iteratively applying a [similarity](https://en.wikipedia.org/wiki/Similar_matrix) transform induced by computing the [QR decomposition](https://en.wikipedia.org/wiki/QR_decomposition) until convergence is reached.

Given a matrix $A_k$, we compute the next matrix in the sequence $A_{k+1} = R_kQ_k$, where $A_k = Q_kR_k$ from the QR decomposition. This sequence of matrices will converge to an upper triangular matrix with the eigenvalues as the diagonal elements. It's fairly easy to implement, but it only converges under certain conditions (for instance, non-symmetric matrices aren't guaranteed to converge).

We'll walk through the Rust in more detail shortly, but my first draft looked something like this:

```rust
fn qr_algorithm(a: &Matrix, iterations: usize, tolerance: f64) -> Vec<Complex> {
    let n = a.order;
    let mut eigenvalues = vec![Complex::zero(); n];
    
    let mut a_k = a.clone();
    for _k in 0..iterations {
        let (q, r) = a_k.qr_decomposition();
        a_k = &r * &q;

        // Check for convergence: is matrix nearly upper triangular?
        let mut radius = 0.0;
        for i in 0..n {
            for j in 0..i {
                radius += a_k.get(i, j).norm();
            }
        }
        if radius < tolerance {
            break;
        }
    }

    // The diagonal elements of a_k converge to the eigenvalues of a
    for i in 0..n {
        eigenvalues[i] = a_k.get(i, i);
    }
    eigenvalues
}
```

This implementation is straightforward, but it doesn't account for arbitrary non-symmetric matrices, and it converges slowly. For instance, consider the following permutation matrix:

$$
A = \begin{bmatrix}
0 & 1 & 0 \\
0 & 0 & 1 \\
1 & 0 & 0
\end{bmatrix}
$$

with cube roots of unity eigenvalues $\{1, e^{2\pi i/3}, e^{4\pi i/3}\}$. When we perform QR decomposition on this matrix, we obtain $A = QR$ where

$$\left\{
Q = \begin{bmatrix}
0 & 1 & 0 \\
0 & 0 & 1 \\
1 & 0 & 0
\end{bmatrix} \equiv A,
R = \begin{bmatrix}
1 & 0 & 0 \\
0 & 1 & 0 \\
0 & 0 & 1
\end{bmatrix} \equiv I_3
\right\}
$$

since $A$ is already orthogonal! This means our algorithm enters a cycle and doesn't converge! So orthogonal matrices are a fixed-point of the naive iteration algorithm.

Remedying the slow convergence employs some tricks to *shift* the QR decomposition and is a nice little extension. But generalising the naive implementation requires implementing the [Hessenberg reduction](https://en.wikipedia.org/wiki/Hessenberg_matrix#Reduction_to_Hessenberg_matrix) using [Householder reflectors](https://en.wikipedia.org/wiki/Householder_transformation) and was not fun to implement.

A passing StackExchange [comment](https://math.stackexchange.com/questions/3072758/is-it-possible-to-find-complex-eigenvalues-with-qr-decomposition#comment6342889_3072758) suggested it could be possible to 'hack' around this by perturbing the matrix by a random complex unitary similarity transform. But I couldn't find much in the way of formalisation or bounds on the sensitivity of this approach, so I opted for implementing the Hessenberg reduction in the end.

These resources helped me understand the naive QR algorithm and extensions to it:
- [Computing Eigenvalues and Eigenvectors using QR Decomposition](https://www.andreinc.net/2021/01/25/computing-eigenvalues-and-eigenvectors-using-qr-decomposition)
- [Solving large scale eigenvalue problems](https://people.inf.ethz.ch/arbenz/ewp/Slides/slides4.pdf)
- [How Does A Computer Calculate Eigenvalues?](https://madrury.github.io/jekyll/update/statistics/2017/10/04/qr-algorithm.html)

## Implementing the QR algorithm in Rust

### Complex numbers

Now, many treatments of the QR algorithm focus on the real case, or [use clever tricks to avoid explicit complex arithmetic](https://www.cs.cornell.edu/~bindel/class/cs6210-f12/notes/lec26.pdf). But it was important to me to properly handle complex matrices, and getting dirty with the complex arithmetic gave me a better understanding of Rust structs.

First up, I learned that, unlike Python, Rust doesn't have any built-in complex number type in `std`/`core`, so we'll roll our own:

```rust {source="component/src/lib.rs" id="complex-struct" filename="lib.rs"}
```

and define complex arithmetic and a few other properties of complex numbers that we'll require:

```rust {source="component/src/lib.rs" id="complex-impl" filename="lib.rs"}
```

I wanted to define `Complex::ZERO` as a `pub const` value, but the compiler raised an `error: const definitions aren't supported with #[wasm_bindgen]`, so I ended up defining it as a method instead.

Also, why can't we combine a struct definition with the method implementation in one block? I'm sure there's good reason for it, but it felt strange having them as separate blocks.

I found the `hypot` method on `f64` interesting. It computes the complex norm $\left|x+yi\right| = \sqrt{x^2 + y^2}$ in a numerically stable way. Supposedly. The docstring gives some very hand-wavy caveats around the numerical precision and platform-dependence. I'm curious what `hypot` gets compiled into for WASM targets but I couldn't work this out by looking at the compiled `.wasm`.

Lastly, we define operator overloads for the basic arithmetic operations:

```rust {source="component/src/lib.rs" id="complex-ops" filename="lib.rs"}
```

### Circles

We'll also define a `Circle` type to represent our Gershgorin circles we'll be computing:

```rust {source="component/src/lib.rs" id="circle-struct" filename="lib.rs"}
```

An immediate question I was pondering is: what happens if at some point, we try to instantiate a `Circle` with a negative radius? Where is it best to defend against that? It seems like the options are either to:
1. Build a constructor for the `Circle` struct that only allows for non-negative radii, or
2. Add in `assert` calls to catch downstream abuses.

Both seem fine (and the first is often considered idiomatic in other languages), but I was hoping Rust might provide some way to guard against this at the type level.

### Matrices

To represent 2D matrices in Rust, we can use a `Vec<Complex>` in row-major order. In row-major order, elements are stored linearly in memory, with the index for position $(i, j)$ calculated as $A[i \cdot n + j]$, where $n$ is the matrix size (assuming a square matrix), and both $i$ and $j$ range from $0$ to $n-1$.

This approach sacrifices the multi-dimensional slicing available with `Vec<Vec<Complex>>`, but we can easily work around it by implementing helper `get` and `set` methods that wrap around the indexing. My understanding of WASM is that memory is allocated as linear arrays, and, well, it felt natural to represent matrices as linear arrays too.

```rust {source="component/src/lib.rs" id="matrix-struct" filename="lib.rs"}
```

We'll define a bunch of constructors and primitive methods for our `Matrix` type: 

```rust {source="component/src/lib.rs" id="matrix-impl" filename="lib.rs"}
```

I don't really like the ergonomics of the `col_dot` and `col_norm` methods, but it was the simplest way I could think of to get the job done. An alternative involved defining helper methods to extract `Matrix` columns into new `Vec<Complex>` and having functions for `norm` and `dot` defined on `&[Complex]` slices. In hindsight, I probably should've eaten the extra cost of allocations and used the latter approach, as it'd have simplified the Hessenberg reduction code too.

Ordinarily I'd be looking for more functional ways to reduce boilerplate, but as I was dipping my toe into Rust, I figured I'd eschew iterators and their ilk in favour of plain, old, boring imperative programming and classic `for` loops. But methods like `scalar_mul` are just screaming for a functional refactor. I assume there's a trait similar to Haskell's [`Functor` typeclass](https://wiki.haskell.org/index.php?title=Functor) that would allow me to [`fmap`](https://hackage-content.haskell.org/package/base-4.22.0.0/docs/Data-Functor.html#v:fmap) over the entries of the `Matrix` and apply an arbitrary function.

Next up, the usual matrix arithmetic:

```rust {source="component/src/lib.rs" id="matrix-ops" filename="lib.rs"}
```

#### Operator overloading

Initially I thought I wouldn't bother with operator overloading, but there's only so many `.mul`, `.add` method calls I could take before I cracked. Rust's implementation of operator overloading was straightforward, much like Python's `__dunder__` methods. The documentation was great here too; I could just emulate the examples. Some thoughts I should probably look into, though:

1. From what I've seen, instead of defining `scalar_mul` as a method on `Matrix`, I can *also* implement the `Mul` trait to support multiplication between `Complex` and `Matrix` types. I think this is what the `type Output = ...` helps control, though I'm still not sure I understand that syntax and how it fits inside the trait implementation.
2. Whereas in regular methods I could use `Self` instead of the struct name, for some reason the compiler errored when I tried the same for my `impl std::ops::<Op> for &Matrix` blocks.
3. As another quality of life improvement, I should really support arithmetic operations between `f64` and `Complex` types. It got very tiresome and verbose having to wrap every literal in a `Complex::real` call before using it in a computation. I avoided it as I assume I'd have a bunch of boilerplate, but in hindsight it would've cleaned up the code considerably.
4. I didn't implement any of the [`(Add|Sub|Div|Mul)Assign`](https://doc.rust-lang.org/core/ops/trait.AddAssign.html) traits, even though it'd make a bunch of code more concise. I'm sure there are good technical reasons why it's not the case, but I was surprised there was no `#[derive(AddAssign)]` macro that could be added to an implementation of `std::ops::Add` to automatically derive the `AddAssign` trait.

### Gershgorin circles

Lastly, a succinct method to compute the Gershgorin circles:

```rust {source="component/src/lib.rs" id="gershgorin_circles" filename="lib.rs"}
```

### QR decomposition

Now we're ready for the more complex algorithms. First up, here's our QR decomposition using the [Gram-Schmidt process](https://en.wikipedia.org/wiki/QR_decomposition#Using_the_Gram%E2%80%93Schmidt_process):

```rust {source="component/src/lib.rs" id="qr-decomposition" filename="lib.rs"}
```

### Householder reflectors

Householder reflectors enable us to reduce matrices into Hessenberg form, and provide a similarity transformation, so eigenvalues are preserved. To compute a Householder reflector $P=I−2uu^\top$ for a vector $\require{physics} \vectorbold x \in \mathbb{R}^n \setminus \set{0}$, we set $\DeclareMathOperator{\sign}{sign} \rho = -\sign(x_1)$ and evaluate

$$
z := (x_1 - \rho \norm{x}, x_2, \ldots, x_n).
$$

and

$$
u := \frac {z} {\norm{z}}
$$

```rust {source="component/src/lib.rs" id="householder-reflector" filename="lib.rs"}
```

#### Option types

[`Option`](https://doc.rust-lang.org/std/option/) being a first-class citizen in Rust is really handy. I used it to provide an early-exit for undefined `householder_reflector` vectors. I loved the [`let-else`](https://doc.rust-lang.org/book/ch06-03-if-let.html#staying-on-the-happy-path-with-letelse) syntactic sugar that lets you bind to the result if present, or execute a different code block otherwise:

```rust
let Some(u_k) = householder_reflector(x_k) else { continue };
```

### Hessenberg reduction

A complex matrix $H$ is Hessenberg if its elements below the first subdiagonal are zero: $h_{i,j} = 0$ where $i > j + 1$.

This was by far the ugliest algorithm to implement, and I'm still not happy with my solution. I think I could've computed the Hessenberg matrix directly using a bunch of matrix multiplications for a cleaner implementation, but every textbook reference used the below approach to avoid the additional overhead of computing the whole product naively.

These [lecture notes in numerical linear algebra](https://www.math.kth.se/na/SF2524/matber15/qrmethod.pdf) helped me define the Householder reflector and Hessenberg reduction algorithms.

```rust {source="component/src/lib.rs" id="hessenberg-reduction" filename="lib.rs"}
```

#### Macros, and debugging and testing

I thought I didn't like macros after debugging some horrific SAS macro spaghetti years ago. Variables just popped into existence willy-nilly after calling a macro, and good luck pinning down where they came from or what they represented. But Rust macros seem ... safe? sane? sound?

In particular, I really liked the `dbg!` macro, and the fact that it's actually an expression means you can be selective about what part of an expression you want to debug. I had a nasty bug in my Hessenberg reduction where I was grabbing the wrong element of `h`, and adding in some `dbg!` macros helped me pin it down. Similarly, the `#[test]` macro attribute and the neat VS Code integration really helped me test as I wrote new code!

### QR algorithm with eigenvalue shifts and deflation

Last but not least, the QR algorithm with the eigenvalue shift, deflation, and Hessenberg reduction! Our shift is the eigenvalue of the trailing 2x2 block that is closest to the diagonal element of the current matrix iterate. At each iteration, if we've converged, then we reduce the order of the matrix by one. I don't think it strictly qualifies, but it feels a little like dynamic programming.

The below code was adapted from a [Python `numpy` implementation](https://johnfoster.pge.utexas.edu/numerical-methods-book/LinearAlgebra_EigenProblem2.html#Python/NumPy-implementation-of-QR-eigenvalue-algorithm).

```rust {source="component/src/lib.rs" id="qr-algorithm" filename="lib.rs"}
```

#### The block pattern

Initially I'd written a helper function `eigenvalues_2x2` that accepted 4 complex numbers representing a matrix and returned a tuple of the eigenvalues using the closed-form for a 2x2 matrix. This is fine, but I'd just recently read [a post](https://notgull.net/block-pattern/) on Rust's block pattern and re-wrote it to leverage an inline block expression. It's only a simple change, but I think it's declarative and neat!

#### Zero-cost abstractions and Cloning, borrowing, copying

I'm not a big fan of naming intermediate variables in expressions unless they have significance of their own (e.g. the `trace` or `discriminant` in the `eigenvalues_2x2` computation). For instance, in the `householder_reflector` function, I call to `norm(&x)` twice, instead of caching the result in an intermediate variable like `let norm_x = norm(&x)`. I hope the compiler is smart enough to recognise that `norm` is idempotent and that `x` doesn't change between invocations, so it can perform common subexpression elimination. Though I wonder how this changes when targeting WASM for compilation.

I think I was overzealous in my use of cloning. I'm used to the standard `pandas` and `numpy` idiom of "return a copy, don't mutate", and I tried to replicate that in Rust. With a better understanding of the borrow checker and some hindsight, I think I'd find many ways to reduce unnecessary allocations and write more idiomatic code.

#### Default arguments

I was initially surprised to learn that Rust doesn't support default parameter values. In practice, this isn't much of a problem, and enforcing everything to be explicit is definitely friendlier for testing. Here my `qr_algorithm` function accepts a `max_iter` parameter, but suggested values (~10 iterations seemed to suffice from some heuristic testing) can just be added to the docstring.

#### Clippy, the compiler, and LSP experience

Well, I know people wax lyrical about Clippy, but I get the hype. It doesn't just catch mistakes, it gives you specific nitpicks on how to fix them! It honestly made the whole experience so much more straightforward. Similarly, the Rust LSP integration in my editor was great, and it was great having documentation available at my fingertips. I love the way mutable variables and mutating methods are underlined, and the inline type hints. I kept GitHub Copilot's Inline Suggestions completely turned off throughout; I can't stand it and think it would utterly destroy the learning experience.

Another thing I found really neat: I assumed that if I initialised a variable to a `Vec::with_capacity(n)` I'd have to declare the type of the vector. But often the explicit types 'just disappear' and the compiler can infer based on the types of the values that I'm `.push`ing in. Type safety and concision!

## Compiling Rust to WebAssembly

To compile our Rust source code into WebAssembly, I used [`wasm-pack`](https://github.com/drager/wasm-pack), which wraps around both [`wasm-bindgen`](https://github.com/wasm-bindgen/wasm-bindgen) to auto-generate JavaScript bindings for WASM-interop, and [`wasm-opt`](https://github.com/WebAssembly/binaryen?tab=readme-ov-file#wasm-opt) to reduce the final `.wasm` binary.

```shell
$ cargo new --lib gershgorin-circle-theorem
$ wasm-pack build --target web --no-typescript --no-pack
```

After a bit of renaming and tweaking, the directory tree looks like this:

```text
content/blog/eigenvalue-solver-in-rust-for-webassembly
├── index.md
└── component
    ├── Cargo.lock
    ├── Cargo.toml
    ├── component.html
    ├── pkg
    │   ├── gershgorin_circle_theorem_bg.wasm
    │   └── gershgorin_circle_theorem.js
    ├── src
    │   ├── lib.rs
    │   └── tests.rs
    └── target
```

The `index.md` file contains this blog post and is compiled into `index.html` by Hugo. The `component.html` is embedded into the generated `index.html` as an `iframe`. The `src` directory stores our Rust code, and `pkg` contains both the compiled `.wasm` and auto-generated JS bindings artifacts![*](fn "I haven't published the `tests.rs` file."). If you're interested, open the iframe above in a new tab and poke around the paths to check out the source files (or view the blog repository on GitHub).

To view the interactive component in your browser, start a web server to serve the files in the `pkg/` directory. I'm using `hugo serve` since I'm publishing it through my blog.

So how does this all work?

If you read through the `wasm-bindgen` docs, you'll notice the crate is doing a *lot* of [heavy lifting](https://rustwasm.github.io/docs/wasm-bindgen/contributing/design/index.html) to make inter-op between Rust and WebAssembly seamless. The `#[wasm_bindgen]` attribute macro exposes structs and methods to JavaScript, meaning this *just works*:

```html
<script type="module">
    import init, { Complex, Matrix } from "./pkg/gershgorin_circle_theorem.js";
    init().then(() => {
        const matrix = new Matrix(order, values);
        const circles = matrix.gershgorin_circles();
        const eigvals = matrix.qr_algorithm(10n, 1e-8);
    }
</script>
```

where the `matrix` object is the result of [parsing](https://github.com/rawify/Complex.js/blob/fe13869913d52939ba5fc23e39fb4b17835e47a9/src/complex.js#L179) an array of `Complex` objects from an HTML `<input>` element. 

This really blew my mind! Under the hood, `wasm-bindgen` is taking care of all the JavaScript boilerplate and glue for struct definitions, getters and setters for public fields, memory allocation, constructors, destructors, casting between numeric types and container data structures like `Vec` and `array`. That meant I could focus solely on writing the Rust implementation, and just annotate the blocks I wanted publicly exposed to JavaScript.

To get a glimpse at what the WASM code looks like under the hood, I found it instructive to peek at the WASM S-expressions from the decompiled `.wasm` binary. If you take a function like

```rust
pub fn gershgorin_circles(&self) -> Vec<Circle>
```

we can see it exposed in WebAssembly:

```wasm
(export "matrix_gershgorin_circles" (func 71))
;; many lines of code
(func (;71;) (type 23) (param i32) (result i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 f64 f64 f64 externref)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 11
    global.set 0
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
;; many, many more lines of code
```

These resources helped me learn enough Rust and WASM to pull this together:
- [Scientific Computing: A Rust adventure](https://lpalmieri.com/posts/2019-02-23-scientific-computing-a-rust-adventure-part-0-vectors/)
- [Implementing the Game of Life in Rust & WASM](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)
- [Mozilla Developer's Guide to Rust and WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)

### What's the future of Rust & WebAssembly?

I'm optimistic about the future of WebAssembly. As I understand it, there's a lot of ongoing discussion about future DOM-integration (which may or may not happen) and consideration to couple JavaScript and WASM more tightly to facilitate less boilerplate.

The [situation for Rust](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/) is less clear to me. For instance, the [docs](https://rustwasm.github.io/docs/wasm-pack/) for `wasm-pack` say the documentation is not maintained there, and following the links through lands you in a bit of a cycle. It seems like there are also some [open questions](https://github.com/wasm-bindgen/wasm-bindgen/issues/4634) around the future of `wasm-pack`.

With respect to size and performance, I've got no real complaints. The compiled artifacts in `pkg` come to ~84kB and the speed appears instantaneous. From what I understand, the compiled memory allocator from Rust accounts for most of the `.wasm` binary size, and there are various `wasm-opt` flags you can use to reduce the binary size further. Undoubtedly I could've achieved similar performance in pure JavaScript, but where's the fun in that?

## Publishing interactive components with Hugo

To make the interactive component available on my blog, we need to [build](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html#without-a-bundler) the `cargo` crate with `wasm-pack` to produce the `.wasm` and `.js` artifacts and host them. I also wanted the Rust source code to be available to anyone, and I didn't want it hosted in a separate repository.

The easiest approach in Hugo is to host the source files inside the `static` root directory, and then use a [Hugo shortcode](https://gohugo.io/content-management/shortcodes/) to embed the `component.html` page as an `iframe`. The downside is that the *content* of the blog post is then separated from the *code*.

The alternative I settled on is to write the blog content in a standard `index.md` page, and encapsulate the source code and build artifacts as [Page resources](https://gohugo.io/content-management/page-resources/#article) inside a folder `content/blog/eigenvalue-solver-in-rust-for-webassembly` representing a Hugo [Page bundle](https://gohugo.io/content-management/page-bundles/). The problem is that Hugo treats any `.html` file as a `page` type, and doesn't publish the resource to the `public` folder. The clunky [workaround](https://github.com/gohugoio/hugo/issues/12274#issuecomment-2640061092) is to define a shortcode `publish-html-as-resource.html` that explicitly overrides the default behaviour.

```go-html-template
{{ range .Page.Resources.Match "**.html" }}
  {{ $publishPath := urls.JoinPath $.Page.RelPermalink .Name }}
  {{ (.Content | resources.FromString $publishPath).Publish }}
{{ end }}
```

We can then directly inline an `iframe` HTML tag in the `index.md` blog file, or define a re-usable shortcode and call it:

```go-html-template
{{</* iframe 
    src="component/component.html"
    width="100%"
    height="820"
*/>}}
```

In the future I can see myself having more posts with interactive components, and I think this pattern should help with segmenting the content and source, whilst having them neighbouring. It might be nice to publish them as a component gallery on a sub-domain, but that's a todo for another day...

## Conclusion

I'm really pleased with how it all turned out. When I started this project I set myself a goal of not using any crates (other than `wasm-bindgen`), and despite some painful moments of regret, it was definitely worth battling through and implementing it all myself.

I would never have attempted this project with JS, but learning and implementing it with Rust felt much more manageable. I expected a lot more pain ensuring everything worked at the Rust-WASM boundary, but I was pleasantly surprised by how seamless it all was. Credit to `wasm-bindgen` and the broader community for doing all the heavy lifting.

### What's next?

There's a lot left to learn. For instance, I don't really understand lifetimes yet, or how the debugger works, or how to implement custom macros.

As usual, I've got too many questions and too many things I'd like to explore, and here's a list to myself so I don't forget:
1. I'd like to revisit my [Heuristics for Minimum Euclidean Skeletons](/blog/2025-11-09-heuristics-for-minimum-euclidean-skeletons.md) algorithm and implement it in Rust & WASM for an interactive web demo. My code for this project was some pretty dusty Python using [NetworkX](https://github.com/networkx/networkx), but I'm sure with hindsight it could be vastly improved.
2. I'd also like to explore what classes of matrices have interesting spectra. I could have some sort of drop-down that randomly generates such matrices (e.g. stochastic matrices, positive definite matrices, etc.) for my interactive component.
3. After writing the eigenvalue solver and visualisation, I stumbled across a post on [Gerschgorin Disks and Brauer's ovals of Cassini](https://bwlewis.github.io/cassini/). This is really cool! I love that you can get [even tighter bounds](https://planetmath.org/brauersovalstheorem) by considering pairwise interactions between the matrix's rows!
4. Can we minimise the amount of explicit JavaScript glue and implement more of the *frontend* in Rust using the [`js-sys`](https://docs.rs/js-sys/latest/js_sys/)/[`web-sys`](https://docs.rs/web-sys/latest/web_sys/) crates too? Or go even further and use a fully-fledged framework like [`Leptos`](https://github.com/leptos-rs/leptos) or [`dioxus`](https://github.com/DioxusLabs/dioxus)? In particular, I don't really love the way SVGs are constructed with JS, and it'd be nice to have a more declarative interface for it in pure Rust.
