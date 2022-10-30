# On the Shannon Capacity of a Graph

## Definitions

adjacency matrix

independent set

> side of maximum independent set of a graph $\alpha(G)$

clique graph (完全图)

clearly $\alpha(G^{k})\ge\alpha(G)^{k}$ and $\alpha(G^{k+1})\ge\alpha(G)\alpha(G^{k})$

Shannon Capacity $\Theta(G)$

> $\Theta(G) = sup\{\alpha(G^{k})^{\frac{1}{k}}\}=\lim_{k \rightarrow \infty}\alpha(G^{k})^{\frac{1}{k}}$

ordinary capacity rate $\alpha^{*}(G)$

> assign a non nonnegative weight on $w(x)$ to vertices $x$ of $G$ such that $\sum_{x \in C}w(x) \le 1$ where $C$ is clique of $G$ the $\alpha^{*}(G)$ is defined as $\sum_{x\in G}w(x)$

k-coloration

> partition of a $G$ to k independent induced subgraph

chromatic number

edge transitivity

strong product

orthonormal representation

> to use $n$ vectors represent a $n$ order graph, where $v_{i}, v_{j}$ are orthogonal if $i$ vertex and $j$ vertex are not adjacent and not orthogonal otherwise

> every graph have a orthonormal representation, can be proved by induction

tensor product

value of an orthonormal representation

$$
\min_{c} \max_{1\le i \le n} \frac{1}{(c^{T}u_{i})^{2}}
$$

> this is well defined on the extended real number system

handle of the representation: the vector $c$ that yielding the minimum of the orthogonal representation

$\theta(G)$ denote the minimum value over all representations of $G$

a orthonormal representation is optimal if its value equal to $\theta(G)$

---

## Lemma

### Lemma 1

If $(u_{n})$ and $(v_{n})$ be orthonormal representation of $G$ and $H$ respectively. Then the vectors $u_{i}\circ v_{j}$ form an orthonormal representation of $G\cdot H$

> this prove is trivial

### Lemma 2

$$
\theta(G\cdot H) \le \theta(G)\theta(H)
$$

> this follows directly from the definition

### Lemma 2.1

$$
\theta(G\cdot H) = \theta(G)\theta(H)
$$

### Lemma 3

$$
\alpha(G) \le \theta(G)
$$

> let $(u_{n})$ be an optimal orthonormal representation of $G$ with handle $c$. Let $\{k\}$, be a maximum independent set in $G$. Then $(u_{k})$ are pair wise orthogonal, and so
> 
> $$
> 1 
= c^{2} 
\ge \sum_{i=1}^{k}(c^{T}u_{i})^{2} 
\ge \frac{\alpha(G)}{\theta(G)}
> $$
> 
> > the first inequality could be proved by changing the basis of $(u_{k})$ to standard basis

### Lemma 4

Let $(u_{1},\dots,u_{n})$ be an orthonormal representation of $G$ $(v_{1},\dots,v_{n})$ be orthonormal representation of the complementary graph $\bar{G}$. Moreover, let $c$ and $d$ be any vectors. Then

$$
\sum_{i=1}^{n}(u_{i}^{T}c)^{2}(v_{i}^{T}d)^{2} \le c^{2}d^{2}
$$

> ![ScreenShot Wed Oct 26 16:21:54 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-16-21-58-ScreenShot%20Wed%20Oct%2026%2016%3A21%3A54%20CST%202022.png)

---

## Theorem

### Theorem 1

$$
\Theta(G)\le \theta(G)
$$

> this follows from the definition of $\Theta(G)$ and lemma 1,2

### Theorem 2

$$
\Theta(C_{5}) = \sqrt{5}
$$

> ![ScreenShot Tue Oct 25 16:52:22 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/25-17-08-41-ScreenShot%20Tue%20Oct%2025%2016%3A52%3A22%20CST%202022.png)note that this not two adjacency ribs that are orthogonal but two ribs that have one rib between them are orthogonal

### Theorem 3

Let $G$ be a graph on vertices $\{1,\dots,n\}$, then $\theta(G)$ is the minimum of the largest eigenvalue of any symmetric matrix $(a_{i,j})_{i\neq j, 1\le i,j \le j}$ such that

