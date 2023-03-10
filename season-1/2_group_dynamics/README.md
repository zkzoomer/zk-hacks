# ZKHACK #2: Group Dynamics

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

For this problem, we are given several group elements, which are the result of performing $x \cdot G$, where $x$ is a group element and $G$ is the generator point. The order of the group, $G1$, is a natural number 

$$
n = \prod_{i=1}^{r}p_i
$$

where $p_i$ are its prime factors. The discrete logarithm problem then consists of computing $x$ from $x \cdot G$.

If these factors contain small order primes, the **Pohlig-Hellman Algorithm** allows us to efficiently solve the discrete logarithm problem.

Let's take $\bar p_i = \frac{n}{p_i}$, then (via Lagrange's theorem) $\bar p_i \cdot G$ will generate a subgroup of $G1$ which has order $p_i$. If $p_i$ is small enough, we can simply iterate over every $\alpha \in \mathbb{Z}_{p_i}$ in order to find a certain $\alpha$ such that:

$$
\alpha \cdot (\bar p_i \cdot G) = \bar p_i \cdot (x \cdot G) \rightarrow \alpha = x\ mod\ p_i = x_i
$$

Then, using these $x_i = x\ mod\ p_i$, we can efficiently compute $x\ mod\ n = x$, since $x \in \mathbb{Z}_{p_i}$, using the Chinese Remainder Theorem.

