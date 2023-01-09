# ZKHACK #1: Let's Hash it Out

What we wish to do is try and exploit an authentication system that Alice has designed. The only information we are provided at first is the 256 signatures (conveniently chosen number, as we'll see) that were leaked, as well their corresponding hashed messages and Alice's public key (necessary to validate these signatures).

## BLS Signatures

The signing scheme being used is a BLS signature (Boneh, Lynn and Shacham). This scheme relies on pairing-friendly curves (namely [BLS12-381](https://hackmd.io/@benjaminion/bls12-381)), and supports non interactive aggregation properties: given a collection of signatures anyone can produce a short signature that authenticates the entire collection. We'll see how that helps us later.

We can assume that the secret key generation was performed safely, as we are only given the corresponding public key, and the discrete logarithm problem means that it is unfeasible to recover it back:

$$
pk = [sk] g_1
$$

This secret key is what is being used to sign messages, but before we do that, it is necessary to map this message onto a point in the group $G_2$. This signature scheme is based on the wonderful properties of bilinear maps, which we get when using these two special elliptic curves. Note that we will be using $g_i$ to represent generator points, and $G_i$ to represent the groups they generate.

To do this, first we need to convert the arbitrary message into a constant size. The cryptographer's favorite way of doing so is via a hash function, and for Alice this is the [$blake2$ function](https://www.blake2.net/). The corresponding digest is 256 bits, but this is still not necessarily a point in the group. The way that Alice solves this is by using the Pedersen Hash. The idea is to use this array of bits that is the hash ($b_i$), as well as a certain number of fixed points that are part of the group ($P_i$), to arrive at a point inside the group, which we can define as the hash of the original message:

$$
H(m) = \sum_i b_i \cdot P_i
$$

Now that we can map arbitrary message strings into group elements, we can calculate their signature by adding them together $sk$ times:

$$
\sigma = [sk] H(m)
$$

This signature can then be verified by the additional use of the original message, $m$, and the corresponding public key, $pk$. This is where the wonderful properties of bilinear maps come into play. This is represented in our pairing function, which takes a point $P \in G_1 \subset	E(F_q)$ 
and a point $Q \in G_2 \subset E'(F_{q^2})$ and outputs a point from a group $G_T \subset F_{q^{12}}$. That is, for a pairing $e, e : G_1 \times G_2 \rightarrow G_T$.

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

## Forging Signatures

BLS signature schemes, as well as blake2 hashes, are all well battle tested, and it is well unlikely that a simple online problem is asking us to find a hash collision.

The vulnerability introduced here is in the linear combination used for the Pedersen hash. Imagine we have a 2 bit message space, and we know the signature for the following hashed messages:
- $\sigma_1 = [sk]PedersenHash(01) = [sk]P_1$
- $\sigma_2 = [sk]PedersenHash(10) = [sk]P_2$

We could easily forge a signature for the $11$ bit string by simply combining the previous two signatures:

$$
H(m) = PedersenHash(11) = P_1 + P_2
$$

$$
e(pk, H(m)) = e(pk, P_1 + P_2) = e(pk, P_1) \cdot e(pk, P_2) = e([sk]g_1,P_1) \cdot e([sk]g_1, P_2) =
$$

$$
= e(g_1, P_1)^{(sk)} \cdot e(g_1, P_2)^{(sk)} = e(g_1, [sk]P_1) \cdot e(g_1, [sk]P_2) = e(g_1, \sigma_1 + \sigma_2)
$$

By extending this, we can see how one may forge a signature if they have all the points $P_i$ defining the Pedersen hash. But a random set of data is very unlikely to provide us with digests with just a single non-zero bit, so we will need a different approach.

First, we need to understand the data we have been given. We have a total of 256 messages and their corresponding signatures. But are these messages the digest of a blake2 hash, or are we dealing with its preimage (the original message)? From looking at the data we would think its the former, but running this simple bit of code:

```rs
let (pk, ms, sigs) = puzzle_data();
verify(pk, &ms[0], sigs[0]);
```

Lets us confirm that we are dealing with the actual original messages, as the function _verify_ here takes the _msg_ parameter and hashes it into the curve as specified earlier.

So now that we understand the data we are given, let's understand the problem at hand. We are provided with 256 signatures that do not make for such neat bit constructions as described earlier, but the way to forge a signature is somewhat similar. Let's assume the result of hashing these into the curve make for a perfect 256 dimensional base. We can try and represent an arbitrary 256 bit vector, let's name it $x$, as a combination (using some binary coefficients $b_i$) of the blake2 digests of the messages that were leaked, let's name them $m_i$:

$$
x = \sum_i b_i \cdot m_i
$$

A Pedersen Hash is simply a linear combination of group elements, that either get added or not depending on the corresponding values of the bit array. Let's try to represent the Pedersen Hash of our message as linear combination of the leaked Pedersen Hashes: 

$$
H(m) = \sum_i \gamma_i \cdot H(m_i)
$$

How we find these coefficients is a different story, but if we could find them we can see how we could forge a signature:

$$
e(pk, H(m)) = e(g_1, \sigma) = e(pk, \sum_i \gamma_i \cdot H(m_i)) = \prod_i e(pk, \gamma_i \cdot H(m_i)) =
$$

$$
= \prod_i e(pk, H(m_i))^{\gamma_i} = \prod_i e(g_1, \sigma_i)^{\gamma_i} = e(g_1, \sum_i \gamma_i \cdot \sigma_i)
$$

Simply put, if we can write our own blake2 hash as a combination of the other blake2 hashes, then too we can forge the corresponding signature as the same combination of the corresponding signatures! Note that 256 is the minimum number of bit arrays necessary to represent an arbitrary 256 bit array. A similar way to think about this are vector bases in linear algebra. We can suspect that the puzzle organisers chose these 256 values at will, as one might expect the chances of 256 random vectors not being enough to represent the 256 dimensional space considerably high.

But how do we find such combination? Let's recall that before signing our message, we hash it via blake2 and then map it into a group element via the Pedersen Hash. Since each message $m_i$ has such a hash, we can assemble these into a matrix:

$$
\begin{bmatrix}
b^{m_1}_{1} & b^{m_1}_{2} &\cdots & b^{m_1}_{256} \\
b^{m_2}_{1} & b^{m_2}_{2} &\cdots & b^{m_2}_{256} \\
\cdots &\cdots &\cdots&\cdots \\
b^{m_{256}}_{1} & b^{m_{256}}_{2} &\cdots & b^{m_{256}}_{256}\\
\end{bmatrix}
\begin{bmatrix}
           P_1 \\
           P_2 \\
           \vdots \\
           P_{256}
\end{bmatrix}
= \begin{bmatrix}
           H(m_1) \\
           H(m_2)\\
           \vdots \\
           H(m_{256})
\end{bmatrix}
\longleftrightarrow
\cdot \overrightarrow{P} = M \cdot \overrightarrow{H(m)} 
$$

We want to find coefficients $\gamma_i$ to represent the blake2 binary array digest of our original message, $x_i$, such that:

$$
\sum_i x_i \cdot P_i = \sum_i \gamma_i \cdot H(m_i) = \sum_i \gamma_i \cdot (\sum_j M_{ij} \cdot P_j) = \sum_j (\sum_i \gamma_i \cdot M_{ij}) \cdot P_j
$$

The coefficients $\gamma_i$ we look for must allow us to write the intermediate hash of our message as linear combination of the other intermediate hashes. Meaning our coefficients will be the result of solving the following equation:

$$
x_j = \sum_i M_{ij} \cdot \gamma_i \longleftrightarrow
\begin{bmatrix}
           x_1 \\
           x_2 \\
           \vdots \\
           x_{256}
\end{bmatrix}=
\begin{bmatrix}
b^{m_1}_{1} & b^{m_1}_{2} &\cdots & b^{m_1}_{256} \\
b^{m_2}_{1} & b^{m_2}_{2} &\cdots & b^{m_2}_{256} \\
\cdots &\cdots &\cdots&\cdots \\
b^{m_{256}}_{1} & b^{m_{256}}_{2} &\cdots & b^{m_{256}}_{256}\\
\end{bmatrix}
\begin{bmatrix}
           \gamma_1 \\
           \gamma_2 \\
           \vdots \\
           \gamma_{256}
\end{bmatrix}
\longleftrightarrow
\overrightarrow{x} = M \cdot \overrightarrow{\gamma}
$$

All that is left is writing this into a bit of code to create our forged signature.

## Code Implementation

As described above, the whole basis of our forgery is on solving this equation:

$$
\overrightarrow{x} = M \cdot \overrightarrow{\gamma}
$$

To do this, we will employ the help of [Sage](https://www.sagemath.org/), a powerful and open-source mathematics software. You can choose to install it, or run it directly from your browser via [CoCalc](https://cocalc.com/projects).

But first, we need to transform our leaked messages into the matrix $M$ described above. We can do so with this bit of rust code:

```rs
fn bytes_to_bits_string(bytes: &[u8]) -> String {
    let bits = bytes_to_bits(bytes);
    let mut s = String::with_capacity(bits.len());
    for bit in bits {
        if bit {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    return s;
}

fn write_msg_to_file(msg: &[u8], mut file: &File) {
    let blake = hash_to_curve(msg).0;
    let string = bytes_to_bits_string(&blake);
    file.write_all(string.as_ref()).unwrap();
    file.write_all(b"\n").unwrap();
}

fn main() {
    [...]
    let mut file = File::create("bit_vectors.txt").unwrap();
        println!("{}", ms.len());
        for m in ms {
        write_msg_to_file(&m, &file);
    }
    [...]
}
```
Which will save the $M$ matrix as a text file, [bit_vectors.txt](./bit_vectors.txt). To complete the equation described above, we need to specify $\overrightarrow{x}$, the binary vector resulting of hashing via blake2 our username. This is achieved via this bit of code:

```rs
let m = b"deenz";
let (_digest_hash, _digest_point) = hash_to_curve(m);
println!("{}", bytes_to_bits_string(&digest_hash));
```

In other words, a lover of sardines will see themselves represented as the following binary vector:

```
1100111011001010011000100010011011111001111111101110101100011100001001110011011100010010101000000111000111001001000100010001001101011010100101011001101011010000110000001100110010100110000111000110010110100110111010110100111000100010110010111101100011000000
```

We can now take our $M$ matrix and this binary vector, $\overrightarrow{x}$, into SageMath, and run the following program:

```python - well not really but does the trick for sage
A = list()

# The file bit_vectors.txt was defined by our Rust code
with open("bit_vectors.txt", 'r') as f:
  for line_index, line in enumerate(f):
      A.append(list())
      for bit_index in range(0, 256):
          A[line_index].append(int(line[bit_index]))

# Defining the order of the curve
P = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
F = FiniteField(P)
# Now defining this matrix over this finite field
M = Matrix(F, A)

# Binary vector resulting from blake2 hashing our username
username_hash = '1100111011001010011000100010011011111001111111101110101100011100001001110011011100010010101000000111000111001001000100010001001101011010100101011001101011010000110000001100110010100110000111000110010110100110111010110100111000100010110010111101100011000000'
username_bits = vector(F, [int(i) for i in username_hash])

# The resulting gamma coefficients from solving the equation
gamma = M.solve_left(username_bits)
gamma = vector(F, gamma)
# Printing the generator multipliers
print('[')
for item in gamma:
    content = str(hex(item))[2:]
    # Important to pad this hex string with zeroes, else rust hex::decode will not work
    print('"' + '0'*(64 - len(content)) + content + '",')
print(']')
```

This will print our $\overrightarrow{\gamma}$ coefficients. In the rust code provided, these get assigned into the `coefficient_strings` variable. All that is left is to compute the linear combination:

$$
H(m) = \sum_i \gamma_i \cdot H(m_i)
$$

Which can be done in rust via the following code:

```rs
// Transform these coefficients into field elements
let mut coeffs: Vec<ScalarField> = Vec::new();
for coeff_str in coefficient_strings {
    coeffs.push(ScalarField::from_be_bytes_mod_order(&hex::decode(coeff_str).unwrap()));
}

// Forging the signature as the described linear combination of known signatures
let mut forged_sig = G1Projective::default();
for (coeff_i, sig_i) in coeffs.iter().zip(sigs.iter()) {
    forged_sig += sig_i.mul(coeff_i.into_repr());
}

// Checking that it all works out as expected
verify(pk, m, forged_sig.into_affine());
```

And that's it! We have forged a signature! We can see the signature printed in console by simply running: 

```rs
let mut hexsig = vec![0u8; 48];
G1Affine::serialize(
    &forged_sig.into_affine(),
    hexsig.as_mut_slice(),
).unwrap();
println!("Forged signature = {:?}", hexsig.encode_hex::<String>());
```
Which will give us the following hex string:
```
d208d88420ae3706208120439e314a3dcc2937674f7139b5219af792e692d889e300929cdc919ac8e5525a429b64ed00
```

## Fixing the Problem

As [Ben Edgington](https://hackmd.io/@benjaminion/bls12-381#Hash-and-check) explains, the simplest approach to hash an arbitrary message to the curve is via the _hash-and-check_ algorithm, which can be implemented as the following: 
1. Take our message and hash it via blake2: $h \leftarrow blake2(m) \mod p$, where $m$ is the message and $p$ is the field modulus
2. If $h$ does not satisfy the curve equation (that is, there is no point in the curve with $h$ as the value for the x-coordinate), increase it by one: $h \leftarrow h + 1 \mod p$
3. Continue increasing until $h$ falls in the curve, returning the corresponding point $(x, y)$

This implementation would base its security on the collision resistance of the hash function, while the given implementation via Pedersen hashes allows for reconstruction of new and **valid** signatures using past public data.

Trying it out
=============

Use `cargo run --release` to see it in action
