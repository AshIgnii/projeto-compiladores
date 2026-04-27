int abs(int x) {
    int result;
    if (x < 0)
        result = 0 - x;
    else
        result = x;
    return result;
}

int mod(int a, int b) {
    int result;
    result = a - (a / b) * b;
    if (result < 0)
        result = result + b;
    return result;
}

int gcd(int a, int b) {
    int temp;
    a = abs(a);
    b = abs(b);
    while (b > 0) {
        temp = b;
        b = mod(a, b);
        a = temp;
    }
    return a;
}

int lcm(int a, int b) {
    int g;
    g = gcd(a, b);
    return (a / g) * b;
}

int power(int base, int exp) {
    int result;
    result = 1;
    while (exp > 0) {
        result = result * base;
        exp = exp - 1;
    }
    return result;
}

int isPrime(int n) {
    int i;
    int result;
    result = 1;
    if (n <= 1)
        result = 0;
    else {
        i = 2;
        while (i * i <= n) {
            if (mod(n, i) == 0)
                result = 0;
            i = i + 1;
        }
    }
    return result;
}

int countPrimes(int limit) {
    int count;
    int i;
    count = 0;
    i = 2;
    while (i <= limit) {
        if (isPrime(i) == 1)
            count = count + 1;
        i = i + 1;
    }
    return count;
}

int main() {
    int r;
    int i;
    r = gcd(48, 18);
    print(r);
    r = gcd(0 - 36, 24);
    print(r);
    r = lcm(12, 8);
    print(r);
    r = lcm(7, 13);
    print(r);
    r = power(2, 10);
    print(r);
    r = power(3, 5);
    print(r);
    r = abs(0 - 42);
    print(r);
    r = mod(17, 5);
    print(r);
    r = mod(100, 7);
    print(r);
    i = 2;
    while (i <= 30) {
        if (isPrime(i) == 1)
            print(i);
        i = i + 1;
    }
    r = countPrimes(100);
    print(r);
    return 0;
}