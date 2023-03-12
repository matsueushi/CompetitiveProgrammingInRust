# %%
import csv
import numpy as np
import plotly.graph_objects as go


# %%
with open("tools/out/height_11.csv") as f:
    reader = csv.reader(f, delimiter=",")
    data = []
    for row in reader:
        data.append(list(map(int, row)))

z = np.array(data)
x = list(range(z.shape[0]))
y = list(range(z.shape[1]))

fig = go.Figure(data=[go.Surface(z=z)])
fig

# %%
