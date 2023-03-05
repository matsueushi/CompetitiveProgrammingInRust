# %%
import seaborn as sns
import numpy as np
import pandas as pd


# %%
def read_file(path):
    print(path)
    with open(path, "r") as f:
        data = f.readline()
        n, w, k, c = map(int, data.split(" "))
        # print(n, w, k, c)
        ar = []
        for _ in range(n):
            data = f.readline()
            ar.append(list(map(int, data.split(" "))))
    
    return ar

# %%
ar = []
ran = range(100)

for r in ran:
    path = f"./tools/in/{r:04}.txt"
    ar.append(read_file(path))

ar = np.array(ar)

# %%
# ヒートマップ
sns.heatmap(ar[2])

# %%
# 硬さのヒストグラム
sns.histplot(ar.flatten())

# %%
# 硬さの順位点
q = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
np.quantile(ar, q)

# [markdown]
# array([  12.,   24.,   56.,  121.,  236.,  434.,  788., 1454., 2608.])
# -> 500あれば破壊できそう

# %%
# テスト結果のコストのヒストグラム
def read_score(path):
    with open(path, "r") as f:
        data = f.read()
        cost = np.array(list(map(int, data.split())))

    return cost

cost = read_score("./tools/score_best.txt")
new_cost = read_score("./tools/score.txt")
sns.distplot(cost)
sns.distplot(new_cost)


# %%
sns.lineplot(cost)
sns.lineplot(new_cost)


# %%
pd.DataFrame(cost).describe()
pd.DataFrame(new_cost).describe()

# %%
# 結果を保存しておく
# %%
