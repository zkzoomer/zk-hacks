# ZKHACK #1: Group Dynamics

What we wish to do is to recover the secret that Alice used for a trusted setup, as part of a Groth16 proof scheme. The information we are provided is the fact that she used a 128-bit number as her secret, $s$, and we are also given two arrays of **elliptic curve points**. 

These points we are given are the result of performing the following operations using the secret $s$:
- $[s^i] \cdot G_1$ for $0 ⩽ i ⩽ 62$
- $[α \cdot s^i] \cdot G_1$ for $0 ⩽ i ⩽ 31$
- $[β \cdot s^i] \cdot G_1$ for $0 ⩽ i ⩽ 31$
- $[s^i] \cdot G_2$ for $0 ⩽ i ⩽ 31$

We can see the first of these arrays is the result of adding a generator point $G_1$ onto itself a given number of times. Similarly, the second of these arrays is the result of adding a different generator point $G_2$ onto itself another given number of times. We are not given the values for $α$ and $β$, but, as we will see, this is not an issue.

Before we jump onto the solution, we will go through a quick refresher on prime order subgroups.

## Prime Order Subgroups
Let $G$ be any group, where we define the group operation by multiplication ($\cdot$) and its identity element by $1$. If we let $b$ be any element of $G$, then for any positive integer $k$ the expression $b^k$ denotes the product of $b$ with itself $k$ times.

$$
b^k = b \cdot b \cdot \cdot \cdot b
$$

The **discrete logarithm problem** is defined as finding an integer $k$ such that:
$$
b^k = b \cdot b \cdot \cdot \cdot b = a
$$

Depending on the group over which we are operating, the group operation can be represented using additive notation, $+$. Repeated applications are then described as scalar multiplication, $\cdot$. Using this different notation, the **discrete logarithm problem** can be represented as finding an integer $k$ such that:

$$
b \cdot k = b + b + . . . + b = a
$$

This is the notation that was used to describe the problem, and the notation we will use for solving it. Several important algorithms in cryptography base their security on the assumption that **the discrete logarithm problem over carefully chosen groups has no efficient solution**, the key part there being these _carefully chosen groups_.

For this problem, we are given several group elements, which are the result of performing $x \cdot G$, where $x$ is a group element and $G$ is the generator point. The order of the group, $G1$, is a natural number $n = \prod_{i=1}^{r}p_i$, where $p_i$ are its prime factors. The discrete logarithm problem then consists of computing $x$ from $x \cdot G$.

If these factors contain small order primes, the **Pohlig-Hellman Algorithm** allows us to efficiently solve the discrete logarithm problem.

Let's take $\bar p_i := \frac{n}{p_i}$, then (via Lagrange's theorem) $\bar p_i \cdot G$ will generate a subgroup of $G1$ which has order $p_i$. If $p_i$ is small enough, we can simply iterate over every $\alpha \in \mathbb{Z}_{p_i}$ in order to find a certain $\alpha$ such that:
$$
\alpha \cdot (\bar p_i \cdot G) = \bar p_i \cdot (x \cdot G) \rightarrow \alpha = x\ mod\ p_i = x_i
$$

Then, using these $x_i = x\ mod\ p_i$, we can efficiently compute $x\ mod\ n = x$, since $x \in \mathbb{Z}_{p_i}$, using the Chinese Remainder Theorem.

