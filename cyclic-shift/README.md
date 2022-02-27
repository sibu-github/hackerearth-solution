# Cyclic shift

A large binary number is represented by a string `A` of size `N` and comprises of 1s and 0s. You must perform a cyclic shift on this string. The cyclic shift operation is defined as follows:

- If the string `A` is `[A0, A1, A2, ... An-1]`, then after performing one cyclic shift, the string becomes `[A1, A2, ... An-1, A0]`. You performed the shift infinite number of times and each time you recorded the value of the binary number represented by the string. The maximum binary number formed after performing (possibly 0) the operation is `B`. Your task is to determine the number of cyclic shifts that can be performed such that the value represented by the string `A` will be equal to `B` for the `Kth` time.

## Input format

- First line: A single integer `T` denoting the number of test cases.
- For each test case:
  -- First line: Two space-separated integers `N` and `K`
  -- Second line: `A` denoting the string

## Output format

For each test case, print a single line containing one integer that represents the number of cyclic shift operations performed such that the value represented by string `A` is equal to `B` for the `Kth` time.

## Constraints

```
1 <= T <= 10^3
1 <= N <= 10^5
1 <= K <= 10^9
```

## Sample Input

```
2
5 2
10101
6 2
010101
```

## Sample Output

```
9
3
```

## Explanation

For the 1st test case, the value of `B` is `11010`. After performing 4 cyclic shifts the value represented by array `A` becomes equal to `B` for the first time. After performing additional 5 cyclic shifts the value represented by array `A` becomes `B` for the second time. Hence, the answer is `4 + 5 = 9`.

For the 2nd test case, the value of `B` is `101010`. After performing cyclic 1 shifts the value represented by array `A` becomes equal to `B` for the first time. After performing additional 2 cyclic shifts the value represented by array `A` becomes `B` for the second time. Hence, the answer is `1 + 2 = 3`.
