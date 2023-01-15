import matplotlib.pyplot as plt

if __name__ == "__main__":
    # データの呼び込み
    n = int(input())
    xs = []
    ys = []
    for i in range(n):
        x, y = map(int, input().split())
        xs.append(x)
        ys.append(y)

    # 可視化する
    plt.scatter(xs, ys)
    for i in range(n):
        plt.text(xs[i] + 0.05, ys[i] + 0.05, i + 1)

    plt.savefig("src/bin/a46_trace.png")
