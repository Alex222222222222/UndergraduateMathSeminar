# Lecture Note

## Corollary 1 (Upper Bound $\theta(C_{n})$)

### Dependency

- [Lemma 1 (Circulant Matrix)](#lemma-1-circulant-matrix)
- [Theorem 1 (Upper Bound Edge Transitive)](#theorem-1-upper-bound-edge-transitive)

### Statement

For odd $n$

$$
\theta(C_{n}) = \frac{
    n\cos(\pi/n)
}{
    1+\cos(\pi/n)
}
$$

## Lemma 1 (Circulant Matrix)

### Dependency

- [Circulant matrix - Wikipedia](https://en.wikipedia.org/wiki/Circulant_matrix)

### Statement

For odd $n$, the least eigenvalue $\lambda$ of $n\times n $ matrix

$$
A = \begin{bmatrix}
1 & 1 & & & & & 1 \\
1 & 1 & 1 & & & & \\
& 1 & 1 & \ddots & & & \\
& & \ddots & \ddots & \ddots & & \\
& & & \ddots & 1 & 1 & \\
& & & & 1 & 1 & 1 \\
1 & & & & & 1 & 1 \\
\end{bmatrix}
$$

is equal to

$$
\lambda = 1 - 2 \cos(\pi/n)
$$

## Theorem 1 (Upper Bound Edge Transitive)

### Dependency

- [Theorem 2 (Upper Bound)](#theorem-2-upper-bound)
- [Theorem 3 (Spectral)](#theorem-3-spectral)
- [Theorem 4 (Minimum Largest Eigenvalue)](#theorem-4-minimum-largest-eigenvalue)
- [Theorem 5 (Orbit and Stabiliser)](#theorem-5-orbit-and-stabiliser)
- [Theorem 6 (Eigenvalue Positive Semi-definite)](#theorem-6-eigenvalue-positive-semi-definite)
- [Theorem 7 (Sum Positive Semi-definite)](#theorem-7-sum-positive-semi-definite)

### Statement

Let $G$ be a regular graph with a edge transitive automorphism group. Let eigenvalues of adjacent matrix of $G$ to be $\lambda_{1}\ge\dots\ge\lambda_{n}$. Then

$$
\theta(G) = n\frac{
    1 - \lambda_{n}
}{
    \lambda_{1} - \lambda_{n}
}
$$

### Proof

Assume $\Gamma$ is a edge transitive automorphism group of $G$ that contains permutation matrix of vertices.

Let $C$ be a matrix that met the demand of [Theorem 4 (Minimum Largest Eigenvalue)](#theorem-4-minimum-largest-eigenvalue). Let

$$
\bar{C} = \frac{1}{\left|\Gamma\right|}\sum_{P \in \Gamma} PCP^{-1}
$$

Clearly $\bar{C}$ still met the demand of [Theorem 4 (Minimum Largest Eigenvalue)](#theorem-4-minimum-largest-eigenvalue)

Also, as $\Gamma$ is edge transitive. From group theory, the number of stabiliser of any edge should be same. Then let the number of stabiliser of any edge of  $G$ be $s$. Thus, for any entry $\bar{c}_{i',j'}$ of $\bar{C}$ which $i',j'$ are adjacent and not equal.

$$
\bar{c}_{i',j'} = \frac{s}{\left|\Gamma\right|}\sum_{i,j \text{ adjacent and not equal }} c_{i,j}
$$

Let $A$ be the adjacency matrix of $G$. Then clearly there exist some $x$, such that

$$
\bar{C} = J - x(A-I)
$$

Thus, let $\lambda$ be the largest eigenvalue of $\bar{C}$. And $\lambda_{1}\ge\dots\ge\lambda_{n}$ be eigenvalues of $A$. From the proof of [Theorem 2 (Upper Bound)](#theorem-2-upper-bound)

$$
\lambda \ge n\frac{
    1 - \lambda_{n}
}{
    \lambda_{1} - \lambda_{n}
}
$$

Next, we prove $\theta(G) \ge \lambda$. So, it is sufficient to prove that $\theta(G)I-\bar{C}$ is positive semi-definite and by [Theorem 6 (Eigenvalue Positive Semi-definite)](#theorem-6-eigenvalue-positive-semi-definite) $\theta(G) \ge \lambda$.

So,

$$
\theta(G)I-\bar{C} = \frac{1}{\left|\Gamma\right|}
\sum_{P \in \Gamma} P(\theta(G)I-C)P^{-1}
$$

Clearly, $P(\theta(G)I-C)P^{-1}$ is positive semi-definite, so $\theta(G)I-\bar{C}$ is positive semi-definite.

## Theorem 2 (Upper Bound)

### Dependency

- [Theorem 4 (Minimum Largest Eigenvalue)](#theorem-4-minimum-largest-eigenvalue)

### Statement

Given any regular graph $G$, let $\lambda_{1}\ge\dots\ge\lambda_{n}$ be eigenvalues of its adjacency matrix $A$. Then

$$
\theta(G) \le n\frac{
    1 - \lambda_{n}
}{
    \lambda_{1} - \lambda_{n}
}
$$

### Proof

Clearly, the vector $j$ with all entries equal to $1$ is  an eigenvector of $A$ with eigenvalue $k$. Next, we prove that $\lambda_{1} = k$. 

Suppose, $k' > k$ is a eigenvalue of $A$ with eigenvector $v'$ . Let $a$ be the entry of $v'$ with largest absolute value and index $i$. And $(w')^{T}$ be the $i^{th}$ row of $A$. Thus

$$
\begin{aligned}
\left<w',v'\right> &= k'a \\
&= \sum_{j=0}^{n} w'_{j}v'_{j} \\
&= \sum_{w_{j}\neq0}v'_{j}
\end{aligned}
$$

Thus,

$$
\begin{aligned}
\left|\left<w',v'\right>\right| &= k'\left|a\right| \\
&= \left|\sum_{w_{j}\neq0} w'_{j}v'_{j}\right| \\
&\le \sum_{w_{j}\neq0}\left|v'_{j}\right| \\
&\le k\left|a\right|
\end{aligned}
$$

  Which contradicts $k' > k$. So, $\lambda_{1}=k$.

## Theorem 3 (Spectral)

### Dependency

- Linear Algebra Done Right

## Theorem 4 (Minimum Largest Eigenvalue)

## Theorem 5 (Orbit and Stabiliser)

## Theorem 6 (Eigenvalue Positive Semi-definite)

## Theorem 7 (Sum Positive Semi-definite)
