import matplotlib.pyplot as plt

if __name__ == "__main__":
    # cat src/bin/a46_input.txt src/bin/a46_output.txt | python3 src/bin/a46_plotter.py
    # 入力データの読み込み
    n = int(input())
    xs = []
    ys = []
    for i in range(n):
        x, y = map(int, input().split())
        xs.append(x)
        ys.append(y)

    orders = []
    for i in range(n + 1):
        x = int(input())
        orders.append(x)

    # 可視化する
    plt.scatter(xs, ys, color="b")
    plt.axis('square')
    for i in range(n):
        plt.text(xs[i] + 0.05, ys[i] + 0.05, i + 1)

    for i in range(n):
        j = orders[i] - 1
        j_next = orders[i + 1] - 1
        dx = xs[j_next] - xs[j]
        dy = ys[j_next] - ys[j]
        plt.arrow(xs[j], ys[j], 0.8 * dx, 0.8 * dy, color="g", width=0.02)

    plt.savefig("src/bin/a46_trace.png")
