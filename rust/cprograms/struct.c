
struct bit_field {
    int data1:8;
};

int main() {
    struct bit_field bf = {
        .data1 = 0x1234,
    };

    return 0;
}
