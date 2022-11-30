# no externals for once

ANCHOR = 20201227
INITIAL_SUBJECT = 7

TEST_KEY_A = 5764801
TEST_KEY_B = 17807724

KEY_A = 7573546
KEY_B = 17786549


def loop_multiply_standard(loops: int, subject_num: int):
    res = 1
    for _ in range(loops):
        res = (res * subject_num) % ANCHOR
    return res


def memoized_loop() -> int:
    memo = {}

    def loop_multiply(loops: int, subject_num: int):
        if (loops, subject_num) not in memo:
            res = 1
            if loops != 0:
                res = (loop_multiply(loops - 1, subject_num) * subject_num) % ANCHOR
            memo[(loops, subject_num)] = res
        return memo[(loops, subject_num)]

    return loop_multiply


if __name__ == "__main__":
    loop_multiply = memoized_loop()
    loop_size_a, loop_size_b = 0, 0
    iteration = 1
    while loop_size_b == 0 or loop_size_a == 0:
        iteration += 1
        current = loop_multiply(iteration, INITIAL_SUBJECT)
        if current == INITIAL_SUBJECT:
            raise Exception(f"ran the ring in without finding keys")
        if current == KEY_A:
            loop_size_a = iteration
        if current == KEY_B:
            loop_size_b = iteration

    loop_size_a, loop_size_b = 10985209, 925199
    print(f"loop size a is {loop_size_a}\nloop size b is {loop_size_b}")

    encryption_key_a = loop_multiply_standard(loop_size_a, KEY_B)
    encryption_key_b = loop_multiply_standard(loop_size_b, KEY_A)
    if encryption_key_a != encryption_key_b:
        raise Exception(f"did not find same key: {encryption_key_a} != {encryption_key_b}")
    print(f"encryption key is {encryption_key_b}")