In reality, these iterations are done using the [baby-step giant-step](https://en.wikipedia.org/wiki/Baby-step_giant-step) algorithm, so each only takes $O(\sqrt{p_i})$ time. The time complexity for the whole problem therefore becomes $O(\sqrt{p})$ with $p$ being the largest prime factor of $n$. 

To mazimize computation time and thus the security of the system, we would look into maximizing this largest prime factor. But, as we will see, this is not enough to guarantee the security of the system.

## Elliptic Curves

The elliptic curve [BLS12-381](https://hackmd.io/@benjaminion/bls12-381#About-curve-BLS12-381) is defined as as:

$$
E_1 : y^2 = x^3 + 4 \text{ over } \mathbb{F_p}
$$

While its [twist](https://en.wikipedia.org/wiki/Twists_of_elliptic_curves) is defined as:

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
\displaylines{E_2: 13^2 \cdot 23^2 \cdot 2713 \cdot 11953 \cdot 262069 \cdot \\ 
52435875175126190479447740508185965837690552500527637822603658699938581184513 \cdot \\
40209603535950732159472636672046657539270680067118115942565678586877 \\
7272553337714697862511267018014931937703598282857976535744623203249}
$$

The small prime factors among these represent a vulnerability, as we see from using the Pohlig-Hellman Algorithm. Usually, we would be working on a subgroup of the curve that is of a single large prime order $r$. This is achieved by multiplying every point on the curve by a cofactor $q$, which is the product of all the other smaller prime factors of the group.

If we pick $r = 52435875175126190479447740508185965837690552500527637822603658699938581184513$, then, for the first curve:

$$
q = 3 \cdot 11^2 \cdot 10177^2 \cdot 859267^2 \cdot 52437899^2 = 76329603384216526031706109802092473003
$$

By doing this we can define these two corresponding groups:
- $G1$ is the subgroup of order $r$ of the BLS12_381 curve.
- $G2$ is the subgroup of order $r$ of the quadratic twist of the BLS12_381 curve.
Note that the order of these two has to be the same for the pairing to work.

## Solving our Discrete Logarithm Problem
We are given the set of points mentioned at the start. To solve the challenge, we wish to find $s$ by solving the discrete logarithm problem of either $s \cdot G_1$ or $s \cdot G_2$, where $G_1$, $G_2$ are the generator points for the groups $E_1$, $E_2$.

As covered, the Pohlig-Hellman Algorithm allows us to efficiently solve the discrete logarithm problem, but this efficiency is only ever practical if the prime factors $p_i$ of $n$, the order of the group, are small enough. This is because we will need to iterate through these values. For our case, the large value of $r$ (the largest prime factor), makes this solution unfeasible as it stands.

However, just as we can use the smaller prime factors of the group order to _project_ curve elements into a large prime order subgroup, we can also use this same idea to _project_ curve elements into an _unsafe_ subgroup.

Let's first find which are the prime factors of $G_1$. We can do this by using [Sage](https://www.sagemath.org/):

```python
# Constructing the field F1 as specified, with p being the prime defined above
p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F1 = GF(p)
E1 = EllipticCurve(F1,[0,4])

# Elliptic curve point corresponding to ts1_0, which is our generator point
g1 = E1(
    0x0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69, 
    0x12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5
)

# 3 * 11 * 10177 * 859267 * 52437899 * 
# 52435875175126190479447740508185965837690552500527637822603658699938581184513
print(factor(g1.order()))
```

Let's see how this _projection_ works in practice by finding the value for $s \mod p_i^1$, where $p_i^1$ is one of these prime factors of $G_1$. If we multiply $G_1$ by $\frac{n_1}{p_i^1}$, where $n_1$ is the order of the group, the order of the subgroup that we can generate when using this result as our generator point will be $p_i^1$. Note that multiplying by $\frac{n_1}{p_i^1}$ corresponds to multiplying by all the other prime factors of $G_1$ that are not $p_i^1$.

For our secret $s$, the result of $s \cdot (\frac{n_1}{p_i^1} \cdot G_1)$ will be in this subgroup of order $p_i^1$, and we also have that $s \cdot (\frac{n_1}{p_i^1} \cdot G_1) = \frac{n_1}{p_i^1} \cdot (s \cdot G_1)$.

After we have done this _projection_ for $p_i^1$, we can use the implementation of the Pohlig-Hellman algorithm provided by Sage to find $s_i^1 = s \mod p_i^1$.

We can extract more information about the secret $s$ by repeating this same procedure with the prime factors of $G_2$. From the specifications of the curve, we know that these are:

$$
\displaylines{E_2: 13 \cdot 23 \cdot 2713 \cdot 11953 \cdot 262069 \cdot \\ 
52435875175126190479447740508185965837690552500527637822603658699938581184513 \cdot \\
40209603535950732159472636672046657539270680067118115942565678586877 \\
7272553337714697862511267018014931937703598282857976535744623203249}
$$

We can do the same trick as before to _project_ curve elements into an _unsafe_ group with a single small prime factor. If we multiply $G_1$ by $\frac{n_2}{p_i^2}$, where $n_2$ is the order of the group, the order of the corresponding subgroup will be $p_i^2$.

For our secret $s$, the result of $s \cdot (\frac{n_2}{p_i^2} \cdot G_2)$ will be in this subgroup of order $p_i^2$, and we also have that $s \cdot (\frac{n_2}{p_i^2} \cdot G_2) = \frac{n_2}{p_i^2} \cdot (s \cdot G_1)$.

Now, let's turn this idea into actual code by again booting up Sage:

```python
# Constructing the fields F1 and F2 as specified, with p being the prime defined above
q = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F1 = GF(q)
R.<x> = PolynomialRing(F1)
F2.<u> = F1.extension(x^2+1)

E1 = EllipticCurve(F1,[0,4])
E2 = EllipticCurve(F2,[0,4*(1+u)])

# Order of the usual subgroup of the BLS12_381 curve
r = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001
# The big prime defined earlier
bp = 0x8D9F503DEEEB5D5C423572788BEA4D6AE0490C5AFCA1EEB2A9D75BB98B95878AFAB9C0DA5CF222C377D87384D026CD73826D177200C0D3B1

# Elliptic curve point corresponding to ts1_0, which is our generator point g1
g1 = E1(
    0x0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69, 
    0x12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5
)
# Elliptic curve point corresponding to ts1_1, which is our generator point g1 added to itself s times
sg1 = E1(
    0x16C2385B2093CC3EDBC0F2257E8F23E98E775F8F6628767E5F4FC0E495285B95B1505F487102FE083E65DC8E9E3A9181,
    0x0F4B73F63C6FD1F924EAE2982426FC94FBD03FCEE12D9FB01BAF52BE1246A14C53C152D64ED312494A2BC32C4A3E7F9A
)
# The prime factors of our g1
ord_g1 = 3 * 11 * 10177 * 859267 * 52437899 * r

# Elliptic curve point corresponding to ts2_0, which is our generator point g2
g2 = E2(
    0x1173F10AD9F2DBEE8B6C0BB2624B05D72EEC87925F5C3633E2C000E699A580B842D3F35AF1BE77517C86AEBCA1130AE4 + u*0x0434043A97DA28EF7100AE559167FC613F057B85451476ABABB27CFF0238A32831A0B4D14BA83C4F97247C8AC339841F, 
    0x0BEBEC70446CB91BB3D4DC5C8412915E99D612D8807C950AB06BC41583F528FDA9F42EC0FE7CD2991638187EF44258D3 + u*0x19528E3B5C90C73A7092BB9AFDC73F86C838F551CCD9DBBA5CC6244CF76AB3372193DBE5B62383FAAE728728D4C1E649
)
# Elliptic curve point corresponding to ts2_1, which is our generator point g2 added to itself s times
sg2 = E2(
    0x165830F15309C878BFE6DD55697860B8823C1AFBDADCC2EF3CD52B56D4956C05A099D52FE4545816830C525F5484A5FA + u*0x179E34EB67D9D2DD32B224CDBA57D4BB7CF562B4A3E33382E88F33882D91663B14738B6772BF53A24653CE1DD2BFE2FA, 
    0x150598FC4225B44437EC604204BE06A2040FD295A28230B789214B1B12BF9C9DAE6F3759447FD195E92E2B42E03B5006 + u*0x12E23B19E117418C568D4FF05B7824E5B54673C3C08D8BCD6D8D107955287A2B075100A51C81EBA44BF5A1ABAD4764A8
)
# The prime factors of our g2
ord_g2 = 13 * 23 * 2713 * 11953 * 262069 * bp * r

# Small prime factors we will be working with
# We don't use the bigger ones because those make Pohlig-Hellman unfeasible
p_g1 = [3, 11, 10177, 859267, 52437899]
p_g2 = [13, 23, 2713, 11953, 262069]

# Returns s mod p, where p must be a small factor of ord_g1
def compute_s_mod_p_g1(p):
    q = Integer(ord_g1/p)
    return discrete_log(q * sg1, q * g1, operation='+')

# Returns s mod p, where p must be a small factor of ord_g2
def compute_s_mod_p_g2(p):
    q = Integer(ord_g2/p)
    return discrete_log(q * sg2, q * g2, p, operation='+')
    
# Running the Pohlig-Hellman algorithm for all of the small factors present
s_mod_p_g1 = [compute_s_mod_p_g1(n) for n in p_g1]
s_mod_p_g2 = [compute_s_mod_p_g2(n) for n in p_g2]

# Computing via the Chinese remainder theorem what we can extract from the secret s
s_prime = crt(s_mod_p_g1 + s_mod_p_g2, p_g1 + p_g2)
# Which equals to s mod n_prime, with n_prime being the product of all these small factors
n_prime = reduce(lambda x, y: x*y, p_g1) * reduce(lambda x, y: x*y, p_g2)

# 5592216610550884993006174526481245
print(s_prime)
# 38452154918091875653578148163112927
print(n_prime)
```

After running the Pohlig-Hellman algorithm for all of these small factors, and adding all these contributions together via the Chinese remainder theorem, we now have $s' = s \mod n' = 5592216610550884993006174526481245$, where $n' = 38452154918091875653578148163112927$.

We still fall short of uncovering the full secret, as $\log_2(s') \approx 112$, while we are told that the size of $s$ is 128 bits. And we can confirm this by trying to run the assertions in the challenge: they will fail.

However, since we significantly reduced the brute-force space from 128 bits down to a measly 16 bits, we can look into simply running an exhaustive search over these remaining bits until we find the original secret. 

Since $s' = s \mod n'$, there exists some $k \in \mathbb{N}$ such that $s = k \cdot n' + s'$. And we know as well that the value for $k$ has an upper bound at around $2^{16}$. Following with a little Rust code: 

```rust
let (_ts1, _ts2) = puzzle_data();

let s_prime = Fr::from_str("5592216610550884993006174526481245").unwrap();
let n_prime = Fr::from_str("38452154918091875653578148163112927").unwrap();

for k in 0..2**16 {
  let s = n_prime*Fr::from(k) + s_prime;
  if _ts1[0].mul(s) == _ts1[1] && _ts2[0].mul(s) == _ts2[1] {
    println!("{}", s);
    return;
  }
}
```

Which gives us the following value for our secret:

$$
s = 0x56787654567876541234321012343210
$$

Or, in decimal representation:

$$
s = 114939083266787167213538091034071020048
$$

## Fixing the Problem
As we saw, this vulnerability is introduced when we leave small prime factors as part of the group order. The way this is avoided is multiplying every point on the curve by a cofactor $q$, which is the product of all the other smaller prime factors of the group. As we saw, this way we get that both the BLS12-381 curve and its twist end up with order $r$.

Trying it out
=============

Use `cargo run --release` to see it in action
