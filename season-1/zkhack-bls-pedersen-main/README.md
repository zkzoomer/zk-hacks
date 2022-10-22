# ZKHACK #1: Let's Hash it Out

What we wish to do is try and exploit the authentication system that Alice has designed. The only information we are provided at first is the 256 signatures (conveniently chosen number, as we'll see) that were leaked, as well their corresponding hashed messages and Alice's public key (necessary to validate these signatures).

## BLS signatures

The signing scheme being used is a BLS signature (Boneh, Lynn and Shacham). This scheme relies on pairing-friendly curves (namely BLS12-381), and supports non interactive aggregation properties: given a collection of signatures anyone can produce a short signature that authenticates the entire collection. We'll see how that helps us later.

We can assume that the secret key generation was performed safely, as we are only given the corresponding public key, and the discrete logarithm problem means that it is unfeasible to recover it back:

$$
pk = [sk] g_1
$$

This secret key is what is being used to sign messages, but before we do that, it is necessary to map this message onto a point in the group $G_2$. This signature scheme is based on the wonderful properties of bilinear maps, which we get when using this two special elliptic curves.

To do this, first we need to convert the arbitrary message into a constant size. The cryptographer's favorite way of doing so is via a hash function, and for Alice this is $BLAKE2$. The corresponding digest is 256 bits, but this is still not necessarily a point in the group. The way that Alice solves this is by using the Pedersen Hash. The idea is to use this array of bits ($b_i$), as well as a certain number of fixed points ($P_i$), to arrive at a point inside the group, which we can call the hash of the original message:

$$
H(m) = \sum_i b_i \cdot P_i
$$

Now that we can map arbitrary message strings into group elements, we can calculate their signature by adding them together $sk$ times:

$$
\sigma = [sk] H(m)
$$

This signature can then be verified by the additional use of the original message, $m$, and the corresponding public key, $pk$. This is where the wonderful properties of bilinear maps come into play. This is represented in our pairing function, which takes a point $P \in G_1 \subset	E(F_q)$ and a point $Q \in G_2 \subset	E'(F_{q^2})$ and outputs a point from a group $G_T \subset F_{q^{12}}$. That is, for a pairing $e, e : G_1 \times
G_2 \rightarrow G_T$.

We can denote this operation as $e(P, Q)$. Now, let's see the very special properties that pairings have:
- $e(P, Q + R) = e(P, Q) \cdot e(P, R)$
- $e(P + S, R) = e(P, R) \cdot e(S, R)$
- $e(s \cdot A, B) = e(A, B)^s = e(A, s \cdot B)$

These properties are used to verify digital signatures, as then it holds that: 
$$
 e(pk, H(m)) = e([sk]g_1, H(m)) = e(g_1, H(m))^{(sk)} = e(g_1, [sk]H(m)) = e(g_1, \sigma) 
$$
And so a signature is valid if, and only if: 
$$
e(g_1, \sigma) = e(pk, H(m))
$$

## Forging signatures

BLS signature schemes, as well as BLAKE2 hashes, are all well battle tested, and it is well unlikely that a simple online problem is asking us to find a hash collision.

The vulnerability introduced here is in the linear combination used for the Pedersen hash. Imagine we have a 2 bit message space, and we know the signature for the following hashed messages:
- $\sigma_1 = [sk]PedersenHash(01) = [sk]P_1$
- $\sigma_2 = [sk]PedersenHash(10) = [sk]P_2$

We could easily forge a signature for the $11$ bit string by simply combining the previous two signatures:
$$
H(m) = PedersenHash(11) = P_1 + P_2
$$
$$
e(pk, H(m)) = e(pk, P_1 + P_2) = e(pk, P_1) \cdot e(pk, P_2) = e([sk]g_1,P_1) \cdot e([sk]g_1, P_2) = \\
= e(g_1, P_1)^{(sk)} \cdot e(g_1, P_2)^{(sk)} = e(g_1, [sk]P_1) \cdot e(g_1, [sk]P_2) = e(g_1, \sigma_1 + \sigma_2)
$$

So it is easy to see how one may forge a signature if they have the $P_i$ corresponding to bit arrays with a single $1$. But the question is, how do we extend this?

We are provided with 256 signatures that do not make for such neat bit constructions, but the way to forge a signature is somewhat similar. Let's assume that the preimages of the 256 Pedersen Hashes make for a base in the 256 dimension space. We can try an represent an arbitrary 256 bit vector, let's name it $x$, as a combination (using binary coefficients $b_i$) of the preimages of the ones that were leaked, name them $v_i$:
$$
x = \sum_i b_i \cdot v_i
$$
A Pedersen Hash is simply a linear combination of group elements, that either get added or not depending on the corresponding values of the bit array. We have also shown that we can represent an arbitrary 256 bit array as linear combination of the ones we have, and so, it is also possible to represent the Pedersen Hash of our message as linear combination of the leaked Pedersen Hashes: 
$$
H(m) = \sum_i \gamma_i \cdot H(m_i)
$$
How we find these coefficients is a different story, but if we can find them we can easily see how we could forge a signature:
$$
e(pk, H(m)) = e(g_1, \sigma) = e(pk, \sum_i \gamma_i \cdot H(m_i)) = \prod_i e(pk, \gamma_i \cdot H(m_i)) = \\
= \prod_i e(pk, H(m_i))^{\gamma_i} = \prod_i e(g_1, \sigma_i)^{\gamma_i} = e(g_1, \sum_i \gamma_i \cdot \sigma_i)
$$

Simply put, if we can write our our hash as a combination of the other hashes, then too we can forge the corresponding signature as the same combination of the corresponding signatures!

Note that 256 is the minimum number of bit arrays necessary to represent an arbitrary 256 bit array. A similar way to think about this are vector bases in linear algebra. It is obvious the puzzle organisers chose these 256 values at will, as one might expect the chances of 256 random vectors not being enough to represent the 256 dimensional space considerably high.

All that is left is writing this into a bit of code to create our forged signature.

## Code implementation


Trying it out
=============

Use `cargo run --release` to see it in action