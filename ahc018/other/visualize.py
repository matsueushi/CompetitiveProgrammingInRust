#%%
import seaborn as sns
import numpy as np

# %%
with open("./tools/in/0000.txt", "r") as f:
    data = f.readline()
    n, w, k, c = map(int, data.split(" "))
    print(n, w, k, c)
    ar = []
    for _ in range(n):
        data = f.readline()
        ar.append(list(map(int, data.split(" "))))

# %%
ar = np.array(ar)

# %%
# ヒートマップ
sns.heatmap(ar)

# %%
# 硬さのヒストグラム
sns.histplot(ar.flatten())

# %%
# 硬さの順位点
q = [0.05, 0.1, 0.25, 0.50, 0.90, 0.95]
np.quantile(ar, q)

# [markdown]
# -> 500あれば破壊できそう

# %%