$$
a_{i,j}=1, \text{if } i=j \text{ or if } i \text{ and } 
j \text{ are nonadjacent }
$$

> note that $a_{i,j}$ may not equal to $0$ if $i$ and $j$ are adjacent

> let the minimum of the largest eigenvalue be $\theta$
> 
> First prove that $\theta \le \theta(G)$
> 
> ![ScreenShot Tue Oct 25 20:05:04 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/25-20-06-47-ScreenShot%20Tue%20Oct%2025%2020%3A05%3A04%20CST%202022.png)
> 
> ![ScreenShot Tue Oct 25 20:06:57 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/25-20-07-05-ScreenShot%20Tue%20Oct%2025%2020%3A06%3A57%20CST%202022.png)
> 
> let $U=\{(c-\frac{u_{i}}{(c^{T}u_{i})})..\}$
> 
> Thus $\theta(G)I-A=U^{T}U+{\theta(G)-\frac{1}{(c^{T}u_{i})^{2}}}$
> 
> Thus $\theta(G)I-A$ is positive semidefinite
> 
> Next prove that $\theta \ge \theta(G)$
> 
> ![ScreenShot Wed Oct 26 10:13:53 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-10-14-00-ScreenShot%20Wed%20Oct%2026%2010%3A13%3A53%20CST%202022.png)
> 
> Here, $\delta_{i,j}$ is the entry or identity matrix. $\delta_{i,j} = e_{i,j}$
> 
> Here, as $\lambda I - A $ is symmetric and semidefinite then there exist orthogonal matrix $U$ and eigenvalue $\{\lambda_{i}\}, i=1,2,\dots,n$ and at least one of them is $0$.
> 
> $$
> \lambda I - A = U^{T}
\begin{bmatrix}
\lambda_{1} & & & & \\
 & \lambda_{2} & & & \\
& & \ddots & & \\
& & & \lambda_{n-1} & \\
& & & & \lambda_{n} \\
\end{bmatrix}
U
> $$
> 
> Let,
> 
> $$
> U^{T} = \begin{bmatrix}
y_{1} & y_{2} & \dots & y_{n}\\
\end{bmatrix}
> $$
> 
> where $y_{i}$ are $1\times n $ vectors
> 
> Then, as $\lambda_{i}$ is nonnegative 
> 
> $$
> \lambda I - A = \begin{bmatrix}
\sqrt{\lambda_{1}}y_{1} 
& \sqrt{\lambda_{2}}y_{2} 
& \dots 
& \sqrt{\lambda_{n}}y_{n}\\
\end{bmatrix}\begin{bmatrix}
\sqrt{\lambda_{1}}y_{1} \\
\sqrt{\lambda_{2}}y_{2} \\
\vdots \\
\sqrt{\lambda_{n}}y_{n}\\
\end{bmatrix}
> $$
> 
> Then let,
> 
> $$
> \begin{bmatrix}
\sqrt{\lambda_{1}}y_{1} \\
\sqrt{\lambda_{2}}y_{2} \\
\vdots \\
\sqrt{\lambda_{n}}y_{n}\\
\end{bmatrix} = \begin{bmatrix}
x_{1} 
& x_{2} 
& \dots 
& x_{n}\\
\end{bmatrix}
> $$
> 
> Thus, $\{x_{i}\}$ are the required vectors
> 
> ![ScreenShot Wed Oct 26 10:33:28 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-10-35-46-ScreenShot%20Wed%20Oct%2026%2010%3A33%3A28%20CST%202022.png)
> 
> As at least one of $\{\lambda_i{}\}$ is $0$, then $c$ must exist.
> 
> ![ScreenShot Wed Oct 26 10:38:50 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-10-43-19-ScreenShot%20Wed%20Oct%2026%2010%3A38%3A50%20CST%202022.png)

### Theorem 4

Let $G$ be a graph on the set of vertices $\{1,2,\dots,n\}$, and let $B=(b_{i,j})$ be a $n\times n $ matrix range over all positive semidefinite symmetric matrices such that 

$$
b_{i,j}=0
$$