In reality, these iterations are done using the [baby-step giant-step](https://en.wikipedia.org/wiki/Baby-step_giant-step) algorithm, so each only takes $O(\sqrt{p_i})$ time. The time complexity for the whole problem therefore becomes $O(\sqrt{p})$ with $p$ being the largest prime factor of $n$. 

To mazimize computation time and thus the security of the system, we would look into maximizing this largest prime factor. ???

## Elliptic Curves

The elliptic curve [_BLS12-381_](https://hackmd.io/@benjaminion/bls12-381#About-curve-BLS12-381) is defined as as:
$$
E_1 : y^2 = x^3 + 4 \text{ over } \mathbb{F_p}
$$
While its twist is defined as:
$$
E_2 : y^2 = x^3 + 4\cdot(1 + u) \text{ over } \mathbb{F_{p^2}} 
$$
With $p$ being the prime:
$$
p = 4002409555221667393417789825735904156556882819939007885332 \\
058136124031650490837864442687629129015664037894272559787
$$

The orders of the corresponding groups have the following prime factors:
$$
E_1: 3 \cdot 11^2 \cdot 10177^2 \cdot 859267^2 \cdot 52437899^2 \cdot \\
52435875175126190479447740508185965837690552500527637822603658699938581184513
$$
$$
E_2: 13^2 \cdot 23^2 \cdot 2713 \cdot 11953 \cdot 262069 \cdot \\ 
52435875175126190479447740508185965837690552500527637822603658699938581184513 \cdot \\
40209603535950732159472636672046657539270680067118115942565678586877 \\
7272553337714697862511267018014931937703598282857976535744623203249
$$
The small prime factors among these represent a vulnerability, as we see from using the Pohlig-Hellman Algorithm. Usually, we would be working on a subgroup of the curve that is of a single large prime order $r$. This is achieved by multiplying every point on the curve by a cofactor $q$, which is the product of all the other smaller prime factors of the group.

If we pick $r = 52435875175126190479447740508185965837690552500527637822603658699938581184513$, then, for the first curve:
$$
q = 3 \cdot 11^2 \cdot 10177^2 \cdot 859267^2 \cdot 52437899^2 = 76329603384216526031706109802092473003
$$

By doing this we can define these two corresponding groups:
- $G1$ is the subgroup of order $r$ of the _BLS12_381_ curve.
- $G2$ is the subgroup of order $r$ of the quadratic twist of the _BLS12_381_ curve.
Note that the order of these two has to be the same for the pairing to work.

## Solving our Discrete Logarithm Problem
We are given the set of points mentioned at the start. To solve the challenge, we wish to find $s$ by solving the discrete logarithm problem of either $s \cdot G_1$ or $s \cdot G_2$, where $G_1$, $G_2$ are the generator points for the groups $E_1$, $E_2$.

As covered, the Pohlig-Hellman Algorithm allows us to efficiently solve the discrete logarithm problem, but this efficiency is only ever practical if the prime factors $p_i$ of $n$, the order of the group, are small enough. This is because we will need to iterate through these values. For our case, the large value of $r$ (the largest prime factor), makes this solution unfeasible as it stands.

However, just as we can use the smaller prime factors of the group order to _project_ curve elements into a large prime order subgroup, we can also use this same idea to _project_ curve elements into an _unsafe_ subgroup.

If we multiply $G_1$ (the generator point for $E_1$) by $r$, the order of the subgroup that we can generate when using this result as generator point is $n_1 := 3 \cdot 11 \cdot 10177 \cdot 859267 \cdot 52437899$. As such, for our secret $s$, the result of $s \cdot (r \cdot G_1)$ will be in this subgroup, and we also have that $s \cdot (r \cdot G_1) = r \cdot (s \cdot G_1)$.

After this _projection_ we can use the Pohlig-Hellman Algorithm to find $s_1 = s \mod n_1$, but we will still be missing some information about the secret $s$. We can gather more information by using $E_2$ and its generator point $G_2$.

The [BLS12-381](https://hackmd.io/@benjaminion/bls12-381) curve specifications give us the cofactor of $E_2$: the order of $E_2$ must then be $r \cdot \textrm{cofactor}$ by its definition. This is because the order of the _safe_ subgroup of $E_2$ must be the same as in $E_1$ for the pairing to work. The prime factors of this cofactor are:

$$
\textrm{cofactor} = 13^2 \cdot 23^2 \cdot 2713 \cdot 11953 \cdot 262069 \cdot \\ 40209603535950732159472636672046657539270680067118115942565678586877 \\
7272553337714697862511267018014931937703598282857976535744623203249
$$

The small prime factors among these represent a vulnerability, as covered. If we denote the last of these factors as $bp$, we can do the same trick as before to _project_ curve elements into an _unsafe_ group: $bp \cdot r \cdot G_2$ and $bp \cdot r \cdot (s\cdot G_2)$, should both be in a subgroup of order $n_2 = 13^2 \cdot 23^2 \cdot 2713 \cdot 11953 \cdot 262069$.

However, $bp \cdot r \cdot G_2$ may generate a subgroup of order smaller than $n_2$. We know that the order of $bp \cdot r \cdot G_2$ is the smallest integer $n'_2$ such that $n'_2 \cdot bp \cdot r \cdot G_2$ equals the point at infinity.

The point at infinity is the identity element of the elliptic curve arithmetic curve, and is often denoted as $\mathcal{O}$. For any point $\mathcal{P}$, we have that $\mathcal{P} + \mathcal{O} = \mathcal{O} + \mathcal{P} = \mathcal{P}$. The groups we are working with are cyclic in nature, meaning that any point is a generator, except the point at infinity as we have just seen.

We can therefore iterate over the prime factors of $n_2$ until we find this smallest integer $n'_2 = 2713 \cdot 11953 \cdot 262069$. Now, if we multiply $G_2$ by $13^2 \cdot 23 ^2 \cdot bp \cdot r$, the order of the subgroup we generate when using this result as generator point is $n'_2$. We can then define our base point to be $ B = 13^2 \cdot 23 ^2 \cdot bp \cdot r \cdot G_2$

As such, for our secret $s$, the result of $s \cdot B$, and we also have that $s \cdot B = 13^2 \cdot 23 ^2 \cdot bp \cdot r \cdot (s \cdot G_2)$. With this _projection_ we can again use the Pohlig-Hellman Algorithm to find $s_2 = s \mod n'_2$.

We now have $s_1 = s \mod n_1$ and $s_2 = s \mod n'_2$, and, since $\textrm{gcd}(n_1, n'_2) = 1$, we can apply the Chinese remainder theorem to compute $s' = s \mod (n_1 \cdot n'_2)$.

We still fall short of uncovering the full secret: $\log_2(s') \approx 105$, while we are told that the size of $s$ is 128 bits. We can then do an exhaustive search over the remaining bits: for some  $k \in \mathbb{N}, k < 2^{22}$, we have that $s = k\cdot(n_1 \cdot n'_2) + s'$.
