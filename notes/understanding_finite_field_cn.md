# Understanding Finite Field

## 从群到域 From group to field

### 群的定义

群是由一个集合G，以及一个二元操作 ᛫ 组成，表示为(G, ᛫)，符合以下性质：

1. 封闭性：∀ a,b ∈ G, a ᛫ b ∈ G 
2. 结合律：∀ a,b,c ∈ G, (a ᛫ b) ᛫ c = a ᛫ (b ᛫ c)
3. 单位元：∃ e ∈ G, a ᛫ e = e ᛫ a = a 
4. 逆元：∀ a ∈ G, ∃ b ∈ G, st. a ᛫ b = b ᛫ a = e

如果还符合：

5. 交换律：∀ a,b ∈ G, a ᛫ b = b ᛫ a ，则称为阿贝尔群

### 环的定义

环是由一个集合R，以及两个二元操作符 +, ᛫ 组成，表示为(R, +, ᛫)，符合以下性质：

1. (R, +) 是一个阿贝尔群
2. R在 ᛫ 运算下符合结合律：∀ a,b,c ∈ G, (a · b) · c = a · (b · c)
3. R在 ᛫ 运算下存在单位元：∃ e ∈ G, a ᛫ e = e ᛫ a = a 
4. ᛫ 运算对 + 运算符合分配律：∀ a,b,c ∈ G, a ⋅ (b + c) = (a · b) + (a · c) 

举例：

1. (Z, +, ᛫)是一个环，整数没有乘法逆元；
2. 多项式环

### 域的定义

域是有一个集合F，以及两个二元操作符 +, ᛫ 组成，表示为(F, +, ᛫)，符合以下性质：

1. (F, +) 是一个阿贝尔群
2. (F - {0}, ᛫) 是一个阿贝尔群
3. ᛫ 运算对 + 运算符合分配律：∀ a,b,c ∈ G, a ⋅ (b + c) = (a · b) + (a · c) 

## 模n加法群 Additive group of integers modulo n

一个简单的群是整数加法群(Z, +)，符合群定义四条性质，它是一个无限群。如果将加法换成模n加法，得到一个有限群，G = Zn = {0, 1, 2, .., n-1}，它满足结合律、封闭性，单位元是0，每个元素都存在逆元。

### 循环群 Cyclic group

对任意元素 g ∈ G，&lt; g&gt; = {g^k | k ∈ Z}，称作子群，g是该子群的生成元，也叫做模n原根，子群的元素个数叫做群的阶。

当存在g，使得G = &lt; g&gt; 时，则G叫做循环群。 

<!-- 阶为n的循环群可表示为 G = {e, g^1, g^2, .., g^n-1}。 -->
阶为n的循环群可表示为 G = 
<img src="https://render.githubusercontent.com/render/math?math=\{e, g^1, g^2, .., g^{n-1} \}">，当群运算时+时，g^2表示g+g；当群运算是·时，g^2表示g·g。

###  素数阶群 Group of prime order

在模n加法群中，当n = p为素数时，此时 G = {0, 1, 2, 3, .., n-1}，阶为φ(n) = n。此时它是一个循环群，生成元是任意非0元素，共n-1个；

## 模n乘法群 Multiplicative group of integers modulo n

group of primitive residue classes modulo n

我们知道非零整数集合在乘法运算下构成阿贝尔群，(Z-{0}, ᛫)，而正整数集合在乘法运算下(Z+, ᛫)无法构成一个群，因为不存在逆元。

但在模n乘法运算下，集合取 G = {x | gcd(x, n) = 1, x ∈ Zn} 其中Zn = {0, 1, 2, .., n-1}，可以保证 ∀ a ∈ G，∃ b ∈ G, st.a ᛫ b = b ᛫ a = e = 1.

