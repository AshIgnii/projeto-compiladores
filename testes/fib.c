int gcd(int a, int b) {
    int temp;
    while (b != 0) {
        temp = b;
        b = a - (a / b) * b;
        a = temp;
    }
    return a;
}

float sqrt(float x) {
    float guess;
    int i;
    guess = x / 2;
    i = 0;
    while (i < 20) {
        guess = (guess + x / guess) / 2;
        i = i + 1;
    }
    return guess;
}

int main() {
    int a;
    int b;
    int result;
    float root;
    a = 48;
    b = 18;
    result = gcd(a, b);
    print(result);
    root = sqrt(result);
    print(root);
    if (result > 1) {
        if (result >= 6)
            print(result * 2);
        else
            print(result + 1);
    }
    else
        print(0);
    if (a <= 100)
        if (b == 18)
            print(a - b);
    return 0;
}