for every pair $(i,j)$ of distinct adjacent vertices and 

$$
\mathrm{Tr}(B) = 1
$$

Then

$$
\theta(G) = \max_{B} \mathrm{Tr}(BJ)
$$

> $\mathrm{Tr}(BJ)$ is the sum of the entries in $B$.

> Let $\theta = \max_{B} \mathrm{Tr}(BJ)$.
> 
> First prove $\theta \ge \theta(G)$
> 
> ![ScreenShot Wed Oct 26 12:55:10 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-12-55-13-ScreenShot%20Wed%20Oct%2026%2012%3A55%3A10%20CST%202022.png)
> 
> ![ScreenShot Wed Oct 26 12:51:45 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-12-51-49-ScreenShot%20Wed%20Oct%2026%2012%3A51%3A45%20CST%202022.png)
> 
> Next prove that $\theta = \theta(G)$
> 
> ![ScreenShot Wed Oct 26 14:10:24 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-14-12-07-ScreenShot%20Wed%20Oct%2026%2014%3A10%3A24%20CST%202022.png)
> 
> Now define
> 
> $$
> a_{i,j} = \begin{cases}
  y + a_{k}  & \{i,j\}=\{i_{k},j_{k}\} \\
  y & n \text{otherwise}
\end{cases}
> $$
> 
> Then 
> 
> $$
> \begin{aligned}
a^{T}\hat{h}& = 
\sum_{\{i,j\}=\{i_{k},j_{k}\}} a_{k}h_{i}h_{j}

y (\sum h_{i})^{2} \\
&= y \sum_{i=1}^{n}\sum_{j=1}^{n} h_{i}h_{j}

\sum_{\{i,j\}=\{i_{k},j_{k}\}} a_{k}h_{i}h_{j} \\
&= \sum_{i=1}^{n}\sum_{j=1}^{n} a_{i,j}h_{i}h_{j} \\
&\le \alpha
\end{aligned}
> $$
> 
> 
> 
> Let $A=(\frac{a_{i,j}}{y})$ , and the largest eigenvalue of $A$ is equal to 
> 
> $$
> \max\{x^{T}Ax:\lvert x \rvert = 1\}
> $$
> 
> this implies that the largest eigenvalue of $A$ is at most $\alpha$. From Theorem 3, get $\theta(G) \le \alpha$. A contradiction. This proved the claim.
> 
> ![ScreenShot Wed Oct 26 16:14:25 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-16-14-29-ScreenShot%20Wed%20Oct%2026%2016%3A14%3A25%20CST%202022.png)
> 
> ![ScreenShot Wed Oct 26 16:14:45 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/26-16-14-48-ScreenShot%20Wed%20Oct%2026%2016%3A14%3A45%20CST%202022.png)

### Theorem 5

Let $(v_{1},\dots,v_{n})$ range over all orthonormal representations of $\bar{G}$ and $d$ over all unit vectors. Then

$$
\theta(G) = \max \sum_{i=1}^{n}(d^{T}v_{i})^{2}
$$

> ![ScreenShot Thu Oct 27 14:34:10 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/27-14-34-26-ScreenShot%20Thu%20Oct%2027%2014%3A34%3A10%20CST%202022.png)
> 
> ![ScreenShot Thu Oct 27 14:35:44 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/27-14-35-47-ScreenShot%20Thu%20Oct%2027%2014%3A35%3A44%20CST%202022.png)
> 
> ![ScreenShot Fri Oct 28 14:25:18 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-27-21-ScreenShot%20Fri%20Oct%2028%2014%3A25%3A18%20CST%202022.png)

### Theorem 6

Let $A$ range over all matrices such that $a_{i,j}=0$ if $i,=j$ are adjacent in $G$, and let $\lambda_{1}(A)\ge\dots\ge\lambda_{n}(A)$  denote the eigenvalues of $A$. Then

$$
\theta(G)=\max_{A}
\left\{1-\frac{\lambda_{1}(A)}{\lambda_{n}(A)}\right\}
$$

> ![ScreenShot Fri Oct 28 13:30:21 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-13-30-40-ScreenShot%20Fri%20Oct%2028%2013%3A30%3A21%20CST%202022.png)