证明：由裴蜀等式(Bézout's identity)，关于x, y的线性丢番图方程：ax + ny = gcd(a, n)，必有解，且gcd(a, n)是ax + by能够表达的最小的正整数。当a, n互质时，ax + ny = 1，ax = 1 mod n，其中x即a模n乘法运算下的逆元。

### 求解乘法逆元

1. 拓展欧几里得算法 Extended euclidean algorithm 

    欧几里得算法是求解gcd(a, n)的一种算法，也称作辗转相除法。它基于以下观察：
    对 n = a * q + r，q是商，r是余数，可证：
    gcd(a, n) = gcd(r, a)

    计算过程[参考](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)。

2. 费马小定理 Fermat’s little theorem 

    当gcd(a,p) = 1，且p是素数时，

    <img src="https://render.githubusercontent.com/render/math?math=a^{p-1} \equiv 1  \mod n \Rightarrow  a \cdot a^{p-2} \equiv 1  \mod n"> </br> 

    a模n的乘法逆元为 <img src="https://render.githubusercontent.com/render/math?math=a^{p-2}">，此时可以用快速幂(fast powering algo)计算出来。

    

### 模n乘法群的元素个数

模n乘法群的元素个数由欧拉函数(Euler totient function)给出，φ(n)表示{1, 2, .., n-1}中与n互质的元素个数。

<img src="https://render.githubusercontent.com/render/math?math=\varphi (n)=n\prod _{p\mid n}\left(1-{\frac {1}{p}}\right)=n\left({\frac {p_{1}-1}{p_{1}}}\right)\left({\frac {p_{2}-1}{p_{2}}}\right)\cdots \left({\frac {p_{\omega (n)}-1}{p_{\omega (n)}}}\right)">

其中p表示n分解出来的不相同的素数，证明[参考](https://en.wikipedia.org/wiki/Euler%27s_totient_function#Proof_of_Euler's_product_formula)。一些有用的结论： 

1. 当n是素数时，φ(n) = n-1；

2. 当n是两个素数乘积时，φ(n) = φ(pq) = φ(p)φ(q) = (p-1)(q-1)；

欧拉定理(Euler's theorem)：

<img src="https://render.githubusercontent.com/render/math?math=a^{\Phi(n)} \equiv 1 \mod n">

可以看作是费马小定理的推广，证明[参考](https://en.wikipedia.org/wiki/Euler%27s_theorem#Proofs)。

### 模n乘法群成为循环群的条件

模n乘法群不一定是一个循环群，当n = 1, 2, 4, p^k, 或 2p^k时，它是循环群，p是素数。群的阶为φ(n)，而生成元的个数为φ(φ(n))。证明[参考](https://math.stackexchange.com/questions/2155137/cyclic-group-generators-of-order-n)。

## 有限域 Finite field

### 素数域 Prime field

最简单的有限域是阶为素数的域(GF(p), 模n加法, 模n乘法)，GF(p) = {0, 1, 2, .., p-1} ，也可写成
<img src="https://render.githubusercontent.com/render/math?math=\mathbb{F}_{p}">。

检查下它的性质：

1. (GF(p), +)是模n加法群，也是阿贝尔群；
2. (GF(p) - {0}, ᛫)是模n乘法群，也是阿贝尔群；
3. ᛫ 对 + 满足分配律

如果n不是素数，设n = ab，则 a᛫b = 0 mod n，不满足性质2。

子域是指GF(q) ⊆ GF(p)，(GF(q), +, ᛫)也满足域的定义。当p是素数时，可以证明GF(q)和GF(p)是同构的。

域的特征(characteristic)：指满足 1 + 1 + ..+ 1 (共n个1) = 0 的最小整数n。在素数域上即是p，且∀ x ∈ GF(p)，均有px = 0。此时有等式

<img src="https://render.githubusercontent.com/render/math?math=(x %2B y)^{p}=x^{p} %2B y^{p}">。

### 素数幂域 Prime power field

另一种常见的有限域是素数幂域，表示为GF(p^n)，其中p是素数，n>1，p^n是有限域的元素个数。比起素数域，它的构造要复杂很多。

### 多项式环 Polynomial ring

常见的多项式形如：

<img src="https://render.githubusercontent.com/render/math?math=f(x) = f_0  %2B f_1x %2B f_2x^2 %2B \cdots %2B f_nx^n">

## 有限域上的多项式 Polynomials over finite field

### 有限域上的多项式插值 Polynomial interplation over finite field

## 基于群的密码学 Group-based cryptography

RSA 

## 基于域的密码学 Field-based cryptography

AES 

ECDSA

## 更多应用

### 多项式插值

### 里德-所罗门码 Reed-Solomon codes

## 附录

### 求解公因子 GCD 

#### (拓展)欧几里得算法

1. 迭代

``` 
function egcd(a, b)
    s := 0;    old_s := 1
    t := 1;    old_t := 0
    r := b;    old_r := a
    
    while r ≠ 0 do
        quotient := old_r div r
        (old_r, r) := (r, old_r − quotient × r)
        (old_s, s) := (s, old_s − quotient × s)
        (old_t, t) := (t, old_t − quotient × t)
    
    output "Bézout coefficients:", (old_s, old_t)
    output "greatest common divisor:", old_r
    output "quotients by the gcd:", (t, s)
```

2. 递归

``` 
function egcd_recursive(a,b)
    if b = 0 do
        (a, 1, 0)
    else
        quotient = a/b;
        rem = a%b;
        r = egcd_recursive(b,rem);
        (r[0],r[2],r[1]-r[2]*quotient)

    (gcd, x, y) = egcd_recursive(a,b)
    output "Bézout coefficients:", (x, y)
    output "greatest common divisor:" gcd
```

一点分析：这里不是尾递归，r[0]就是gcd，从最深的栈到返回都不变，整个过程会构造出x, y系数来。
当递归调用了4次结束，参考迭代调用算法:

``` 
s_0 = 1
s_1 = 0
s_2 = s_0 - s_1 * q_1
s_3 = s_1 - s_2 * q_2
s_4 = s_2 - s_3 * q_3
```

todo 

#### stein算法

