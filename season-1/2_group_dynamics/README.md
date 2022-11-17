# ZKHACK #1: Group Dynamics

What we wish to do is to recover the secret that Alice used for a trusted setup, as part of a Groth16 proof scheme. The information we are provided is the fact that she used a 128-bit number as her secret, $s$. We are also given two arrays of **elliptic curve points**. The first one of this containing 127 points of the $G_1$ group, while the second one contains 32 points of the $G_2$ group. We will see later that this is actually not the case, but to continue with the explanation let's assume for now these points lie in $G_1$ and $G_2$.

These points we are given are the result of performing the following operations using the secret $s$:
- $[s^i] \cdot G_1$ for $0 ⩽ i ⩽ 62$
- $[α \cdot s^i] \cdot G_1$ for $0 ⩽ i ⩽ 31$
- $[β \cdot s^i] \cdot G_1$ for $0 ⩽ i ⩽ 31$
- $[s^i] \cdot G_2$ for $0 ⩽ i ⩽ 31$

## Prime Order Subgroups
As a quick reminder, the discrete logarithm problem on a given group $G$ consists of finding an integer $k$ such that:
$$
b^k = b \cdot b \cdot \cdot \cdot b = a
$$
The group operation is also sometimes represented using additive notation, $+$, and repeated applications are described as scalar multiplication, $\cdot$. Using this different notation, the discrete logarithm problem can be represented as:
$$
b \cdot k = b + b + . . . + b = a
$$
This is the notation that was used to describe the problem, and the notation we will use for solving it. Several important algorithms in cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen groups has no efficient solution.

For this problem, we are given several group elements, which are the result of performing $x \cdot G$, where $x$ is a group element and $G$ is the generator point. The order of the group, $G1$, is a natural number $n = \prod_{i=1}^{r}p_i$, where $p_i$ are its prime factors. The discrete logarithm problem then consists of computing $x$ from $x \cdot G$.

If these factors contain small order primes, the **Pohlig-Hellman Algorithm** allows us to efficiently solve the discrete logarithm problem.

Let's take $\bar p_i := \frac{n}{p_i}$, then (via Lagrange's theorem) $\bar p_i \cdot G$ will generate a subgroup of $G1$ which has order $p_i$. If $p_i$ is small enough, we can simply iterate over every $\alpha \in \mathbb{Z}_{p_i}$ in order to find a certain $\alpha$ such that:
$$
\alpha \cdot (\bar p_i \cdot G) = \bar p_i \cdot (x \cdot G) \rightarrow \alpha = x\ mod\ p_i = x_i
$$

Then, using these $x_i = x\ mod\ p_i$, we can efficiently compute $x\ mod\ n = x$, since $x \in \mathbb{Z}_{p_i}$, using the Chinese Remainder Theorem.

In reality, these iterations are done using the [baby-step giant-step](https://en.wikipedia.org/wiki/Baby-step_giant-step) algorithm, so each only takes $O(\sqrt{p_i})$ time. The time complexity for the whole problem therefore becomes $O(\sqrt{p})$ with $p$ being the largest prime factor of $n$. It's easy then to see why it is bad to have small order subgroups.

## Elliptic Curves

The elliptic curve [_BLS12_381_](https://hackmd.io/@benjaminion/bls12-381#About-curve-BLS12-381) is defined as as:
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
E_2: 13 \cdot 23 \cdot 2713 \cdot 11953 \cdot 262069 \cdot \\ 
52435875175126190479447740508185965837690552500527637822603658699938581184513 \cdot \\
40209603535950732159472636672046657539270680067118115942565678586877 \\
7272553337714697862511267018014931937703598282857976535744623203249
$$
As covered, the small prime factors among these represent a vulnerability. Usually, we would be working on a subgroup of the curve that is of a larger prime order $r$. This is achieved by multiplying every point on the curve by a cofactor $s$, which is the product of all the smaller prime factors of the group.

If we pick $r = 52435875175126190479447740508185965837690552500527637822603658699938581184513$, then, for the first curve:
$$
s = 3 \cdot 11^2 \cdot 10177^2 \cdot 859267^2 \cdot 52437899^2 = 76329603384216526031706109802092473003
$$

By doing this we can define these two corresponding groups:
- $G1$ is the subgroup of order $r$ of the _BLS12_381_ curve.
- $G2$ is the subgroup of order $r$ of the quadratic twist of the _BLS12_381_ curve.

## Solving our Discrete Logarithm Problem
We are given the set of points mentioned at the start. To solve the challenge, we wish to find $s$ by solving the discrete logarithm problem of either $s \cdot G_1$ or $s \cdot G_2$, where $G_1$, $G_2$ are the generator points for the groups $E_1$, $E_2$.