### Theorem 7

$$
\theta(G\cdot H) \le \theta(G)\theta(H)
$$

> ![ScreenShot Fri Oct 28 14:08:22 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-08-27-ScreenShot%20Fri%20Oct%2028%2014%3A08%3A22%20CST%202022.png)

### Theorem 8

If $G$ has a vertex-transitive automorphism group, then

$$
\theta(G)\theta(\bar{G}) = n
$$

> ![ScreenShot Fri Oct 28 14:30:57 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-31-01-ScreenShot%20Fri%20Oct%2028%2014%3A30%3A57%20CST%202022.png)
> 
> ![ScreenShot Fri Oct 28 14:31:13 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-31-17-ScreenShot%20Fri%20Oct%2028%2014%3A31%3A13%20CST%202022.png)

### Theorem 9

![ScreenShot Sat Oct 29 13:41:01 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-13-41-07-ScreenShot%20Sat%20Oct%2029%2013%3A41%3A01%20CST%202022.png)

> > The assertion $v_{2},\dots,v_{n}$ are also eigenvectors of $J$ have some problems.
> > 
> > Let, G be
> > 
> > ![graph.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-15-46-04-graph.png)
> > 
> > Then
> > 
> > $$
> > A = \begin{bmatrix}
1 & 1 &   &   \\
1 & 1 &   &   \\
  &   & 1 & 1 \\
  &   & 1 & 1 \\
\end{bmatrix}
> > $$
> > 
> > Also $\lambda_{1}=2,\lambda_{4}=0$. So,
> > 
> > $$
> > \frac{-n\lambda_{n}}{\lambda_{1}-\lambda_{n}}=0
> > $$
> > 
> > xHowever, $\alpha(G)=2$, and $\theta(G) \ge \alpha(G)$
> > 
> > So the theorem may not be true for general cases.
> > 
> > However, the theorem may be true for $C_{n},n \text{ is odd}$
> 
> The correct theorem should be
> 
> $$
> \theta(G) \le n\frac{1-\lambda_{n}}{\lambda_{1}-\lambda_{n}}
> $$
> 
> > Proof.
> > 
> > Consider the matrix $J-x(A-I)$. This satisfies the condition of Theorem 3, and hence the largest eigenvalue of $J-x(A-I)$ is greater than $\theta(G)$. It is clear that the vector with all ones $j$ is a eigenvector of $A$ with eigenvalue $k$. Next we proof that $k$ is the largest eigenvalue of $A$, so that $k = \lambda_{1}$
> > 
> > Suppose $k' > k$ is a eigenvalue of $A$ with eigenvector $v$. Clearly $v$ is not parallel to $j$. So let the $i^{th}$ entry of $v$ be the entry $b$ with largest absolute value. Then the inner product of $i^{th}$ row $w$ of $A$ and $v$ equals to $k'b$. And also
> > 
> > $$
> > k'b = \sum_{w_{j}\neq 0}v_{j}
> > $$
> > 
> > $$
> > k'\left|
b
\right|= \left|
\sum_{w_{j}\neq 0}v_{j}
\right|
\le \sum_{w_{j}\neq 0}\left|v_{j}\right|\le
k\left|
b
\right|
> > $$
> > 
> > Which contradict to $k'>k$. So $\lambda_{1}=k$
> > 
> > Then by the proof of Spectral Theorem, there exist $n-1$ vectors $v_{1},\dots,v_{n-1}$which is the eigenvector of $A$ and orthogonal to $j$. Thus, $v_{1},\dots,v_{n-1}$ is also eigenvectors of $J$ with eigenvalue $0$. Let $\lambda_{1}',\dots,\lambda_{n-1}'$ be eigenvalues of $A$ belonging to $v_{1},\dots,v_{n-1}$.
> > 
> > So, clearly, eigenvalues of $J−x(A−I)$ should be $n-x\lambda_{1}+x,-x\lambda_{2}+x,\dots,-x\lambda_{n}+x$. Thus,
> > 
> > $$
> > \theta_{G} \le min(n-x\lambda_{1}+x,x\lambda_{n}+x)
> > $$
> > 
> > Choose, $x$ to be $\frac{n}{\lambda_{1}-\lambda_{n}}$, we get 
> > 
> > $$
> > \theta(G) 
\le n\frac{1-\lambda_{n}}{\lambda_{1}-\lambda_{n}}
> > $$
> 
> ![ScreenShot Sat Oct 29 14:37:17 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-14-37-22-ScreenShot%20Sat%20Oct%2029%2014%3A37%3A17%20CST%202022.png)

