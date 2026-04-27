float absFloat(float x) {
    float r;
    if (x >= 0)
        r = x;
    else
        r = 0 - x;
    return r;
}

int getMaxIter() {
    return 30;
}

float sqrt(float n) {
    float guess;
    int i;
    int maxIter;
    maxIter = getMaxIter();
    guess = n / 2;
    if (n < 1)
        guess = 1;
    i = 0;
    while (i < maxIter) {
        guess = (guess + n / guess) / 2;
        i = i + 1;
    }
    return guess;
}

float cubeRoot(float n) {
    float guess;
    int i;
    int neg;
    neg = 0;
    if (n < 0) {
        n = 0 - n;
        neg = 1;
    }
    guess = n / 3;
    if (guess < 1)
        guess = 1;
    i = 0;
    while (i < 30) {
        guess = (2 * guess + n / (guess * guess)) / 3;
        i = i + 1;
    }
    if (neg == 1)
        guess = 0 - guess;
    return guess;
}

float distance(float x1, float y1, float x2, float y2) {
    float dx;
    float dy;
    dx = x2 - x1;
    dy = y2 - y1;
    return sqrt(dx * dx + dy * dy);
}

float piLeibniz(int terms) {
    float result;
    float sign;
    float term;
    int i;
    result = 0;
    sign = 1;
    i = 0;
    while (i < terms) {
        term = sign / (2 * i + 1);
        result = result + term;
        sign = 0 - sign;
        i = i + 1;
    }
    return result * 4;
}

float areaCircle(float radius) {
    float pi;
    pi = piLeibniz(10000);
    return pi * radius * radius;
}

float hypotenuse(float a, float b) {
    return sqrt(a * a + b * b);
}

int main() {
    float r;
    float d;
    float area;
    int iters;
    iters = getMaxIter();
    print(iters);
    r = sqrt(144);
    print(r);
    r = sqrt(2);
    print(r);
    r = cubeRoot(27);
    print(r);
    r = cubeRoot(0 - 8);
    print(r);
    d = distance(0, 0, 3, 4);
    print(d);
    d = distance(1, 2, 4, 6);
    print(d);
    r = piLeibniz(10000);
    print(r);
    area = areaCircle(5);
    print(area);
    r = hypotenuse(3, 4);
    print(r);
    return 0;
}