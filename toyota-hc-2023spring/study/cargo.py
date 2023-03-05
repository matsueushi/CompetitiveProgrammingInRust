# %%
import polars as pl
import seaborn as sns


# %%
q = (
    pl.scan_csv("./testcases/d1cb95b8.csv")
        .with_columns(
        (pl.col("w") * pl.col("h")*pl.col("d")).alias("vol")
    )
)

df = q.collect()

# %%
sns.histplot(df["w"]).set(title='w')

# %%
sns.histplot(df["h"]).set(title='w')

# %%
sns.histplot(df["d"]).set(title='d')

# %%
# %%
sns.histplot(df["vol"]).set(title='vol')

# %%