### Theorem 10

$$
\theta(G) \le \alpha^{*}(G)
$$

> ![ScreenShot Sat Oct 29 15:05:25 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-15-05-31-ScreenShot%20Sat%20Oct%2029%2015%3A05%3A25%20CST%202022.png)

### Theorem 11

Assume that $G$ admits an orthonormal representation in dimension $d$. Then

$$
\theta(G) \le d
$$

> ![ScreenShot Sat Oct 29 15:11:10 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-15-11-14-ScreenShot%20Sat%20Oct%2029%2015%3A11%3A10%20CST%202022.png)

### Theorem 12

If $G$ has a vertex-transitive automorphism group, then $\Theta(G\cdot\bar{G}=\left|V(G)\right|)$. If, in addition, $G$ is self-complementary, then $\Theta(G)=\sqrt{\left|V(G)\right|}$.

> ![ScreenShot Sat Oct 29 15:17:41 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-15-17-46-ScreenShot%20Sat%20Oct%2029%2015%3A17%3A41%20CST%202022.png)
> 
> ![ScreenShot Sat Oct 29 15:18:13 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-15-18-18-ScreenShot%20Sat%20Oct%2029%2015%3A18%3A13%20CST%202022.png)

### Theorem 13

Let $n \ge 2r$, and let the graph $K(n,r)$ be defined as the graph whose vertices are the $r-\text{subsets}$ of an $n-\text{element}$ set $S$, two subsets being adjacent if and only if they are disjoint, Then

$$
\Theta(K(n,r)) = \begin{pmatrix}
n-1 \\
r-1
\end{pmatrix}
$$

> ![ScreenShot Sat Oct 29 16:05:40 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-16-05-44-ScreenShot%20Sat%20Oct%2029%2016%3A05%3A40%20CST%202022.png)
> 
> ![ScreenShot Sat Oct 29 16:05:59 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/29-16-06-10-ScreenShot%20Sat%20Oct%2029%2016%3A05%3A59%20CST%202022.png)

---

## Corollary

### Corollary 1

If $(v_{1},\dots,v_{n})$ is an orthonormal representation of $\bar{G}$ and $d$ is any unit vector, then

$$
\theta(G) \ge \sum_{i=1}^{n} (v_{i}^{T}d)^2
$$

> This proof is trivial.

### Corollary 2

$$
\theta(G)\theta(\bar{G}) \ge n
$$

### Corollary 3

Let $\lambda_{1}\ge\dots\ge\lambda_{n}$ be the eigenvalues of the adjacency matrix of a graph $G$. Then the chromatic number of $G$ is at least

$$
1-\frac{\lambda_{1}}{\lambda_{n}}
$$

> ![ScreenShot Fri Oct 28 14:00:09 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-00-13-ScreenShot%20Fri%20Oct%2028%2014%3A00%3A09%20CST%202022.png)
> 
> ![ScreenShot Fri Oct 28 14:00:29 CST 2022.png](https://raw.githubusercontent.com/Alex222222222222/ImgBed/main/2022/10/28-14-04-09-ScreenShot%20Fri%20Oct%2028%2014%3A00%3A29%20CST%202022.png)

### Corollary 4

If $G$ has a vertex-transitive automorphism group, then

$$
\Theta(G)\Theta(\bar{G}) \le n
$$

### Corollary 5

For odd $n$,

$$
\theta(C_{n}) = \frac{
    n\cos(\pi/n)
}{
    1 + \cos(\pi/n)
}
$$

> Check eigenvalues and eigenvectors of ([Circulant matrix - Wikipedia](https://en.wikipedia.org/wiki/Circulant_matrix))